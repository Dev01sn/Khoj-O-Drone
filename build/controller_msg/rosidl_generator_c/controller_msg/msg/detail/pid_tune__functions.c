// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from controller_msg:msg/PIDTune.idl
// generated code does not contain a copyright notice
#include "controller_msg/msg/detail/pid_tune__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


bool
controller_msg__msg__PIDTune__init(controller_msg__msg__PIDTune * msg)
{
  if (!msg) {
    return false;
  }
  // kp
  // ki
  // kd
  return true;
}

void
controller_msg__msg__PIDTune__fini(controller_msg__msg__PIDTune * msg)
{
  if (!msg) {
    return;
  }
  // kp
  // ki
  // kd
}

bool
controller_msg__msg__PIDTune__are_equal(const controller_msg__msg__PIDTune * lhs, const controller_msg__msg__PIDTune * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // kp
  if (lhs->kp != rhs->kp) {
    return false;
  }
  // ki
  if (lhs->ki != rhs->ki) {
    return false;
  }
  // kd
  if (lhs->kd != rhs->kd) {
    return false;
  }
  return true;
}

bool
controller_msg__msg__PIDTune__copy(
  const controller_msg__msg__PIDTune * input,
  controller_msg__msg__PIDTune * output)
{
  if (!input || !output) {
    return false;
  }
  // kp
  output->kp = input->kp;
  // ki
  output->ki = input->ki;
  // kd
  output->kd = input->kd;
  return true;
}

controller_msg__msg__PIDTune *
controller_msg__msg__PIDTune__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  controller_msg__msg__PIDTune * msg = (controller_msg__msg__PIDTune *)allocator.allocate(sizeof(controller_msg__msg__PIDTune), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(controller_msg__msg__PIDTune));
  bool success = controller_msg__msg__PIDTune__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
controller_msg__msg__PIDTune__destroy(controller_msg__msg__PIDTune * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    controller_msg__msg__PIDTune__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
controller_msg__msg__PIDTune__Sequence__init(controller_msg__msg__PIDTune__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  controller_msg__msg__PIDTune * data = NULL;

  if (size) {
    data = (controller_msg__msg__PIDTune *)allocator.zero_allocate(size, sizeof(controller_msg__msg__PIDTune), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = controller_msg__msg__PIDTune__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        controller_msg__msg__PIDTune__fini(&data[i - 1]);
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
controller_msg__msg__PIDTune__Sequence__fini(controller_msg__msg__PIDTune__Sequence * array)
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
      controller_msg__msg__PIDTune__fini(&array->data[i]);
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

controller_msg__msg__PIDTune__Sequence *
controller_msg__msg__PIDTune__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  controller_msg__msg__PIDTune__Sequence * array = (controller_msg__msg__PIDTune__Sequence *)allocator.allocate(sizeof(controller_msg__msg__PIDTune__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = controller_msg__msg__PIDTune__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
controller_msg__msg__PIDTune__Sequence__destroy(controller_msg__msg__PIDTune__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    controller_msg__msg__PIDTune__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
controller_msg__msg__PIDTune__Sequence__are_equal(const controller_msg__msg__PIDTune__Sequence * lhs, const controller_msg__msg__PIDTune__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!controller_msg__msg__PIDTune__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
controller_msg__msg__PIDTune__Sequence__copy(
  const controller_msg__msg__PIDTune__Sequence * input,
  controller_msg__msg__PIDTune__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(controller_msg__msg__PIDTune);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    controller_msg__msg__PIDTune * data =
      (controller_msg__msg__PIDTune *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!controller_msg__msg__PIDTune__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          controller_msg__msg__PIDTune__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!controller_msg__msg__PIDTune__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
