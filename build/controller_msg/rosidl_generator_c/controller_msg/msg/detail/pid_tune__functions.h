// generated from rosidl_generator_c/resource/idl__functions.h.em
// with input from controller_msg:msg/PIDTune.idl
// generated code does not contain a copyright notice

#ifndef CONTROLLER_MSG__MSG__DETAIL__PID_TUNE__FUNCTIONS_H_
#define CONTROLLER_MSG__MSG__DETAIL__PID_TUNE__FUNCTIONS_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stdlib.h>

#include "rosidl_runtime_c/visibility_control.h"
#include "controller_msg/msg/rosidl_generator_c__visibility_control.h"

#include "controller_msg/msg/detail/pid_tune__struct.h"

/// Initialize msg/PIDTune message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * controller_msg__msg__PIDTune
 * )) before or use
 * controller_msg__msg__PIDTune__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_controller_msg
bool
controller_msg__msg__PIDTune__init(controller_msg__msg__PIDTune * msg);

/// Finalize msg/PIDTune message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_controller_msg
void
controller_msg__msg__PIDTune__fini(controller_msg__msg__PIDTune * msg);

/// Create msg/PIDTune message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * controller_msg__msg__PIDTune__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_controller_msg
controller_msg__msg__PIDTune *
controller_msg__msg__PIDTune__create();

/// Destroy msg/PIDTune message.
/**
 * It calls
 * controller_msg__msg__PIDTune__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_controller_msg
void
controller_msg__msg__PIDTune__destroy(controller_msg__msg__PIDTune * msg);

/// Check for msg/PIDTune message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_controller_msg
bool
controller_msg__msg__PIDTune__are_equal(const controller_msg__msg__PIDTune * lhs, const controller_msg__msg__PIDTune * rhs);

/// Copy a msg/PIDTune message.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source message pointer.
 * \param[out] output The target message pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer is null
 *   or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_controller_msg
bool
controller_msg__msg__PIDTune__copy(
  const controller_msg__msg__PIDTune * input,
  controller_msg__msg__PIDTune * output);

/// Initialize array of msg/PIDTune messages.
/**
 * It allocates the memory for the number of elements and calls
 * controller_msg__msg__PIDTune__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_controller_msg
bool
controller_msg__msg__PIDTune__Sequence__init(controller_msg__msg__PIDTune__Sequence * array, size_t size);

/// Finalize array of msg/PIDTune messages.
/**
 * It calls
 * controller_msg__msg__PIDTune__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_controller_msg
void
controller_msg__msg__PIDTune__Sequence__fini(controller_msg__msg__PIDTune__Sequence * array);

/// Create array of msg/PIDTune messages.
/**
 * It allocates the memory for the array and calls
 * controller_msg__msg__PIDTune__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_controller_msg
controller_msg__msg__PIDTune__Sequence *
controller_msg__msg__PIDTune__Sequence__create(size_t size);

/// Destroy array of msg/PIDTune messages.
/**
 * It calls
 * controller_msg__msg__PIDTune__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_controller_msg
void
controller_msg__msg__PIDTune__Sequence__destroy(controller_msg__msg__PIDTune__Sequence * array);

/// Check for msg/PIDTune message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_controller_msg
bool
controller_msg__msg__PIDTune__Sequence__are_equal(const controller_msg__msg__PIDTune__Sequence * lhs, const controller_msg__msg__PIDTune__Sequence * rhs);

/// Copy an array of msg/PIDTune messages.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source array pointer.
 * \param[out] output The target array pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer
 *   is null or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_controller_msg
bool
controller_msg__msg__PIDTune__Sequence__copy(
  const controller_msg__msg__PIDTune__Sequence * input,
  controller_msg__msg__PIDTune__Sequence * output);

#ifdef __cplusplus
}
#endif

#endif  // CONTROLLER_MSG__MSG__DETAIL__PID_TUNE__FUNCTIONS_H_
