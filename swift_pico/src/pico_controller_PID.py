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

        # ---------------- State ----------------
        self.current_state = [0.0, 0.0, 0.0]      # [x, y, z] raw WhyCode units
        self.desired_state = [1.5, -1.5, 13.0]    # target [x, y, z]
        self.have_marker = False                  # True after first marker message

        # ---------------- Command message ----------------
        self.cmd = SwiftMsgs()
        self.cmd.rc_roll = 1500
        self.cmd.rc_pitch = 1500
        self.cmd.rc_yaw = 1500
        self.cmd.rc_throttle = 1500

        self.pos_error = Error()

        self.USE_GUI = False

    
        self.Kp = [14,14,28]
        self.Ki = [0.00112,0.00112,0.06]
        self.Kd = [800,800,1800]


        self.kp_scale = [1.0, 1.0, 1.0]
        self.ki_scale = [0.008, 0.008, 0.008]
        self.kd_scale = [10.0, 10.0, 10.0]

        
        self.sign = [-1, -1, 1]

    
        self.base_values = [1500, 1500, 1500]

        self.prev_error = [0.0, 0.0, 0.0]
        self.error_sum = [0.0, 0.0, 0.0]
        self.first_run = True

        # Output limits [pitch, roll, throttle]
        self.min_values = [1000, 1000, 1000]
        self.max_values = [2000, 2000, 2000]

        # Anti-windup
        self.i_zone = 1.0          # integrate only when |error| < this
        self.max_i_term = 100.0    # cap on the I contribution (command units)

        self.sample_time = 0.033   # control loop period (s)

        # ---------------- Publishers ----------------
        self.command_pub = self.create_publisher(SwiftMsgs, '/drone_command', 10)
        self.pos_error_pub = self.create_publisher(Error, '/pos_error', 10)
        self.heartbeat_pub = self.create_publisher(Header, '/controller_heartbeat', 10)

        # ---------------- Subscribers ----------------
        self.create_subscription(MarkerArray, '/whycode_node/markers', self.whycode_callback, 1)
        self.create_subscription(PIDTune, '/pitch_pid', self.pitch_set_pid, 1)
        self.create_subscription(PIDTune, '/roll_pid', self.roll_set_pid, 1)
        self.create_subscription(PIDTune, '/throttle_pid', self.altitude_set_pid, 1)

        

        self.publish_heartbeat()
        self.create_timer(1.0, self.publish_heartbeat)
        time.sleep(0.5)
        self.arm()
        self.create_timer(self.sample_time, self.pid)

    # ---------------- Heartbeat / arming ----------------
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

    # ---------------- Callbacks ----------------
    def whycode_callback(self, msg):
        if not msg.markers:        # marker not visible: keep last known position
            return
        p = msg.markers[0].position.position
        self.current_state = [p.x, p.y, p.z]
        self.have_marker = True

    def set_gains(self, axis, msg):
        '''Shared by the three gain callbacks. axis: 0=pitch, 1=roll, 2=throttle.'''
        if not self.USE_GUI:
            return
        self.Kp[axis] = msg.kp * self.kp_scale[axis]
        self.Ki[axis] = msg.ki * self.ki_scale[axis]
        self.Kd[axis] = msg.kd * self.kd_scale[axis]
        self.get_logger().info(
            f'axis {axis}: Kp={self.Kp[axis]:.3f} Ki={self.Ki[axis]:.4f} Kd={self.Kd[axis]:.3f}')

    def pitch_set_pid(self, msg):
        self.set_gains(0, msg)

    def roll_set_pid(self, msg):
        self.set_gains(1, msg)

    def altitude_set_pid(self, msg):
        self.set_gains(2, msg)

    # ---------------- PID ----------------
    def clamp(self, value, axis):
        return int(max(self.min_values[axis], min(self.max_values[axis], value)))

    def pid(self):
        if not self.have_marker:
            return

        # Error = current - desired
        errors = [c - d for c, d in zip(self.current_state, self.desired_state)]

        if self.first_run:         # avoid a fake derivative spike on cycle 1
            self.prev_error = errors[:]
            self.first_run = False

        outputs = []
        for i in range(3):
            err = errors[i]

            if abs(err) < self.i_zone:
                self.error_sum[i] += err
            else:
                self.error_sum[i] = 0.0

            p_term = self.Kp[i] * err
            d_term = self.Kd[i] * (err - self.prev_error[i])
            i_term = self.Ki[i] * self.error_sum[i]
            i_term = max(-self.max_i_term, min(self.max_i_term, i_term))

            outputs.append(p_term + i_term + d_term)
            self.prev_error[i] = err

        self.cmd.rc_pitch = self.clamp(self.base_values[0] + self.sign[0] * outputs[0], 0)
        self.cmd.rc_roll = self.clamp(self.base_values[1] + self.sign[1] * outputs[1], 1)
        self.cmd.rc_throttle = self.clamp(self.base_values[2] + self.sign[2] * outputs[2], 2)

        self.pos_error.pitch_error = errors[0]
        self.pos_error.roll_error = errors[1]
        self.pos_error.throttle_error = errors[2]

        self.command_pub.publish(self.cmd)
        self.pos_error_pub.publish(self.pos_error)

        self.get_logger().info(
            f'pos={[round(v, 2) for v in self.current_state]} '
            f'err={[round(e, 2) for e in errors]} '
            f'cmd=[{self.cmd.rc_pitch}, {self.cmd.rc_roll}, {self.cmd.rc_throttle}]',
            throttle_duration_sec=1.0)


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