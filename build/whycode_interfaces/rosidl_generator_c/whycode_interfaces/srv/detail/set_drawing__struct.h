// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from whycode_interfaces:srv/SetDrawing.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__SRV__DETAIL__SET_DRAWING__STRUCT_H_
#define WHYCODE_INTERFACES__SRV__DETAIL__SET_DRAWING__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/SetDrawing in the package whycode_interfaces.
typedef struct whycode_interfaces__srv__SetDrawing_Request
{
  /// draw/hide coordinates
  bool draw_coords;
  /// draw/hide segmentation outcome
  bool draw_segments;
} whycode_interfaces__srv__SetDrawing_Request;

// Struct for a sequence of whycode_interfaces__srv__SetDrawing_Request.
typedef struct whycode_interfaces__srv__SetDrawing_Request__Sequence
{
  whycode_interfaces__srv__SetDrawing_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} whycode_interfaces__srv__SetDrawing_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/SetDrawing in the package whycode_interfaces.
typedef struct whycode_interfaces__srv__SetDrawing_Response
{
  /// bool feedback
  bool success;
} whycode_interfaces__srv__SetDrawing_Response;

// Struct for a sequence of whycode_interfaces__srv__SetDrawing_Response.
typedef struct whycode_interfaces__srv__SetDrawing_Response__Sequence
{
  whycode_interfaces__srv__SetDrawing_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} whycode_interfaces__srv__SetDrawing_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // WHYCODE_INTERFACES__SRV__DETAIL__SET_DRAWING__STRUCT_H_
