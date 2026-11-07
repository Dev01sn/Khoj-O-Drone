#!/usr/bin/env python3
import time
import rclpy
from rclpy.action import ActionClient
from rclpy.node import Node
from action_msgs.msg import GoalStatus
from geometry_msgs.msg import Pose
from waypoint_navigation.action import NavToWaypoint
from waypoint_navigation.srv import GetWaypoints, ReplanStable


class WayPointClient(Node):
    def __init__(self):
        super().__init__('waypoint_client')
        self.action = ActionClient(self, NavToWaypoint, 'waypoint_navigation')
        self.critical_service = self.create_client(GetWaypoints, 'waypoints')
        self.stable_service = self.create_client(ReplanStable, 'replan_stable')
        self.phase = 'waiting'
        self.queue = []
        self.index = 0
        self.goal_handle = None
        self.acceptance_pending = False
        self.last_position = None
        self.feedback_time = 0.0
        self.switch_started = False
        self.clock_start = None
        self.declare_parameter('critical_window', 55.0)
        self.create_timer(0.05, self.tick)
        self.get_logger().info('Waiting for both waypoint services and action server')

    def tick(self):
        if self.phase == 'waiting':
            if not (self.critical_service.service_is_ready() and self.stable_service.service_is_ready() and self.action.server_is_ready()):
                return
            self.phase = 'requesting_critical'
            req = GetWaypoints.Request()
            req.get_waypoints = True
            self.critical_service.call_async(req).add_done_callback(self.critical_received)
        elif self.phase == 'critical' and self.clock_start is not None:
            if time.monotonic() - self.clock_start >= float(self.get_parameter('critical_window').value):
                self.switch()

    def unpack(self, response):
        poses, labels = response.waypoints.poses, response.marker_ids
        if len(poses) != len(labels) or not labels:
            raise ValueError('Empty route or mismatched route labels')
        goals = []
        for pose, label in zip(poses, labels):
            if label == 'TRANSIT':
                goals.append((pose, False, label))
            elif label.startswith('SURVIVOR:'):
                visit, save = Pose(), Pose()
                visit.position.x, visit.position.y, visit.position.z = pose.position.x, pose.position.y, 13.0
                save.position.x, save.position.y, save.position.z = pose.position.x, pose.position.y, 14.0
                visit.orientation.w = save.orientation.w = 1.0
                goals.extend([(visit, True, label + ':visit'), (save, True, label + ':save')])
            elif label == 'HOME':
                home = Pose()
                home.position.x, home.position.y, home.position.z = pose.position.x, pose.position.y, 13.0
                home.orientation.w = 1.0
                goals.append((home, True, 'HOME'))
            else:
                raise ValueError('Unknown route label: ' + label)
        return goals

    def critical_received(self, future):
        try:
            self.queue = self.unpack(future.result())
            self.index = 0
            self.phase = 'critical'
            # Conservative start immediately before dispatching the first goal, never on client launch.
            self.clock_start = time.monotonic()
            self.send_next()
        except Exception as e:
            self.fail('Critical route: %s' % e)

    def send_next(self):
        if self.index >= len(self.queue):
            if self.phase == 'critical':
                self.switch()
            else:
                self.phase = 'done'
                self.get_logger().info('Mission goals finished')
            return
        pose, hover, label = self.queue[self.index]
        goal = NavToWaypoint.Goal()
        goal.waypoint, goal.hover = pose, hover
        self.acceptance_pending = True
        self.get_logger().info('Sending %s' % label)
        future = self.action.send_goal_async(goal, feedback_callback=self.feedback)
        future.add_done_callback(self.accepted)

    def accepted(self, future):
        self.acceptance_pending = False
        try:
            handle = future.result()
            if not handle.accepted:
                self.fail('Action goal was rejected')
                return
            self.goal_handle = handle
            handle.get_result_async().add_done_callback(self.result)
            if self.phase == 'switching':
                self.cancel_active()
        except Exception as e:
            self.fail('Sending action goal: %s' % e)

    def feedback(self, msg):
        pos = msg.feedback.current_waypoint.pose.position
        self.last_position = (pos.x, pos.y, pos.z)
        self.feedback_time = time.monotonic()

    def cancel_active(self):
        if self.goal_handle is not None:
            self.goal_handle.cancel_goal_async().add_done_callback(self.cancel_response)

    def cancel_response(self, future):
        try:
            response = future.result()
            if not response.goals_canceling:
                self.get_logger().warn('Cancel was not accepted; waiting for goal result before replan')
        except Exception as e:
            self.fail('Cancel request failed: %s' % e)

    def result(self, future):
        try:
            response = future.result()
            self.goal_handle = None
            if self.phase == 'switching':
                self.request_stable()
                return
            if response.status != GoalStatus.STATUS_SUCCEEDED:
                self.fail('Goal failed with status %s' % response.status)
                return
            self.index += 1
            # Check deadline even if goal result arrives just before the timer callback.
            if self.phase == 'critical' and time.monotonic() - self.clock_start >= float(self.get_parameter('critical_window').value):
                self.switch()
            else:
                self.send_next()
        except Exception as e:
            self.fail('Action result: %s' % e)

    def switch(self):
        if self.switch_started:
            return
        self.switch_started = True
        self.phase = 'switching'
        self.get_logger().info('Switch: cancel goal and replan from feedback')
        if self.goal_handle is not None:
            self.cancel_active()
        elif not self.acceptance_pending:
            self.request_stable()

    def request_stable(self):
        if self.phase != 'switching':
            return
        if self.last_position is None or time.monotonic() - self.feedback_time > 0.5:
            self.fail('No recent action feedback at Switch; cannot safely replan')
            return
        self.phase = 'requesting_stable'
        req = ReplanStable.Request()
        req.current_position.x, req.current_position.y, req.current_position.z = self.last_position
        self.stable_service.call_async(req).add_done_callback(self.stable_received)

    def stable_received(self, future):
        try:
            response = future.result()
            if not response.marker_ids or response.marker_ids[-1] != 'HOME':
                raise ValueError('Stable route does not end at F6')
            self.queue = self.unpack(response)
            self.index = 0
            self.phase = 'stable'
            self.send_next()
        except Exception as e:
            self.fail('Stable route: %s' % e)

    def fail(self, message):
        self.phase = 'error'
        self.get_logger().error(message)


def main():
    rclpy.init()
    node = WayPointClient()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()

if __name__ == '__main__':
    main()
