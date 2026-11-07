// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from whycode_interfaces:srv/SetCalibPath.idl
// generated code does not contain a copyright notice
#include "whycode_interfaces/srv/detail/set_calib_path__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"

// Include directives for member types
// Member `action`
// Member `path`
#include "rosidl_runtime_c/string_functions.h"

bool
whycode_interfaces__srv__SetCalibPath_Request__init(whycode_interfaces__srv__SetCalibPath_Request * msg)
{
  if (!msg) {
    return false;
  }
  // action
  if (!rosidl_runtime_c__String__init(&msg->action)) {
    whycode_interfaces__srv__SetCalibPath_Request__fini(msg);
    return false;
  }
  // path
  if (!rosidl_runtime_c__String__init(&msg->path)) {
    whycode_interfaces__srv__SetCalibPath_Request__fini(msg);
    return false;
  }
  return true;
}

void
whycode_interfaces__srv__SetCalibPath_Request__fini(whycode_interfaces__srv__SetCalibPath_Request * msg)
{
  if (!msg) {
    return;
  }
  // action
  rosidl_runtime_c__String__fini(&msg->action);
  // path
  rosidl_runtime_c__String__fini(&msg->path);
}

bool
whycode_interfaces__srv__SetCalibPath_Request__are_equal(const whycode_interfaces__srv__SetCalibPath_Request * lhs, const whycode_interfaces__srv__SetCalibPath_Request * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // action
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->action), &(rhs->action)))
  {
    return false;
  }
  // path
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->path), &(rhs->path)))
  {
    return false;
  }
  return true;
}

bool
whycode_interfaces__srv__SetCalibPath_Request__copy(
  const whycode_interfaces__srv__SetCalibPath_Request * input,
  whycode_interfaces__srv__SetCalibPath_Request * output)
{
  if (!input || !output) {
    return false;
  }
  // action
  if (!rosidl_runtime_c__String__copy(
      &(input->action), &(output->action)))
  {
    return false;
  }
  // path
  if (!rosidl_runtime_c__String__copy(
      &(input->path), &(output->path)))
  {
    return false;
  }
  return true;
}

whycode_interfaces__srv__SetCalibPath_Request *
whycode_interfaces__srv__SetCalibPath_Request__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  whycode_interfaces__srv__SetCalibPath_Request * msg = (whycode_interfaces__srv__SetCalibPath_Request *)allocator.allocate(sizeof(whycode_interfaces__srv__SetCalibPath_Request), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(whycode_interfaces__srv__SetCalibPath_Request));
  bool success = whycode_interfaces__srv__SetCalibPath_Request__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
whycode_interfaces__srv__SetCalibPath_Request__destroy(whycode_interfaces__srv__SetCalibPath_Request * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    whycode_interfaces__srv__SetCalibPath_Request__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
whycode_interfaces__srv__SetCalibPath_Request__Sequence__init(whycode_interfaces__srv__SetCalibPath_Request__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  whycode_interfaces__srv__SetCalibPath_Request * data = NULL;

  if (size) {
    data = (whycode_interfaces__srv__SetCalibPath_Request *)allocator.zero_allocate(size, sizeof(whycode_interfaces__srv__SetCalibPath_Request), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = whycode_interfaces__srv__SetCalibPath_Request__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        whycode_interfaces__srv__SetCalibPath_Request__fini(&data[i - 1]);
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
whycode_interfaces__srv__SetCalibPath_Request__Sequence__fini(whycode_interfaces__srv__SetCalibPath_Request__Sequence * array)
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
      whycode_interfaces__srv__SetCalibPath_Request__fini(&array->data[i]);
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

whycode_interfaces__srv__SetCalibPath_Request__Sequence *
whycode_interfaces__srv__SetCalibPath_Request__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  whycode_interfaces__srv__SetCalibPath_Request__Sequence * array = (whycode_interfaces__srv__SetCalibPath_Request__Sequence *)allocator.allocate(sizeof(whycode_interfaces__srv__SetCalibPath_Request__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = whycode_interfaces__srv__SetCalibPath_Request__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
whycode_interfaces__srv__SetCalibPath_Request__Sequence__destroy(whycode_interfaces__srv__SetCalibPath_Request__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    whycode_interfaces__srv__SetCalibPath_Request__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
whycode_interfaces__srv__SetCalibPath_Request__Sequence__are_equal(const whycode_interfaces__srv__SetCalibPath_Request__Sequence * lhs, const whycode_interfaces__srv__SetCalibPath_Request__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!whycode_interfaces__srv__SetCalibPath_Request__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
whycode_interfaces__srv__SetCalibPath_Request__Sequence__copy(
  const whycode_interfaces__srv__SetCalibPath_Request__Sequence * input,
  whycode_interfaces__srv__SetCalibPath_Request__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(whycode_interfaces__srv__SetCalibPath_Request);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    whycode_interfaces__srv__SetCalibPath_Request * data =
      (whycode_interfaces__srv__SetCalibPath_Request *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!whycode_interfaces__srv__SetCalibPath_Request__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          whycode_interfaces__srv__SetCalibPath_Request__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!whycode_interfaces__srv__SetCalibPath_Request__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


// Include directives for member types
// Member `msg`
// already included above
// #include "rosidl_runtime_c/string_functions.h"

bool
whycode_interfaces__srv__SetCalibPath_Response__init(whycode_interfaces__srv__SetCalibPath_Response * msg)
{
  if (!msg) {
    return false;
  }
  // success
  // msg
  if (!rosidl_runtime_c__String__init(&msg->msg)) {
    whycode_interfaces__srv__SetCalibPath_Response__fini(msg);
    return false;
  }
  return true;
}

void
whycode_interfaces__srv__SetCalibPath_Response__fini(whycode_interfaces__srv__SetCalibPath_Response * msg)
{
  if (!msg) {
    return;
  }
  // success
  // msg
  rosidl_runtime_c__String__fini(&msg->msg);
}

bool
whycode_interfaces__srv__SetCalibPath_Response__are_equal(const whycode_interfaces__srv__SetCalibPath_Response * lhs, const whycode_interfaces__srv__SetCalibPath_Response * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // success
  if (lhs->success != rhs->success) {
    return false;
  }
  // msg
  if (!rosidl_runtime_c__String__are_equal(
      &(lhs->msg), &(rhs->msg)))
  {
    return false;
  }
  return true;
}

bool
whycode_interfaces__srv__SetCalibPath_Response__copy(
  const whycode_interfaces__srv__SetCalibPath_Response * input,
  whycode_interfaces__srv__SetCalibPath_Response * output)
{
  if (!input || !output) {
    return false;
  }
  // success
  output->success = input->success;
  // msg
  if (!rosidl_runtime_c__String__copy(
      &(input->msg), &(output->msg)))
  {
    return false;
  }
  return true;
}

whycode_interfaces__srv__SetCalibPath_Response *
whycode_interfaces__srv__SetCalibPath_Response__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  whycode_interfaces__srv__SetCalibPath_Response * msg = (whycode_interfaces__srv__SetCalibPath_Response *)allocator.allocate(sizeof(whycode_interfaces__srv__SetCalibPath_Response), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(whycode_interfaces__srv__SetCalibPath_Response));
  bool success = whycode_interfaces__srv__SetCalibPath_Response__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
whycode_interfaces__srv__SetCalibPath_Response__destroy(whycode_interfaces__srv__SetCalibPath_Response * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    whycode_interfaces__srv__SetCalibPath_Response__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
whycode_interfaces__srv__SetCalibPath_Response__Sequence__init(whycode_interfaces__srv__SetCalibPath_Response__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  whycode_interfaces__srv__SetCalibPath_Response * data = NULL;

  if (size) {
    data = (whycode_interfaces__srv__SetCalibPath_Response *)allocator.zero_allocate(size, sizeof(whycode_interfaces__srv__SetCalibPath_Response), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = whycode_interfaces__srv__SetCalibPath_Response__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        whycode_interfaces__srv__SetCalibPath_Response__fini(&data[i - 1]);
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
whycode_interfaces__srv__SetCalibPath_Response__Sequence__fini(whycode_interfaces__srv__SetCalibPath_Response__Sequence * array)
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
      whycode_interfaces__srv__SetCalibPath_Response__fini(&array->data[i]);
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

whycode_interfaces__srv__SetCalibPath_Response__Sequence *
whycode_interfaces__srv__SetCalibPath_Response__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  whycode_interfaces__srv__SetCalibPath_Response__Sequence * array = (whycode_interfaces__srv__SetCalibPath_Response__Sequence *)allocator.allocate(sizeof(whycode_interfaces__srv__SetCalibPath_Response__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = whycode_interfaces__srv__SetCalibPath_Response__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
whycode_interfaces__srv__SetCalibPath_Response__Sequence__destroy(whycode_interfaces__srv__SetCalibPath_Response__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    whycode_interfaces__srv__SetCalibPath_Response__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
whycode_interfaces__srv__SetCalibPath_Response__Sequence__are_equal(const whycode_interfaces__srv__SetCalibPath_Response__Sequence * lhs, const whycode_interfaces__srv__SetCalibPath_Response__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!whycode_interfaces__srv__SetCalibPath_Response__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
whycode_interfaces__srv__SetCalibPath_Response__Sequence__copy(
  const whycode_interfaces__srv__SetCalibPath_Response__Sequence * input,
  whycode_interfaces__srv__SetCalibPath_Response__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(whycode_interfaces__srv__SetCalibPath_Response);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    whycode_interfaces__srv__SetCalibPath_Response * data =
      (whycode_interfaces__srv__SetCalibPath_Response *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!whycode_interfaces__srv__SetCalibPath_Response__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          whycode_interfaces__srv__SetCalibPath_Response__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!whycode_interfaces__srv__SetCalibPath_Response__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
