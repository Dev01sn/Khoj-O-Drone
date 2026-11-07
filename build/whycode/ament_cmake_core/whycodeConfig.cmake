# generated from ament/cmake/core/templates/nameConfig.cmake.in

# prevent multiple inclusion
if(_whycode_CONFIG_INCLUDED)
  # ensure to keep the found flag the same
  if(NOT DEFINED whycode_FOUND)
    # explicitly set it to FALSE, otherwise CMake will set it to TRUE
    set(whycode_FOUND FALSE)
  elseif(NOT whycode_FOUND)
    # use separate condition to avoid uninitialized variable warning
    set(whycode_FOUND FALSE)
  endif()
  return()
endif()
set(_whycode_CONFIG_INCLUDED TRUE)

# output package information
if(NOT whycode_FIND_QUIETLY)
  message(STATUS "Found whycode: 0.0.1 (${whycode_DIR})")
endif()

# warn when using a deprecated package
if(NOT "" STREQUAL "")
  set(_msg "Package 'whycode' is deprecated")
  # append custom deprecation text if available
  if(NOT "" STREQUAL "TRUE")
    set(_msg "${_msg} ()")
  endif()
  # optionally quiet the deprecation message
  if(NOT ${whycode_DEPRECATED_QUIET})
    message(DEPRECATION "${_msg}")
  endif()
endif()

# flag package as ament-based to distinguish it after being find_package()-ed
set(whycode_FOUND_AMENT_PACKAGE TRUE)

# include all config extra files
set(_extras "")
foreach(_extra ${_extras})
  include("${whycode_DIR}/${_extra}")
endforeach()
