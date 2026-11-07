// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from error_msg:msg/Error.idl
// generated code does not contain a copyright notice
#include "error_msg/msg/detail/error__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


bool
error_msg__msg__Error__init(error_msg__msg__Error * msg)
{
  if (!msg) {
    return false;
  }
  // roll_error
  // pitch_error
  // throttle_error
  // yaw_error
  return true;
}

void
error_msg__msg__Error__fini(error_msg__msg__Error * msg)
{
  if (!msg) {
    return;
  }
  // roll_error
  // pitch_error
  // throttle_error
  // yaw_error
}

bool
error_msg__msg__Error__are_equal(const error_msg__msg__Error * lhs, const error_msg__msg__Error * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // roll_error
  if (lhs->roll_error != rhs->roll_error) {
    return false;
  }
  // pitch_error
  if (lhs->pitch_error != rhs->pitch_error) {
    return false;
  }
  // throttle_error
  if (lhs->throttle_error != rhs->throttle_error) {
    return false;
  }
  // yaw_error
  if (lhs->yaw_error != rhs->yaw_error) {
    return false;
  }
  return true;
}

bool
error_msg__msg__Error__copy(
  const error_msg__msg__Error * input,
  error_msg__msg__Error * output)
{
  if (!input || !output) {
    return false;
  }
  // roll_error
  output->roll_error = input->roll_error;
  // pitch_error
  output->pitch_error = input->pitch_error;
  // throttle_error
  output->throttle_error = input->throttle_error;
  // yaw_error
  output->yaw_error = input->yaw_error;
  return true;
}

error_msg__msg__Error *
error_msg__msg__Error__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  error_msg__msg__Error * msg = (error_msg__msg__Error *)allocator.allocate(sizeof(error_msg__msg__Error), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(error_msg__msg__Error));
  bool success = error_msg__msg__Error__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
error_msg__msg__Error__destroy(error_msg__msg__Error * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    error_msg__msg__Error__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
error_msg__msg__Error__Sequence__init(error_msg__msg__Error__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  error_msg__msg__Error * data = NULL;

  if (size) {
    data = (error_msg__msg__Error *)allocator.zero_allocate(size, sizeof(error_msg__msg__Error), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = error_msg__msg__Error__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        error_msg__msg__Error__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
error_msg__msg__Error__Sequence__fini(error_msg__msg__Error__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      error_msg__msg__Error__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

error_msg__msg__Error__Sequence *
error_msg__msg__Error__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  error_msg__msg__Error__Sequence * array = (error_msg__msg__Error__Sequence *)allocator.allocate(sizeof(error_msg__msg__Error__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = error_msg__msg__Error__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
error_msg__msg__Error__Sequence__destroy(error_msg__msg__Error__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    error_msg__msg__Error__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
error_msg__msg__Error__Sequence__are_equal(const error_msg__msg__Error__Sequence * lhs, const error_msg__msg__Error__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!error_msg__msg__Error__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
error_msg__msg__Error__Sequence__copy(
  const error_msg__msg__Error__Sequence * input,
  error_msg__msg__Error__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(error_msg__msg__Error);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    error_msg__msg__Error * data =
      (error_msg__msg__Error *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!error_msg__msg__Error__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          error_msg__msg__Error__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!error_msg__msg__Error__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
