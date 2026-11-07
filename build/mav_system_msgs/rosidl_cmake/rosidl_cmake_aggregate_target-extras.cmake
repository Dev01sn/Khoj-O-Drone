# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target mav_system_msgs::mav_system_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${mav_system_msgs_TARGETS}.
if(mav_system_msgs_TARGETS AND NOT TARGET mav_system_msgs::mav_system_msgs)
  add_library(mav_system_msgs::mav_system_msgs INTERFACE IMPORTED)
  set_target_properties(mav_system_msgs::mav_system_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${mav_system_msgs_TARGETS}")
endif()
