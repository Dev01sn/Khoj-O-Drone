// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from whycode_interfaces:msg/Marker.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__MSG__DETAIL__MARKER__STRUCT_H_
#define WHYCODE_INTERFACES__MSG__DETAIL__MARKER__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'position'
#include "geometry_msgs/msg/detail/pose__struct.h"
// Member 'rotation'
#include "geometry_msgs/msg/detail/vector3__struct.h"

/// Struct defined in msg/Marker in the package whycode_interfaces.
typedef struct whycode_interfaces__msg__Marker
{
  /// ID
  int8_t id;
  /// in pixels
  int32_t size;
  /// camera coordinates
  float u;
  /// camera coordinates
  float v;
  /// rotation around surface normal
  float angle;
  /// pose
  geometry_msgs__msg__Pose position;
  /// euler angles roll, pitch, yaw
  geometry_msgs__msg__Vector3 rotation;
} whycode_interfaces__msg__Marker;

// Struct for a sequence of whycode_interfaces__msg__Marker.
typedef struct whycode_interfaces__msg__Marker__Sequence
{
  whycode_interfaces__msg__Marker * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} whycode_interfaces__msg__Marker__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // WHYCODE_INTERFACES__MSG__DETAIL__MARKER__STRUCT_H_
