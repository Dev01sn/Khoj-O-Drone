# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target controller_msg::controller_msg
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${controller_msg_TARGETS}.
if(controller_msg_TARGETS AND NOT TARGET controller_msg::controller_msg)
  add_library(controller_msg::controller_msg INTERFACE IMPORTED)
  set_target_properties(controller_msg::controller_msg PROPERTIES
    INTERFACE_LINK_LIBRARIES "${controller_msg_TARGETS}")
endif()
