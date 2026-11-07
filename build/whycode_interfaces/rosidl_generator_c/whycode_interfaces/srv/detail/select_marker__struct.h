// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from whycode_interfaces:srv/SelectMarker.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__SRV__DETAIL__SELECT_MARKER__STRUCT_H_
#define WHYCODE_INTERFACES__SRV__DETAIL__SELECT_MARKER__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'point'
#include "geometry_msgs/msg/detail/point__struct.h"

/// Struct defined in srv/SelectMarker in the package whycode_interfaces.
typedef struct whycode_interfaces__srv__SelectMarker_Request
{
  /// x,y of seleceted marker in image coordinates
  geometry_msgs__msg__Point point;
} whycode_interfaces__srv__SelectMarker_Request;

// Struct for a sequence of whycode_interfaces__srv__SelectMarker_Request.
typedef struct whycode_interfaces__srv__SelectMarker_Request__Sequence
{
  whycode_interfaces__srv__SelectMarker_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} whycode_interfaces__srv__SelectMarker_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/SelectMarker in the package whycode_interfaces.
typedef struct whycode_interfaces__srv__SelectMarker_Response
{
  /// bool feedback
  bool success;
} whycode_interfaces__srv__SelectMarker_Response;

// Struct for a sequence of whycode_interfaces__srv__SelectMarker_Response.
typedef struct whycode_interfaces__srv__SelectMarker_Response__Sequence
{
  whycode_interfaces__srv__SelectMarker_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} whycode_interfaces__srv__SelectMarker_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // WHYCODE_INTERFACES__SRV__DETAIL__SELECT_MARKER__STRUCT_H_
