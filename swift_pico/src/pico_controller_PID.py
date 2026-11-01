#!/usr/bin/env python3
from swift_msgs.msg import SwiftMsgs
from whycode_interfaces.msg import MarkerArray
from controller_msg.msg import PIDTune
from error_msg.msg import Error
from std_msgs.msg import Header
import time
import rclpy
from rclpy.node import Node


class Swift_Pico(Node):
    def __init__(self):
        super().__init__('pico_controller')

        # Current position [x, y, z] - updated in whycode_callback
        self.current_state = [0.0, 0.0, 0.0]
        self.have_marker = False   # PID stays idle until the first valid marker
        self.new_data = False      # True when a fresh marker arrived since the last PID run
        self.use_tuner = False      # True = GUI tuner, False = gains from code
        self.previous_state = [0.0, 0.0, 0.0]
        # Setpoint [x, y, z]
        self.desired_state = [1.5, -1.5, 13.0]
        
        self.prev_axis_error = [0.0,0.0,0.0]
        
        self.prev_time = None
        self.d_filter = 0.4        # 0..1, lower = smoother
        self.max_throttle_step = 25  # max change in throttle command per cycle

        self.state_time = None
        
        self.cmd = SwiftMsgs()
        self.cmd.rc_roll = 1500
        self.cmd.rc_pitch = 1500
        self.cmd.rc_yaw = 1500
        self.cmd.rc_throttle = 1500
        self.cmd.rc_aux4 = 2000

        # Gains [roll, pitch, throttle] - set live from the GUI
        self.kp = [17.0, 17.0, 20.0]
        self.ki = [0.0, 0.0, 0.01]
        self.kd = [40.0, 40.0, 45]

        # ---------------- PID state ----------------
        self.error = [0.0, 0.0, 0.0]
        self.previous_error = [0.0, 0.0, 0.0]
        self.error_sum = [0.0, 0.0, 0.0]
        self.last_derivative = [0.0, 0.0, 0.0]
        self.first_run = True

        self.pos_error = Error()

        # Output sign per axis [roll, pitch, throttle].
        # VERIFY BY EXPERIMENT (error = current - desired):
        # throttle should INCREASE when the drone is below the setpoint.
        self.sign = [-1, -1, 1]

        # Baseline command per axis [roll, pitch, throttle]. Raise throttle baseline to ~hover value.
        self.base = [1500, 1500, 1517]

        # Limits
        self.max_values = [2000, 2000, 2000]
        self.min_values = [1000, 1000, 1000]
        self.i_zone = 1            # only integrate when |error| < i_zone
        self.i_limit = [50.0, 50.0, 50.0]  # max |I contribution| in command counts

        self.sample_time = 0.033  # seconds
        self.log_counter = 0

        # Publishers
        self.command_pub = self.create_publisher(SwiftMsgs, '/drone_command', 10)
        self.pos_error_pub = self.create_publisher(Error, '/pos_error', 10)
        self.heartbeat_pub = self.create_publisher(Header, '/controller_heartbeat', 10)

        # Subscribers
        self.create_subscription(MarkerArray, '/whycode_node/markers', self.whycode_callback, 1)
        if self.use_tuner:
            self.create_subscription(PIDTune, '/throttle_pid', self.altitude_set_pid, 1)
            self.create_subscription(PIDTune, '/pitch_pid', self.pitch_set_pid, 1)
            self.create_subscription(PIDTune, '/roll_pid', self.roll_set_pid, 1)

        self.publish_heartbeat()
        self.create_timer(0.1, self.publish_heartbeat)

        time.sleep(0.5)  # let the heartbeat reach the rosbag before arming
        self.arm()

        self.create_timer(self.sample_time, self.pid)

    # ------------------------------------------------------------------
    def publish_heartbeat(self):
        hb = Header()
        hb.stamp = self.get_clock().now().to_msg()
        hb.frame_id = 'pico_controller_active'
        self.heartbeat_pub.publish(hb)

    def disarm(self):
        self.cmd.rc_roll = 1000
        self.cmd.rc_yaw = 1000
        self.cmd.rc_pitch = 1000
        self.cmd.rc_throttle = 1000
        self.cmd.rc_aux4 = 1000
        self.command_pub.publish(self.cmd)

    def arm(self):
        self.disarm()
        self.cmd.rc_roll = 1500
        self.cmd.rc_yaw = 1500
        self.cmd.rc_pitch = 1500
        self.cmd.rc_throttle = 1500
        self.cmd.rc_aux4 = 2000
        self.command_pub.publish(self.cmd)

    # ------------------------------------------------------------------
    def whycode_callback(self, msg):
        if not msg.markers:          # no marker visible -> keep last state
            return
        # If several markers can be visible, select yours here by id, e.g.:
        # marker = next((m for m in msg.markers if m.id == MY_ID), None)
        pos = msg.markers[0].position.position
        self.current_state = [pos.x, pos.y, pos.z]
        self.state_time = self.get_clock().now().nanoseconds * 1e-9
        self.have_marker = True
        self.new_data = True

    # ------------------------------------------------------------------
    def altitude_set_pid(self, alt):
        self.kp[2] = alt.kp * 1.0
        self.ki[2] = alt.ki * 1.0
        self.kd[2] = alt.kd * 1.0

    def pitch_set_pid(self, pitch):
        self.kp[1] = pitch.kp * 1.0
        self.ki[1] = pitch.ki * 1.0
        self.kd[1] = pitch.kd * 1.0

    def roll_set_pid(self, roll):
        self.kp[0] = roll.kp * 1.0
        self.ki[0] = roll.ki * 1.0
        self.kd[0] = roll.kd * 1.0

    # ------------------------------------------------------------------
    def pid(self):
        # Do nothing until we actually know where the drone is
        if not self.have_marker:
            return

        # 1. Error in [x, y, z]
        self.error = [self.current_state[i] - self.desired_state[i] for i in range(3)]

        # Avoid a derivative spike on the first real update
        if self.first_run:
            self.previous_state = self.current_state.copy()
            self.first_run = False

        if self.new_data:
            t = self.state_time
            if self.prev_time is not None and t - self.prev_time > 1e-3:
                dt = t - self.prev_time
                raw = [(self.current_state[i] - self.previous_state[i]) / dt for i in range(3)]
                a = self.d_filter
                self.last_derivative = [a * raw[i] + (1 - a) * self.last_derivative[i] for i in range(3)]
            self.previous_state = self.current_state.copy()
            self.prev_time = t
            self.new_data = False
            fresh = True
        else:
            fresh = False

        # Map error axes to control axes: roll<-y, pitch<-x, throttle<-z
        axis_err = [self.error[1], self.error[0], self.error[2]]
        axis_der = [self.last_derivative[1], self.last_derivative[0], self.last_derivative[2]]

        outputs = [0.0, 0.0, 0.0]
        self.P_output = [0.0, 0.0, 0.0]
        self.I_output = [0.0, 0.0, 0.0]
        self.D_output = [0.0, 0.0, 0.0]
        for i in range(3):
            # Clear the integral when the error crosses zero (kills windup overshoot)
            if fresh and self.prev_axis_error[i] * axis_err[i] < 0:
                self.error_sum[i] = 0.0
            if fresh:
                self.prev_axis_error[i] = axis_err[i]

            # Integrate only inside the I-zone
            if fresh and abs(axis_err[i]) < self.i_zone:
                self.error_sum[i] += axis_err[i]

            # I output, clamped in command counts (back-calculate the sum)
            i_out = self.ki[i] * self.error_sum[i]
            if self.ki[i] != 0.0 and abs(i_out) > self.i_limit[i]:
                i_out = self.i_limit[i] if i_out > 0 else -self.i_limit[i]
                self.error_sum[i] = i_out / self.ki[i]

            self.P_output[i] = self.kp[i] * axis_err[i]
            self.I_output[i] = i_out
            self.D_output[i] = self.kd[i] * axis_der[i]
            outputs[i] = self.P_output[i] + self.I_output[i] + self.D_output[i]
        self.out_roll, self.out_pitch, self.out_throttle = outputs

        # 3. Command = baseline + signed output, clamped to limits
        cmds = []
        for i in range(3):
            c = int(round(self.base[i] + self.sign[i] * outputs[i]))
            cmds.append(max(self.min_values[i], min(self.max_values[i], c)))
        prev_t = self.cmd.rc_throttle
        cmds[2] = max(prev_t - self.max_throttle_step, min(prev_t + self.max_throttle_step, cmds[2]))
        self.cmd.rc_roll, self.cmd.rc_pitch, self.cmd.rc_throttle = cmds

        # 4. Position error message
        self.pos_error.pitch_error = self.error[0]
        self.pos_error.roll_error = self.error[1]
        self.pos_error.throttle_error = self.error[2]

        self.command_pub.publish(self.cmd)
        self.pos_error_pub.publish(self.pos_error)

        # Log ~once per second instead of every cycle
        self.log_counter += 1
        if self.log_counter >= int(1.0 / self.sample_time):
            self.log_counter = 0
            self.get_logger().info(
                f"err[x,y,z]={[round(e, 3) for e in self.error]} | "
                f"Z: P={self.P_output[2]:.1f} I={self.I_output[2]:.1f} "
                f"D={self.D_output[2]:.1f} -> throttle={self.cmd.rc_throttle} | "
                f"Kp={self.kp} Ki={self.ki} Kd={self.kd}"
            )


def main(args=None):
    rclpy.init(args=args)
    swift_pico = Swift_Pico()

    try:
        rclpy.spin(swift_pico)
    except KeyboardInterrupt:
        swift_pico.get_logger().info('KeyboardInterrupt, shutting down.\n')
    finally:
        swift_pico.disarm()
        swift_pico.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()
