// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from error_msg:msg/Error.idl
// generated code does not contain a copyright notice

#ifndef ERROR_MSG__MSG__DETAIL__ERROR__STRUCT_H_
#define ERROR_MSG__MSG__DETAIL__ERROR__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in msg/Error in the package error_msg.
typedef struct error_msg__msg__Error
{
  float roll_error;
  float pitch_error;
  float throttle_error;
  float yaw_error;
} error_msg__msg__Error;

// Struct for a sequence of error_msg__msg__Error.
typedef struct error_msg__msg__Error__Sequence
{
  error_msg__msg__Error * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} error_msg__msg__Error__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ERROR_MSG__MSG__DETAIL__ERROR__STRUCT_H_
