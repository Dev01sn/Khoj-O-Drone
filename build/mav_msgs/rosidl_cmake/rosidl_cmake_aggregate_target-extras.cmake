# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target mav_msgs::mav_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${mav_msgs_TARGETS}.
if(mav_msgs_TARGETS AND NOT TARGET mav_msgs::mav_msgs)
  add_library(mav_msgs::mav_msgs INTERFACE IMPORTED)
  set_target_properties(mav_msgs::mav_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${mav_msgs_TARGETS}")
endif()
