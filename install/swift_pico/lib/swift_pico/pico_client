#!/usr/bin/env python3

import time
import rclpy
from rclpy.action import ActionClient
from rclpy.node import Node
from action_msgs.msg import GoalStatus

from waypoint_navigation.action import NavToWaypoint
from waypoint_navigation.srv import GetWaypoints


class WayPointClient(Node):

    def __init__(self):
        super().__init__('waypoint_client')
        self.goals = []
        self.goal_index = 0

        self.action_client = ActionClient(self, NavToWaypoint, 'waypoint_navigation')

        self.cli = self.create_client(GetWaypoints, 'waypoints')
        while not self.cli.wait_for_service(timeout_sec=1.0):
            self.get_logger().info('service not available, waiting again...')

        self.req = GetWaypoints.Request()
        self.req.get_waypoints = True

    ###action client functions

    def send_goal(self, waypoint):

        goal_msg = NavToWaypoint.Goal()
        goal_msg.waypoint.position.x = waypoint[0]
        goal_msg.waypoint.position.y = waypoint[1]
        goal_msg.waypoint.position.z = waypoint[2]
        goal_msg.hover = True

        # blocks until the action server is up (polls, so it works before spin)
        self.action_client.wait_for_server()

        self.get_logger().info(
            f'Sending goal {self.goal_index + 1}/{len(self.goals)}: {waypoint}')
        self.send_goal_future = self.action_client.send_goal_async(
            goal_msg, feedback_callback=self.feedback_callback)
        self.send_goal_future.add_done_callback(self.goal_response_callback)

    def goal_response_callback(self, future):

        goal_handle = future.result()
        if not goal_handle.accepted:
            self.get_logger().error('Goal rejected by the action server')
            return

        self.get_logger().info('Goal accepted')
        self.get_result_future = goal_handle.get_result_async()
        self.get_result_future.add_done_callback(self.get_result_callback)

    def get_result_callback(self, future):

        response = future.result()
        if response.status != GoalStatus.STATUS_SUCCEEDED:
            # do not skip ahead: waypoints must be done in order
            self.get_logger().error(f'Goal did not succeed (status {response.status}); stopping.')
            return

        result = response.result
        self.get_logger().info('Result: {0}'.format(result.hov_time))

        self.goal_index += 1

        if self.goal_index < len(self.goals):
            self.send_goal(self.goals[self.goal_index])
        else:
            self.get_logger().info('All waypoints have been reached successfully')

    def feedback_callback(self, feedback_msg):

        feedback = feedback_msg.feedback
        x = feedback.current_waypoint.pose.position.x
        y = feedback.current_waypoint.pose.position.y
        z = feedback.current_waypoint.pose.position.z
        t = feedback.current_waypoint.header.stamp.sec
        # throttled: feedback arrives ~50 times a second
        self.get_logger().info(
            f'Received feedback! The current whycon position is: {x:.2f}, {y:.2f}, {z:.2f}',
            throttle_duration_sec=1.0)
        self.get_logger().info(f'Max time inside box: {t}', throttle_duration_sec=1.0)

    #service client functions

    def send_request(self):
        return self.cli.call_async(self.req)

    def receive_goals(self):
        future = self.send_request()
        rclpy.spin_until_future_complete(self, future)

        response = future.result()
        self.get_logger().info('Waypoints received by the action client')

        # keep exactly the order the service returned
        for pose in response.waypoints.poses:
            waypoints = [pose.position.x, pose.position.y, pose.position.z]
            self.goals.append(waypoints)
            self.get_logger().info(f'Waypoints: {waypoints}')

        if not self.goals:
            self.get_logger().error('Service returned no waypoints')
            return

        self.send_goal(self.goals[0])


def main(args=None):
    rclpy.init(args=args)

    waypoint_client = WayPointClient()
    waypoint_client.receive_goals()

    try:
        rclpy.spin(waypoint_client)
    except KeyboardInterrupt:
        waypoint_client.get_logger().info('KeyboardInterrupt, shutting down.\n')
    finally:
        waypoint_client.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()