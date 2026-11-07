// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from whycode_interfaces:srv/GetGuiSettings.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__SRV__DETAIL__GET_GUI_SETTINGS__STRUCT_H_
#define WHYCODE_INTERFACES__SRV__DETAIL__GET_GUI_SETTINGS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/GetGuiSettings in the package whycode_interfaces.
typedef struct whycode_interfaces__srv__GetGuiSettings_Request
{
  uint8_t structure_needs_at_least_one_member;
} whycode_interfaces__srv__GetGuiSettings_Request;

// Struct for a sequence of whycode_interfaces__srv__GetGuiSettings_Request.
typedef struct whycode_interfaces__srv__GetGuiSettings_Request__Sequence
{
  whycode_interfaces__srv__GetGuiSettings_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} whycode_interfaces__srv__GetGuiSettings_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/GetGuiSettings in the package whycode_interfaces.
typedef struct whycode_interfaces__srv__GetGuiSettings_Response
{
  /// draw/hide coordinates
  bool draw_coords;
  /// draw/hide segmentation outcome
  bool draw_segments;
  /// 0 = camera coords
  /// 1 = 2D coords
  /// 2 = 3D coords
  int8_t coords;
} whycode_interfaces__srv__GetGuiSettings_Response;

// Struct for a sequence of whycode_interfaces__srv__GetGuiSettings_Response.
typedef struct whycode_interfaces__srv__GetGuiSettings_Response__Sequence
{
  whycode_interfaces__srv__GetGuiSettings_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} whycode_interfaces__srv__GetGuiSettings_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // WHYCODE_INTERFACES__SRV__DETAIL__GET_GUI_SETTINGS__STRUCT_H_
