// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from controller_msg:msg/PIDTune.idl
// generated code does not contain a copyright notice

#ifndef CONTROLLER_MSG__MSG__DETAIL__PID_TUNE__STRUCT_H_
#define CONTROLLER_MSG__MSG__DETAIL__PID_TUNE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in msg/PIDTune in the package controller_msg.
typedef struct controller_msg__msg__PIDTune
{
  float kp;
  float ki;
  float kd;
} controller_msg__msg__PIDTune;

// Struct for a sequence of controller_msg__msg__PIDTune.
typedef struct controller_msg__msg__PIDTune__Sequence
{
  controller_msg__msg__PIDTune * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} controller_msg__msg__PIDTune__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // CONTROLLER_MSG__MSG__DETAIL__PID_TUNE__STRUCT_H_
