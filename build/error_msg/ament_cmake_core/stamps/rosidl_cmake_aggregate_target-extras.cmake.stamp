# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target error_msg::error_msg
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${error_msg_TARGETS}.
if(error_msg_TARGETS AND NOT TARGET error_msg::error_msg)
  add_library(error_msg::error_msg INTERFACE IMPORTED)
  set_target_properties(error_msg::error_msg PROPERTIES
    INTERFACE_LINK_LIBRARIES "${error_msg_TARGETS}")
endif()
