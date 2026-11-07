#!/usr/bin/env python3
import time
import math
import threading
import json
from pathlib import Path
from tf_transformations import euler_from_quaternion
import rclpy
from rclpy.action import ActionServer, GoalResponse, CancelResponse
from rclpy.node import Node
from rclpy.callback_groups import ReentrantCallbackGroup, MutuallyExclusiveCallbackGroup
from rclpy.executors import MultiThreadedExecutor

# the action
from waypoint_navigation.action import NavToWaypoint

# pico control specific libraries
from swift_msgs.msg import SwiftMsgs
from std_msgs.msg import Header
from whycode_interfaces.msg import MarkerArray
from error_msg.msg import Error
from controller_msg.msg import PIDTune
from nav_msgs.msg import Odometry


# Success condition from the task: +-0.4 whycode units on EVERY axis, held for 3 s
HOLD_TOL = 0.4
HOLD_TIME = 3.0

def wrap_deg(a):
    return (a + 180.0) % 360.0 - 180.0

class WayPointServer(Node):
    def __init__(self):
        super().__init__('waypoint_server')

        # whycode callback + PID timer share ONE mutually-exclusive group, so they never run at the same time
        # (same behaviour as the single-threaded Task 2A node). The action callback blocks while it waits for the
        # hold, so it gets its own group.
        self.pid_callback_group = MutuallyExclusiveCallbackGroup()
        self.action_callback_group = ReentrantCallbackGroup()
        self.odometry_callback_group = ReentrantCallbackGroup()

        # Current position [x, y, z] - updated in whycode_callback
        self.current_state = [0.0, 0.0, 0.0]
        self.have_marker = False   # PID stays idle until the first valid marker
        self.last_marker_wall = 0.0
        self.marker_loss_count = 0
        self.ground_z = None
        self.setpoint_lock = threading.Lock()
        from ament_index_python.packages import get_package_share_directory
        grid_path = Path(get_package_share_directory('swift_pico')) / 'scripts' / 'grid_whycode_coordinates.json'
        with grid_path.open() as f:
            self.home_xy = json.load(f)['F6']
        self.new_data = False      # True when a fresh marker arrived since the last PID run
        self.use_tuner = False     # True = GUI tuner, False = gains from code
        self.previous_state = [0.0, 0.0, 0.0]
        # Setpoint [x, y, z] - filled from the action goals
        self.desired_state = [0.0, 0.0, 0.0]

        self.prev_axis_error = [0.0, 0.0, 0.0]

        self.prev_time = None
        self.d_filter = 0.4        # 0..1, lower = smoother
        self.max_throttle_step = 25  # max change in throttle command per cycle

        self.state_time = None
        self.dtime = 0.0           # latest whycode timestamp in seconds (used for the hold timer)

        self.yaw = 0.0             # current yaw in degrees (from whycode)
        self.desired_yaw = -0.5    # target yaw in degrees

        self.cmd = SwiftMsgs()
        self.cmd.rc_roll = 1500
        self.cmd.rc_pitch = 1500
        self.cmd.rc_yaw = 1500
        self.cmd.rc_throttle = 1500
        self.cmd.rc_aux4 = 2000

        # Gains [roll, pitch, throttle]
        self.kp = [20.0, 20.0, 20.0]
        self.ki = [0, 0, 0.009]
        self.kd = [55.0, 55.0, 81.0]

        #  PID state 
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

        #  yaw: P controller (new in 2B) 
        self.kp_yaw = 2.0          # rc counts per degree of yaw error
        self.sign_yaw = 1.0        # flip to -1.0 if the yaw error grows instead of shrinking
        self.max_yaw_out = 100.0   # max rc offset from 1500

        #  waypoint handling 
        self.pending_desired = None    # new setpoint handed over from the action callback
        self.control_enabled = False   # no position control until the first goal
        self.busy = False

        # bookkeeping for the hold check
        self.time_inside_sphere = 0
        self.max_time_inside_sphere = 0
        self.point_in_sphere_start_time = None
        self.duration = 0

        self.sample_time = 0.033  # seconds
        self.log_counter = 0

        # Publishers
        self.command_pub = self.create_publisher(SwiftMsgs, '/drone_command', 10)
        self.pos_error_pub = self.create_publisher(Error, '/pos_error', 10)
        self.heartbeat_pub = self.create_publisher(Header, '/controller_heartbeat', 10)

        # Subscribers
        self.create_subscription(MarkerArray, '/whycode_node/markers', self.whycode_callback, 1,
                                 callback_group=self.pid_callback_group)
        if self.use_tuner:
            self.create_subscription(PIDTune, '/throttle_pid', self.altitude_set_pid, 1)
            self.create_subscription(PIDTune, '/pitch_pid', self.pitch_set_pid, 1)
            self.create_subscription(PIDTune, '/roll_pid', self.roll_set_pid, 1)
        self.create_subscription(Odometry, '/rotors/odometry', self.odometry_callback, 10,
                                 callback_group=self.odometry_callback_group)

        # Action server
        self._action_server = ActionServer(
            self,
            NavToWaypoint,
            'waypoint_navigation',
            execute_callback=self.execute_callback,
            goal_callback=self.goal_callback,
            cancel_callback=self.cancel_callback,
            callback_group=self.action_callback_group)

        self.publish_heartbeat()
        self.create_timer(0.1, self.publish_heartbeat)

        time.sleep(0.5)  # let the heartbeat reach the rosbag before arming
        self.arm()

        self.create_timer(self.sample_time, self.pid, callback_group=self.pid_callback_group)

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

    def whycode_callback(self, msg):
        if not msg.markers:
            self.marker_loss_count += 1
            self.last_marker_wall = 0.0
            return
        marker = msg.markers[0]
        pos = marker.position.position
        self.current_state = [pos.x, pos.y, pos.z]
        self.last_marker_wall = time.monotonic()
        if self.ground_z is None and not self.control_enabled:
            self.ground_z = pos.z
        self.state_time = self.get_clock().now().nanoseconds * 1e-9

        # the yaw controller works in degrees, so convert here
        self.yaw = math.degrees(marker.rotation.z)

        # full-resolution marker time, used for the 3 s hold (stamp.sec alone is whole seconds)
        t = msg.header.stamp.sec + msg.header.stamp.nanosec * 1e-9
        if t == 0.0:
            t = self.state_time
        self.dtime = t

        self.have_marker = True
        self.new_data = True

    def odometry_callback(self, msg):
        orientation_q = msg.pose.pose.orientation
        orientation_list = [orientation_q.x, orientation_q.y, orientation_q.z, orientation_q.w]
        roll, pitch, yaw = euler_from_quaternion(orientation_list)

        self.roll_deg = math.degrees(roll)
        self.pitch_deg = math.degrees(pitch)
        self.yaw_deg = math.degrees(yaw)

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

    def pid(self):
        # A new waypoint is applied HERE (in the control cycle), so the setpoint never changes mid-calculation
        with self.setpoint_lock:
            pending = self.pending_desired
            if pending is not None:
                self.pending_desired = None
                self.desired_state = pending
                self.control_enabled = True

        # No goal yet: keep the armed, neutral command alive and wait
        if not self.control_enabled:
            self.command_pub.publish(self.cmd)
            return

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

        # 3b. Yaw: P controller towards desired_yaw (degrees, wrapped to +-180)
        yaw_err = wrap_deg(self.desired_yaw - self.yaw)
        yaw_out = max(-self.max_yaw_out, min(self.max_yaw_out, self.kp_yaw * yaw_err))
        self.cmd.rc_yaw = int(max(1000, min(2000, 1500 + self.sign_yaw * yaw_out)))

        # 4. Position error message
        self.pos_error.pitch_error = self.error[0]
        self.pos_error.roll_error = self.error[1]
        self.pos_error.throttle_error = self.error[2]
        if hasattr(self.pos_error, 'yaw_error'):
            self.pos_error.yaw_error = yaw_err

        self.command_pub.publish(self.cmd)
        self.pos_error_pub.publish(self.pos_error)

        # Log ~once per second instead of every cycle
        self.log_counter += 1
        if self.log_counter >= int(1.0 / self.sample_time):
            self.log_counter = 0
            self.get_logger().info(
                f"err[x,y,z]={[round(e, 3) for e in self.error]} yaw={self.yaw:.1f} | "
                f"Z: P={self.P_output[2]:.1f} I={self.I_output[2]:.1f} "
                f"D={self.D_output[2]:.1f} -> throttle={self.cmd.rc_throttle} | "
                f"Kp={self.kp} Ki={self.ki} Kd={self.kd}"
            )

    # Action server callbacks
    def goal_callback(self, goal_request):
        if self.busy:
            self.get_logger().warn('Goal rejected: previous goal is still being executed')
            return GoalResponse.REJECT
        self.busy = True
        return GoalResponse.ACCEPT

    def cancel_callback(self, goal_handle):
        return CancelResponse.ACCEPT

    def set_target(self, target):
        with self.setpoint_lock:
            self.pending_desired = list(target)

    def fresh_marker(self):
        return self.last_marker_wall > 0 and time.monotonic() - self.last_marker_wall < 0.35

    def hold_here(self):
        self.set_target(self.current_state.copy())

    def execute_callback(self, goal_handle):
        pose = goal_handle.request.waypoint
        p = pose.position
        target = [p.x, p.y, p.z]
        is_home = (goal_handle.request.hover and abs(p.z - 13.0) < 0.05
                   and math.hypot(p.x - self.home_xy['x'], p.y - self.home_xy['y']) < 0.08)
        holding = goal_handle.request.hover
        hold_seconds = 2.0 if abs(p.z - 14.0) < 0.05 else 1.0
        feedback = NavToWaypoint.Feedback()
        result = NavToWaypoint.Result()
        started = time.monotonic()
        inside_since = None
        seen_loss_count = self.marker_loss_count
        self.set_target(target)
        try:
            while rclpy.ok():
                if goal_handle.is_cancel_requested:
                    self.hold_here()
                    goal_handle.canceled()
                    return result
                now = time.monotonic()
                state = self.current_state.copy()
                feedback.current_waypoint.pose.position.x = state[0]
                feedback.current_waypoint.pose.position.y = state[1]
                feedback.current_waypoint.pose.position.z = state[2]
                feedback.current_waypoint.header.stamp = self.get_clock().now().to_msg()
                if self.fresh_marker():
                    goal_handle.publish_feedback(feedback)
                if self.marker_loss_count != seen_loss_count:
                    inside_since = None
                    seen_loss_count = self.marker_loss_count
                inside = self.fresh_marker() and math.dist(state, target) <= HOLD_TOL
                if inside:
                    if inside_since is None:
                        inside_since = now
                    if not holding or now - inside_since >= hold_seconds:
                        break
                else:
                    inside_since = None
                time.sleep(0.02)
            else:
                goal_handle.abort()
                return result
            if is_home:
                # Ground z is measured before the first goal, not hardcoded.
                if self.ground_z is None or self.ground_z <= 14.0:
                    self.get_logger().error('Ground marker height unavailable; refusing automatic landing')
                    goal_handle.abort()
                    return result
                z = target[2]
                last = time.monotonic()
                deadline = last + 25.0
                while rclpy.ok() and time.monotonic() < deadline:
                    if goal_handle.is_cancel_requested:
                        self.hold_here()
                        goal_handle.canceled()
                        return result
                    now = time.monotonic()
                    if not self.fresh_marker():
                        self.hold_here()
                        goal_handle.abort()
                        return result
                    z = min(self.ground_z + 0.10, z + 0.25 * (now - last))
                    last = now
                    self.set_target([target[0], target[1], z])
                    if self.current_state[2] >= self.ground_z - 0.12:
                        self.control_enabled = False
                        self.disarm()
                        goal_handle.succeed()
                        result.hov_time = int(now - started)
                        return result
                    time.sleep(0.05)
                self.hold_here()
                goal_handle.abort()
                return result
            goal_handle.succeed()
            result.hov_time = int(time.monotonic() - started)
            return result
        finally:
            self.busy = False


def main(args=None):
    rclpy.init(args=args)
    waypoint_server = WayPointServer()
    executor = MultiThreadedExecutor()
    executor.add_node(waypoint_server)

    try:
        executor.spin()
    except KeyboardInterrupt:
        waypoint_server.get_logger().info('KeyboardInterrupt, shutting down.\n')
    finally:
        executor.shutdown()
        executor.remove_node(waypoint_server)
        if rclpy.ok():
            waypoint_server.disarm()    
        waypoint_server.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()