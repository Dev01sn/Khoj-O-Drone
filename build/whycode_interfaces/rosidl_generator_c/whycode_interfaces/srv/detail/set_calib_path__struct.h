// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from whycode_interfaces:srv/SetCalibPath.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__SRV__DETAIL__SET_CALIB_PATH__STRUCT_H_
#define WHYCODE_INTERFACES__SRV__DETAIL__SET_CALIB_PATH__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'action'
// Member 'path'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/SetCalibPath in the package whycode_interfaces.
typedef struct whycode_interfaces__srv__SetCalibPath_Request
{
  /// either "save" or "load"
  rosidl_runtime_c__String action;
  /// path to yaml file
  rosidl_runtime_c__String path;
} whycode_interfaces__srv__SetCalibPath_Request;

// Struct for a sequence of whycode_interfaces__srv__SetCalibPath_Request.
typedef struct whycode_interfaces__srv__SetCalibPath_Request__Sequence
{
  whycode_interfaces__srv__SetCalibPath_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} whycode_interfaces__srv__SetCalibPath_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'msg'
// already included above
// #include "rosidl_runtime_c/string.h"

/// Struct defined in srv/SetCalibPath in the package whycode_interfaces.
typedef struct whycode_interfaces__srv__SetCalibPath_Response
{
  /// bool feedback
  bool success;
  /// information message
  rosidl_runtime_c__String msg;
} whycode_interfaces__srv__SetCalibPath_Response;

// Struct for a sequence of whycode_interfaces__srv__SetCalibPath_Response.
typedef struct whycode_interfaces__srv__SetCalibPath_Response__Sequence
{
  whycode_interfaces__srv__SetCalibPath_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} whycode_interfaces__srv__SetCalibPath_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // WHYCODE_INTERFACES__SRV__DETAIL__SET_CALIB_PATH__STRUCT_H_
