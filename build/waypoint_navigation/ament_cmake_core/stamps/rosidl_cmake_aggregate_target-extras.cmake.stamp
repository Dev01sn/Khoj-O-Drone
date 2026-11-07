# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target waypoint_navigation::waypoint_navigation
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${waypoint_navigation_TARGETS}.
if(waypoint_navigation_TARGETS AND NOT TARGET waypoint_navigation::waypoint_navigation)
  add_library(waypoint_navigation::waypoint_navigation INTERFACE IMPORTED)
  set_target_properties(waypoint_navigation::waypoint_navigation PROPERTIES
    INTERFACE_LINK_LIBRARIES "${waypoint_navigation_TARGETS}")
endif()
