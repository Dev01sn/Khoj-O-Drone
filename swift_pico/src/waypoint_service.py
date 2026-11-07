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



# Task 3A detection and routing (kept in this service, alongside Task 2C helpers).
import heapq
import json
from geometry_msgs.msg import Pose
from waypoint_navigation.srv import ReplanStable

def _pixel_of(name):
    return (ord(name[0]) - ord("A") + 1) * CELL_PX, int(name[1:]) * CELL_PX

STEP = 5
MARGIN = 35  # pixels on the 900 x 900 view; verify against drone size and drift


def yellow_survivors(arena):
    hsv = cv2.cvtColor(arena, cv2.COLOR_BGR2HSV)
    mask = cv2.inRange(hsv, np.array((20, 130, 140)), np.array((40, 255, 255)))
    mask = cv2.morphologyEx(mask, cv2.MORPH_OPEN, np.ones((5, 5), np.uint8))
    contours, _ = cv2.findContours(mask, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    found = []
    for c in contours:
        area = cv2.contourArea(c)
        perimeter = cv2.arcLength(c, True)
        if not 300 <= area <= 8000 or perimeter == 0:
            continue
        if 4 * math.pi * area / (perimeter * perimeter) < 0.65:
            continue
        m = cv2.moments(c)
        if not m['m00']:
            continue
        x, y = m['m10'] / m['m00'], m['m01'] / m['m00']
        found.append(Survivor(intersection_name(x, y), (x, y), c))
    return found


def obstacle_mask(arena):
    hsv = cv2.cvtColor(arena, cv2.COLOR_BGR2HSV)
    dark = cv2.inRange(hsv, np.array((0, 0, 0)), np.array((179, 255, 30)))
    dark = cv2.morphologyEx(dark, cv2.MORPH_OPEN, np.ones((5, 5), np.uint8))
    contours, _ = cv2.findContours(dark, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    obstacles = [c for c in contours if 8000 <= cv2.contourArea(c) <= 50000]
    mask = np.zeros(dark.shape, np.uint8)
    cv2.drawContours(mask, obstacles, -1, 255, thickness=cv2.FILLED)
    kernel = cv2.getStructuringElement(cv2.MORPH_ELLIPSE, (2*MARGIN+1, 2*MARGIN+1))
    return obstacles, cv2.dilate(mask, kernel)


def grid_point(xy):
    return tuple(int(round(float(v) / STEP)) for v in xy)


def astar(mask, start, end):
    height, width = mask.shape
    start, end = grid_point(start), grid_point(end)
    def open_cell(p):
        x, y = p[0]*STEP, p[1]*STEP
        return 0 <= x < width and 0 <= y < height and mask[y, x] == 0
    if not open_cell(start) or not open_cell(end):
        raise ValueError('Start or destination is inside an off-limits area')
    frontier = [(0.0, start)]
    cost = {start: 0.0}
    parent = {}
    while frontier:
        _, here = heapq.heappop(frontier)
        if here == end:
            points = [here]
            while points[-1] != start:
                points.append(parent[points[-1]])
            return [(x*STEP, y*STEP) for x, y in reversed(points)]
        for dx, dy in ((1,0),(-1,0),(0,1),(0,-1),(1,1),(1,-1),(-1,1),(-1,-1)):
            nxt = (here[0]+dx, here[1]+dy)
            if not open_cell(nxt):
                continue
            if dx and dy and (not open_cell((here[0]+dx, here[1])) or not open_cell((here[0], here[1]+dy))):
                continue
            new = cost[here] + math.hypot(dx, dy)
            if new < cost.get(nxt, float('inf')):
                cost[nxt] = new
                parent[nxt] = here
                heapq.heappush(frontier, (new + math.dist(nxt, end), nxt))
    raise ValueError('No clear route between %s and %s' % (start, end))


def clear_line(mask, a, b):
    # Check every pixel on the segment, not only the endpoints.
    canvas = np.zeros(mask.shape, np.uint8)
    cv2.line(canvas, a, b, 255, 1)
    return not np.any(cv2.bitwise_and(canvas, mask))


def simplify(mask, points):
    out = [points[0]]
    i = 0
    while i < len(points)-1:
        j = len(points)-1
        while j > i+1 and not clear_line(mask, points[i], points[j]):
            j -= 1
        if not clear_line(mask, points[i], points[j]):
            raise ValueError('Grid segment intersects obstacle; increase resolution')
        out.append(points[j])
        i = j
    return out


def route(mask, start, stops, home=None):
    best = None
    for order in itertools.permutations(stops):
        destinations = list(order) + ([('F6', home)] if home is not None else [])
        here, legs, length = start, [], 0.0
        try:
            for name, dest in destinations:
                path = simplify(mask, astar(mask, here, dest))
                length += sum(math.dist(a, b) for a, b in zip(path, path[1:]))
                legs.append((name, path))
                here = dest
        except ValueError:
            continue
        if best is None or length < best[0]:
            best = (length, legs)
    if best is None:
        raise ValueError('No route found; inspect the mask and start position')
    return best



def load_grid():
    from ament_index_python.packages import get_package_share_directory
    path = Path(get_package_share_directory('swift_pico')) / 'scripts' / 'grid_whycode_coordinates.json'
    with path.open() as f:
        raw = json.load(f)
    return {k: (float(v['x']), float(v['y'])) for k, v in raw.items()}


class WaypointService(Node):
    def __init__(self):
        super().__init__('waypoint_service')
        self.declare_parameter('debug_image', '')
        self.ready = threading.Event()
        self.planning = threading.Lock()
        self.grid = load_grid()
        # Calibration from the team's grid, used to map transit points and feedback.
        names = sorted(self.grid)
        pixels = np.array([[*_pixel_of(n), 1.0] for n in names])
        world = np.array([self.grid[n] for n in names])
        self.mapping = np.linalg.lstsq(pixels, world, rcond=None)[0]
        self.inverse = np.linalg.inv(self.mapping[:2, :])
        self.mask = None
        self.stable = []
        self.critical_route = []
        group = ReentrantCallbackGroup()
        qos = QoSProfile(depth=1, reliability=ReliabilityPolicy.RELIABLE,
                         durability=DurabilityPolicy.TRANSIENT_LOCAL)
        self.image_sub = self.create_subscription(Image, '/image_raw', self.image_callback, qos, callback_group=group)
        self.create_service(GetWaypoints, 'waypoints', self.critical_callback, callback_group=group)
        self.create_service(ReplanStable, 'replan_stable', self.stable_callback, callback_group=group)
        self.get_logger().info('Waiting for the arena image')

    def pixel_to_world(self, p):
        xy = np.array([p[0], p[1], 1.0]) @ self.mapping
        return float(xy[0]), float(xy[1])

    def world_to_pixel(self, x, y):
        p = (np.array([x, y]) - self.mapping[2, :]) @ self.inverse
        return float(p[0]), float(p[1])

    def make_route(self, start, survivors, home=False):
        destinations = [(s.name, _pixel_of(s.name)) for s in survivors]
        _, legs = route(self.mask, start, destinations,
                                home=_pixel_of('F6') if home else None)
        goals = []
        for name, path in legs:
            for p in path[1:-1]:
                x, y = self.pixel_to_world(p)
                goals.append((x, y, 'TRANSIT'))
            x, y = self.grid[name]
            goals.append((x, y, 'HOME' if name == 'F6' and home else 'SURVIVOR:' + name))
        return goals

    def image_callback(self, msg):
        if self.ready.is_set() or not self.planning.acquire(blocking=False):
            return
        try:
            arena = rectify_arena(image_msg_to_bgr(msg))
            critical = find_critical_survivors(arena)
            stable = yellow_survivors(arena)
            obstacles, mask = obstacle_mask(arena)
            self.mask = mask
            self.stable = stable
            self.critical_route = self.make_route(_pixel_of('F6'), critical)
            path = self.get_parameter('debug_image').value
            if path:
                overlay = arena.copy()
                overlay[mask != 0] = (overlay[mask != 0] * 0.55 + np.array([0, 0, 100])*0.45).astype(np.uint8)
                for s in critical + stable:
                    cv2.putText(overlay, s.name, tuple(map(int, s.centre)), cv2.FONT_HERSHEY_SIMPLEX, .6, (255,255,255), 2)
                cv2.imwrite(path, overlay)
            self.ready.set()
            self.get_logger().info('Detected critical %s; stable %s; obstacles %d' % ([s.name for s in critical], [s.name for s in stable], len(obstacles)))
        except Exception as e:
            self.get_logger().error('Arena not ready: %s' % e)
        finally:
            self.planning.release()

    def fill(self, response, goals):
        response.waypoints.header.frame_id = 'whycode'
        for x, y, label in goals:
            p = Pose()
            p.position.x, p.position.y, p.position.z = x, y, 13.0
            p.orientation.w = 1.0
            response.waypoints.poses.append(p)
            response.marker_ids.append(label)
        return response

    def wait_ready(self):
        while rclpy.ok() and not self.ready.wait(.1):
            pass
        return self.ready.is_set()

    def critical_callback(self, request, response):
        if request.get_waypoints and self.wait_ready():
            self.fill(response, self.critical_route)
        return response

    def stable_callback(self, request, response):
        if not self.wait_ready():
            return response
        pos = request.current_position
        try:
            p = self.world_to_pixel(pos.x, pos.y)
            self.fill(response, self.make_route(p, self.stable, home=True))
        except Exception as e:
            self.get_logger().error('Stable replan failed: %s' % e)
        return response


def main():
    rclpy.init()
    node = WaypointService()
    executor = MultiThreadedExecutor(num_threads=4)
    executor.add_node(node)
    try:
        executor.spin()
    finally:
        executor.shutdown()
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()

if __name__ == '__main__':
    main()
