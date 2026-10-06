#!/usr/bin/env python3
 
import argparse
import os
import sys
 
import cv2
import numpy as np
 
REQUIRED_IDS = {80, 85, 90, 95}
SIZE = 900            
CELLS = 12            
STEP = SIZE // CELLS  
 
 
RED_RANGES = [((0, 120, 100), (10, 255, 255)), ((170, 120, 100), (179, 255, 255))]
YELLOW_RANGES = [((20, 100, 100), (35, 255, 255))]
 
MIN_AREA = 200
 
 
 
def detect_markers(img):
    dictionary = cv2.aruco.getPredefinedDictionary(cv2.aruco.DICT_4X4_250)
    if hasattr(cv2.aruco, "ArucoDetector"):            
        detector = cv2.aruco.ArucoDetector(dictionary, cv2.aruco.DetectorParameters())
        corners, ids, _ = detector.detectMarkers(img)
    else: 
        corners, ids, _ = cv2.aruco.detectMarkers(
            img, dictionary, parameters=cv2.aruco.DetectorParameters_create())
    found = {}
    if ids is not None:
        for c, i in zip(corners, ids.flatten()):
            found[int(i)] = c.reshape(4, 2).astype(np.float32)
    return found
 
 
def field_corners(markers):
    ids = sorted(REQUIRED_IDS)
    centres = np.array([markers[i].mean(axis=0) for i in ids])
    mid = centres.mean(axis=0)
    inner = np.array([markers[i][np.argmin(np.linalg.norm(markers[i] - mid, axis=1))]
                      for i in ids])
    s = inner.sum(axis=1)
    d = inner[:, 0] - inner[:, 1]
    tl, br = inner[np.argmin(s)], inner[np.argmax(s)]
    tr, bl = inner[np.argmax(d)], inner[np.argmin(d)]
    return np.array([tl, tr, br, bl], dtype=np.float32)
 
 
def warp(img, src):
    dst = np.array([[0, 0], [SIZE - 1, 0], [SIZE - 1, SIZE - 1], [0, SIZE - 1]],
                   dtype=np.float32)
    M = cv2.getPerspectiveTransform(src, dst)
    return cv2.warpPerspective(img, M, (SIZE, SIZE))
 
 
def colour_mask(hsv, ranges):
    mask = np.zeros(hsv.shape[:2], np.uint8)
    for lo, hi in ranges:
        mask |= cv2.inRange(hsv, np.array(lo), np.array(hi))
    kernel = np.ones((5, 5), np.uint8)
    mask = cv2.morphologyEx(mask, cv2.MORPH_OPEN, kernel)
    return cv2.morphologyEx(mask, cv2.MORPH_CLOSE, kernel)
 
 
def blob_centres(mask):
    contours, _ = cv2.findContours(mask, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    out = []
    for c in contours:
        if cv2.contourArea(c) < MIN_AREA:
            continue
        m = cv2.moments(c)
        if m["m00"] > 0:
            out.append((m["m10"] / m["m00"], m["m01"] / m["m00"]))
    return out
 
 
def label_for(cx, cy):
    col = min(max(int(round(cx / STEP)), 1), CELLS - 1)
    row = min(max(int(round(cy / STEP)), 1), CELLS - 1)
    return "%s%d" % (chr(ord("A") + col - 1), row)
 
 
def find_survivors(warped):
    hsv = cv2.cvtColor(warped, cv2.COLOR_BGR2HSV)
    critical = {label_for(*c) for c in blob_centres(colour_mask(hsv, RED_RANGES))}
    stable = {label_for(*c) for c in blob_centres(colour_mask(hsv, YELLOW_RANGES))}
    return sorted(critical), sorted(stable)
 
 
def draw_debug(warped):
    dbg = warped.copy()
    for k in range(1, CELLS):
        cv2.line(dbg, (k * STEP, 0), (k * STEP, SIZE), (0, 255, 0), 1)
        cv2.line(dbg, (0, k * STEP), (SIZE, k * STEP), (0, 255, 0), 1)
    hsv = cv2.cvtColor(warped, cv2.COLOR_BGR2HSV)
    for ranges, colour in ((RED_RANGES, (255, 0, 255)), (YELLOW_RANGES, (255, 255, 255))):
        mask = colour_mask(hsv, ranges)
        contours, _ = cv2.findContours(mask, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
        for c in contours:
            if cv2.contourArea(c) < MIN_AREA:
                continue
            m = cv2.moments(c)
            if m["m00"] == 0:
                continue
            cx, cy = m["m10"] / m["m00"], m["m01"] / m["m00"]
            cv2.drawContours(dbg, [c], -1, colour, 2)
            cv2.circle(dbg, (int(cx), int(cy)), 4, (0, 0, 0), -1)
            cv2.putText(dbg, label_for(cx, cy), (int(cx) + 8, int(cy) - 8),
                        cv2.FONT_HERSHEY_SIMPLEX, 0.7, (0, 0, 0), 3)
            cv2.putText(dbg, label_for(cx, cy), (int(cx) + 8, int(cy) - 8),
                        cv2.FONT_HERSHEY_SIMPLEX, 0.7, colour, 1)
    return dbg
 
 
def write_results(image_path, ids, critical, stable):
    out = os.path.splitext(image_path)[0] + "_results.txt"
    lines = [
        "Detected marker IDs: [%s]" % ", ".join(str(i) for i in sorted(ids)),
        "",
        "Critical Survivors: " + ", ".join(critical),
        "Stable Survivors: " + ", ".join(stable),
    ]
    with open(out, "w") as f:
        f.write("\n".join(lines) + "\n")
    return out
 
 
def main():
    ap = argparse.ArgumentParser(description="Task 1A survivor detection")
    ap.add_argument("--image", required=True, help="path to the arena image")
    ap.add_argument("--debug", action="store_true",
                    help="save composite debug image <name>_debug.png (no GUI)")
    args = ap.parse_args()
 
    img = cv2.imread(args.image)
    if img is None:
        sys.exit("ERROR: could not load image: %s" % args.image)
 
    markers = detect_markers(img)
    print("Detected marker IDs:", sorted(markers))
    missing = REQUIRED_IDS - set(markers)
    if missing:
        sys.exit("ERROR: missing corner markers %s - cannot continue." % sorted(missing))
 
    warped = warp(img, field_corners(markers))
    critical, stable = find_survivors(warped)
    print("Critical Survivors:", critical)
    print("Stable Survivors:  ", stable)
 
    if args.debug:
        cv2.imwrite(os.path.splitext(args.image)[0] + "_debug.png", draw_debug(warped))
 
    out = write_results(args.image, REQUIRED_IDS & set(markers), critical, stable)
    print("Wrote", out)
 
 
if __name__ == "__main__":
    main()

