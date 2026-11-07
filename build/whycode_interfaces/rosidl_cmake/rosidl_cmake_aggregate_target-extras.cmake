# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target whycode_interfaces::whycode_interfaces
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${whycode_interfaces_TARGETS}.
if(whycode_interfaces_TARGETS AND NOT TARGET whycode_interfaces::whycode_interfaces)
  add_library(whycode_interfaces::whycode_interfaces INTERFACE IMPORTED)
  set_target_properties(whycode_interfaces::whycode_interfaces PROPERTIES
    INTERFACE_LINK_LIBRARIES "${whycode_interfaces_TARGETS}")
endif()
