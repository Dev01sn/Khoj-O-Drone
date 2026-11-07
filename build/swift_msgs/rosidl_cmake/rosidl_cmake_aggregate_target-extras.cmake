# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target swift_msgs::swift_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${swift_msgs_TARGETS}.
if(swift_msgs_TARGETS AND NOT TARGET swift_msgs::swift_msgs)
  add_library(swift_msgs::swift_msgs INTERFACE IMPORTED)
  set_target_properties(swift_msgs::swift_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${swift_msgs_TARGETS}")
endif()
