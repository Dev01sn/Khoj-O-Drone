#!/usr/bin/env python3
from __future__ import annotations
import argparse
import csv
import itertools
import math
import re
import sys
import threading
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional, Sequence, Tuple

import cv2
import numpy as np

try:
    import rclpy
    from geometry_msgs.msg import Pose
    from rclpy.callback_groups import ReentrantCallbackGroup
    from rclpy.executors import MultiThreadedExecutor
    from rclpy.node import Node
    from rclpy.qos import DurabilityPolicy, QoSProfile, ReliabilityPolicy
    from sensor_msgs.msg import Image
    from waypoint_navigation.srv import GetWaypoints
except ImportError:  # no ROS on this machine: the offline CLI still works
    rclpy = None
    Node = object

Point = Tuple[float, float]

# Arena layout 
ARENA_PX = 900                 # the playing field is warped to 900 x 900 pixels
GRID_CELLS = 12                # cells ...
CELL_PX = ARENA_PX // GRID_CELLS   # ... of 75 px, so 11 x 11 interior intersections
FLIGHT_HEIGHT = 13.0 
START = "F6" 
CORNER_MARKER_IDS = (80, 85, 90, 95)
GRID_CSV_NAME = "grid_whycode_coordinates.csv"

INTERSECTION_NAME = re.compile(r"^[A-K](?:[1-9]|1[01])$")   # A1 .. K11

# Detection tuning - the values most likely to need a tweak on the real frame
# Red wraps around the ends of OpenCV's 0-179 hue scale, so it needs two bands.
RED_HSV_BANDS = (((0, 120, 100), (10, 255, 255)),
                 ((170, 120, 100), (179, 255, 255)))
TRIANGLE_AREA_PX = (300, 8000)   # accepted blob size on the 900 x 900 image
MIN_SOLIDITY = 0.80              # blob area / convex-hull area; triangles are ~1.0

# Exhaustive search is exact and instant up to 8 stops (8! = 40,320 orders).
MAX_EXACT_STOPS = 8

@dataclass(frozen=True)
class Waypoint:
    """A named grid intersection with its whycode coordinates."""
    name: str
    x: float
    y: float

    @property
    def xy(self) -> Point:
        return (self.x, self.y)

@dataclass(frozen=True)
class Survivor:
    """A critical survivor found in the rectified arena image."""
    name: str                     # nearest intersection, e.g. "D5"
    centre: Point                 # pixel position on the 900 x 900 image
    outline: np.ndarray = field(compare=False, repr=False)

class DetectionError(RuntimeError):
    """The frame could not be turned into a rectified arena."""


#  Vision
_ENCODINGS = {
    "bgr8": (3, None),
    "rgb8": (3, cv2.COLOR_RGB2BGR),
    "rgba8": (4, cv2.COLOR_RGBA2BGR),
    "bgra8": (4, cv2.COLOR_BGRA2BGR),
    "mono8": (1, cv2.COLOR_GRAY2BGR),
}


def image_msg_to_bgr(msg) -> np.ndarray:
    """Convert a sensor_msgs/Image to a BGR array (avoids a cv_bridge dependency)."""
    try:
        channels, conversion = _ENCODINGS[msg.encoding.lower()]
    except KeyError:
        raise ValueError("unsupported image encoding %r" % msg.encoding) from None
    rows = np.frombuffer(msg.data, dtype=np.uint8).reshape(msg.height, msg.step)  # bytes, array('B') or ndarray
    pixels = rows[:, : msg.width * channels].reshape(msg.height, msg.width, channels)
    pixels = np.ascontiguousarray(pixels)          # drop any row padding
    return pixels if conversion is None else cv2.cvtColor(pixels, conversion)


def _detect_corner_markers(frame: np.ndarray) -> Dict[int, np.ndarray]:
    """ArUco 4x4 detection. Handles both OpenCV API generations (they differ)."""
    dictionary = cv2.aruco.getPredefinedDictionary(cv2.aruco.DICT_4X4_250)
    if hasattr(cv2.aruco, "ArucoDetector"):                      # OpenCV >= 4.7
        detector = cv2.aruco.ArucoDetector(dictionary, cv2.aruco.DetectorParameters())
        corners, ids, _ = detector.detectMarkers(frame)
    else:
        corners, ids, _ = cv2.aruco.detectMarkers(
            frame, dictionary, parameters=cv2.aruco.DetectorParameters_create())
    if ids is None:
        return {}
    return {int(i): c.reshape(4, 2).astype(np.float32) for c, i in zip(corners, ids.flatten())}


def _playing_field_corners(markers: Dict[int, np.ndarray]) -> np.ndarray:
    ids = sorted(CORNER_MARKER_IDS)
    arena_centre = np.mean([markers[i].mean(axis=0) for i in ids], axis=0)
    inner = np.array([markers[i][np.argmin(np.linalg.norm(markers[i] - arena_centre, axis=1))]
                      for i in ids])
    x_plus_y = inner.sum(axis=1)
    x_minus_y = inner[:, 0] - inner[:, 1]
    return np.array([inner[np.argmin(x_plus_y)],      # top-left
                     inner[np.argmax(x_minus_y)],     # top-right
                     inner[np.argmax(x_plus_y)],      # bottom-right
                     inner[np.argmin(x_minus_y)]],    # bottom-left
                    dtype=np.float32)


def rectify_arena(frame: np.ndarray) -> np.ndarray:
    """Return a square-on 900 x 900 view of the playing field only."""
    markers = _detect_corner_markers(frame)
    missing = sorted(set(CORNER_MARKER_IDS) - set(markers))
    if missing:
        raise DetectionError("corner markers %s not found (saw %s)" % (missing, sorted(markers)))
    edge = ARENA_PX - 1
    target = np.array([[0, 0], [edge, 0], [edge, edge], [0, edge]], dtype=np.float32)
    transform = cv2.getPerspectiveTransform(_playing_field_corners(markers), target)
    return cv2.warpPerspective(frame, transform, (ARENA_PX, ARENA_PX))


def intersection_name(px: float, py: float) -> str:
    """Name of the interior intersection nearest to a pixel, e.g. (450, 450) -> "F6".

    Clamped to the 11 x 11 interior lines, so a centre near the arena edge still
    yields a valid label instead of something like "L0".
    """
    last = GRID_CELLS - 1
    col = min(max(round(px / CELL_PX), 1), last)
    row = min(max(round(py / CELL_PX), 1), last)
    return "%s%d" % (chr(ord("A") + col - 1), row)


def _red_mask(arena: np.ndarray) -> np.ndarray:
    hsv = cv2.cvtColor(arena, cv2.COLOR_BGR2HSV)
    mask = np.zeros(hsv.shape[:2], np.uint8)
    for low, high in RED_HSV_BANDS:
        mask |= cv2.inRange(hsv, np.array(low), np.array(high))
    kernel = np.ones((5, 5), np.uint8)
    mask = cv2.morphologyEx(mask, cv2.MORPH_OPEN, kernel)    # drop speckle
    return cv2.morphologyEx(mask, cv2.MORPH_CLOSE, kernel)   # fill pinholes


def find_critical_survivors(arena: np.ndarray) -> List[Survivor]:
    """Red triangles in a rectified arena, each reduced to one centre point."""
    contours, _ = cv2.findContours(_red_mask(arena), cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    low, high = TRIANGLE_AREA_PX
    survivors = []
    for outline in contours:
        area = cv2.contourArea(outline)
        hull_area = cv2.contourArea(cv2.convexHull(outline))
        if not low <= area <= high or hull_area == 0 or area / hull_area < MIN_SOLIDITY:
            continue
        perimeter = cv2.arcLength(outline, True)
        if len(cv2.approxPolyDP(outline, 0.04 * perimeter, True)) != 3:
            continue
        m = cv2.moments(outline)
        cx, cy = m["m10"] / m["m00"], m["m01"] / m["m00"]     # m00 > 0 since area >= low
        survivors.append(Survivor(intersection_name(cx, cy), (cx, cy), outline))
    return survivors


#  Geometry: coordinates and route planning (no ROS, no images)
def locate_grid_csv(explicit: str = "") -> Path:
    candidates = [Path(explicit)] if explicit else []
    candidates += [Path.home() / "pico_ws/src/swift_pico/scripts" / GRID_CSV_NAME,
                   Path(__file__).resolve().parent / GRID_CSV_NAME]
    for path in candidates:
        if path.is_file():
            return path
    raise FileNotFoundError("%s not found; looked in: %s" % (GRID_CSV_NAME, ", ".join(map(str, candidates))))


def _find_xy_columns(header: Sequence[str]) -> Optional[Tuple[int, int]]:
    def find(axis: str) -> Optional[int]:
        tokens = [re.split(r"[^a-z0-9]+", h) for h in header]
        for i, h in enumerate(header):
            if h == axis:
                return i
        for i, (h, parts) in enumerate(zip(header, tokens)):
            if axis in parts and "pixel" not in h and "px" not in parts:
                return i
        return None
    x, y = find("x"), find("y")
    return (x, y) if x is not None and y is not None else None


def _as_float(text: str) -> Optional[float]:
    try:
        return float(text)
    except ValueError:
        return None


def load_grid_coordinates(path: Path) -> Dict[str, Point]:
    with open(path, newline="") as f:
        rows = [[cell.strip() for cell in row] for row in csv.reader(f) if any(c.strip() for c in row)]
    has_header = bool(rows) and not any(INTERSECTION_NAME.match(c.upper()) for c in rows[0])
    columns = _find_xy_columns([c.lower() for c in rows.pop(0)]) if has_header else None

    coordinates: Dict[str, Point] = {}
    for row in rows:
        name_at = next((i for i, c in enumerate(row) if INTERSECTION_NAME.match(c.upper())), None)
        if name_at is None:
            continue
        if columns:
            x, y = _as_float(row[columns[0]]), _as_float(row[columns[1]])
        else:
            numbers = [n for i, c in enumerate(row) if i != name_at for n in [_as_float(c)] if n is not None]
            x, y = (numbers[0], numbers[1]) if len(numbers) >= 2 else (None, None)
        if x is not None and y is not None:
            coordinates[row[name_at].upper()] = (x, y)
    if not coordinates:
        raise ValueError("no intersection coordinates could be read from %s" % path)
    return coordinates


def path_length(start: Point, stops: Sequence[Waypoint]) -> float:
    """Total straight-line distance flying start -> stops[0] -> stops[1] -> ..."""
    points = [start] + [s.xy for s in stops]
    return sum(math.dist(a, b) for a, b in zip(points, points[1:]))


def plan_route(start: Point, stops: Sequence[Waypoint]) -> List[Waypoint]:
    if len(stops) <= 1:
        return list(stops)
    if len(stops) <= MAX_EXACT_STOPS:
        return list(min(itertools.permutations(stops), key=lambda order: path_length(start, order)))
    remaining, route, here = list(stops), [], start
    while remaining:
        nearest = min(remaining, key=lambda s: math.dist(here, s.xy))
        remaining.remove(nearest)
        route.append(nearest)
        here = nearest.xy
    return route


def build_route(names: Sequence[str], coordinates: Dict[str, Point]) -> List[Waypoint]:
    if START not in coordinates:
        raise KeyError("start intersection %s missing from the coordinate file" % START)
    stops = []
    for name in dict.fromkeys(names):               # de-duplicate, keep order
        if name not in coordinates:
            raise KeyError("intersection %s missing from the coordinate file" % name)
        stops.append(Waypoint(name, *coordinates[name]))
    return plan_route(coordinates[START], stops)


# Debug picture 
def _pixel_of(name: str) -> Tuple[int, int]:
    return (ord(name[0]) - ord("A") + 1) * CELL_PX, int(name[1:]) * CELL_PX


def draw_debug_image(arena: np.ndarray, survivors: Sequence[Survivor], route: Sequence[Waypoint]) -> np.ndarray:
    img = arena.copy()
    for k in range(1, GRID_CELLS):
        cv2.line(img, (k * CELL_PX, 0), (k * CELL_PX, ARENA_PX), (0, 255, 0), 1)
        cv2.line(img, (0, k * CELL_PX), (ARENA_PX, k * CELL_PX), (0, 255, 0), 1)
    legs = [_pixel_of(START)] + [_pixel_of(w.name) for w in route]
    for a, b in zip(legs, legs[1:]):
        cv2.line(img, a, b, (0, 165, 255), 3)
    for s in survivors:
        cv2.drawContours(img, [s.outline], -1, (255, 0, 255), 2)
        cv2.circle(img, (int(s.centre[0]), int(s.centre[1])), 4, (0, 0, 0), -1)
    white = (255, 255, 255)
    cv2.putText(img, "START " + START, (legs[0][0] + 8, legs[0][1] + 24), cv2.FONT_HERSHEY_SIMPLEX, 0.7, white, 2)
    for order, w in enumerate(route, 1):
        px, py = _pixel_of(w.name)
        cv2.putText(img, "%d:%s" % (order, w.name), (px + 8, py - 8), cv2.FONT_HERSHEY_SIMPLEX, 0.7, white, 2)
    return img


# 3. ROS node
class WaypointService(Node):

    def __init__(self) -> None:
        super().__init__("waypoint_service")
        self.declare_parameter("grid_csv", "")         # override the CSV location
        self.declare_parameter("debug_image", "")      # e.g. /tmp/route.png

        self._route: List[Waypoint] = []
        self._route_ready = threading.Event()          # set once planning has finished
        self._frame_seen = False

        # The service may have to wait for the image callback, so they must be able
        # to run on different executor threads at the same time.
        group = ReentrantCallbackGroup()
        # The camera publishes ONE latched frame: match it, or a late start misses it.
        latched = QoSProfile(depth=1,
                             reliability=ReliabilityPolicy.RELIABLE,
                             durability=DurabilityPolicy.TRANSIENT_LOCAL)
        self._image_sub = self.create_subscription(Image, "/image_raw", self._on_image, latched,
                                                   callback_group=group)
        self.create_service(GetWaypoints, "waypoints", self._on_request, callback_group=group)
        self.get_logger().info("Waiting for one /image_raw frame...")

    # image -> route 
    def _on_image(self, msg) -> None:
        if self._frame_seen:                           # a second delivery changes nothing
            return
        self._frame_seen = True
        try:
            route = self._plan(msg)
        except Exception as error:
            # keep the subscription so a later frame can retry; requests keep waiting
            self.get_logger().error("Route planning failed: %s (waiting for another frame)" % error)
            self._frame_seen = False
            return
        self._route = route
        self._route_ready.set()
        self.destroy_subscription(self._image_sub)

    def _plan(self, msg) -> List[Waypoint]:
        coordinates = load_grid_coordinates(locate_grid_csv(self.get_parameter("grid_csv").value))
        arena = rectify_arena(image_msg_to_bgr(msg))
        survivors = find_critical_survivors(arena)
        names = [s.name for s in survivors]
        self.get_logger().info("Critical survivors (%d): %s" % (len(names), names))
        if len(names) != 4:
            self.get_logger().warn("Expected 4 critical survivors - check the red detection settings.")

        route = build_route(names, coordinates)
        length = path_length(coordinates[START], route)
        self.get_logger().info("Route from %s (%.2f units): %s" % (
            START, length, " -> ".join("%s(%.2f, %.2f)" % (w.name, w.x, w.y) for w in route)))

        debug_path = self.get_parameter("debug_image").value
        if debug_path:
            cv2.imwrite(debug_path, draw_debug_image(arena, survivors, route))
        return route

    # -- GetWaypoints --
    def _on_request(self, request, response):
        if not self._route_ready.is_set():
            self.get_logger().info("Request arrived before the route was ready - waiting.")
            while rclpy.ok() and not self._route_ready.wait(timeout=0.1):
                pass
        if request.get_waypoints:                      # the client sets get_waypoints = True
            has_ids = hasattr(response, "marker_ids")  # optional field
            for w in self._route:
                pose = Pose()
                pose.position.x, pose.position.y, pose.position.z = w.x, w.y, FLIGHT_HEIGHT
                response.waypoints.poses.append(pose)
                if has_ids:
                    response.marker_ids.append(w.name)  # parallel to poses: which survivor each is
            response.waypoints.header.frame_id = "whycode"
            self.get_logger().info("Sent %d waypoints." % len(self._route))
        return response


def run_node(args=None) -> None:
    rclpy.init(args=args)
    node = WaypointService()
    executor = MultiThreadedExecutor(num_threads=4)
    executor.add_node(node)
    try:
        executor.spin()
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


# 4. Offline check
def run_offline(argv: Sequence[str]) -> None:
    parser = argparse.ArgumentParser(description="Plan a route from a saved frame (no ROS needed).")
    parser.add_argument("--image", required=True, help="saved camera frame")
    parser.add_argument("--csv", default="", help="grid_whycode_coordinates.csv (auto-located if omitted)")
    parser.add_argument("--debug", default="", help="write the debug picture here")
    args, _ = parser.parse_known_args(argv)

    frame = cv2.imread(args.image)
    if frame is None:
        sys.exit("ERROR: could not load image: %s" % args.image)
    coordinates = load_grid_coordinates(locate_grid_csv(args.csv))
    arena = rectify_arena(frame)
    survivors = find_critical_survivors(arena)
    route = build_route([s.name for s in survivors], coordinates)

    print("Critical survivors (%d): %s" % (len(survivors), [s.name for s in survivors]))
    print("Route from %s, %.2f units:" % (START, path_length(coordinates[START], route)))
    for order, w in enumerate(route, 1):
        print("  %d. %-4s (%.2f, %.2f, %.1f)" % (order, w.name, w.x, w.y, FLIGHT_HEIGHT))
    if args.debug:
        cv2.imwrite(args.debug, draw_debug_image(arena, survivors, route))
        print("Saved", args.debug)


def main(args=None) -> None:
    if "--image" in sys.argv:
        run_offline(sys.argv[1:])
    elif rclpy is None:
        sys.exit("ERROR: ROS 2 (rclpy) is not available; use --image for the offline check.")
    else:
        run_node(args)


if __name__ == "__main__":
    main()