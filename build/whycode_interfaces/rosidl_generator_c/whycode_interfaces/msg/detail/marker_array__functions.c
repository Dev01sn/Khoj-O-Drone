// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from whycode_interfaces:msg/MarkerArray.idl
// generated code does not contain a copyright notice
#include "whycode_interfaces/msg/detail/marker_array__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `header`
#include "std_msgs/msg/detail/header__functions.h"
// Member `markers`
#include "whycode_interfaces/msg/detail/marker__functions.h"

bool
whycode_interfaces__msg__MarkerArray__init(whycode_interfaces__msg__MarkerArray * msg)
{
  if (!msg) {
    return false;
  }
  // header
  if (!std_msgs__msg__Header__init(&msg->header)) {
    whycode_interfaces__msg__MarkerArray__fini(msg);
    return false;
  }
  // markers
  if (!whycode_interfaces__msg__Marker__Sequence__init(&msg->markers, 0)) {
    whycode_interfaces__msg__MarkerArray__fini(msg);
    return false;
  }
  return true;
}

void
whycode_interfaces__msg__MarkerArray__fini(whycode_interfaces__msg__MarkerArray * msg)
{
  if (!msg) {
    return;
  }
  // header
  std_msgs__msg__Header__fini(&msg->header);
  // markers
  whycode_interfaces__msg__Marker__Sequence__fini(&msg->markers);
}

bool
whycode_interfaces__msg__MarkerArray__are_equal(const whycode_interfaces__msg__MarkerArray * lhs, const whycode_interfaces__msg__MarkerArray * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // header
  if (!std_msgs__msg__Header__are_equal(
      &(lhs->header), &(rhs->header)))
  {
    return false;
  }
  // markers
  if (!whycode_interfaces__msg__Marker__Sequence__are_equal(
      &(lhs->markers), &(rhs->markers)))
  {
    return false;
  }
  return true;
}

bool
whycode_interfaces__msg__MarkerArray__copy(
  const whycode_interfaces__msg__MarkerArray * input,
  whycode_interfaces__msg__MarkerArray * output)
{
  if (!input || !output) {
    return false;
  }
  // header
  if (!std_msgs__msg__Header__copy(
      &(input->header), &(output->header)))
  {
    return false;
  }
  // markers
  if (!whycode_interfaces__msg__Marker__Sequence__copy(
      &(input->markers), &(output->markers)))
  {
    return false;
  }
  return true;
}

whycode_interfaces__msg__MarkerArray *
whycode_interfaces__msg__MarkerArray__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  whycode_interfaces__msg__MarkerArray * msg = (whycode_interfaces__msg__MarkerArray *)allocator.allocate(sizeof(whycode_interfaces__msg__MarkerArray), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(whycode_interfaces__msg__MarkerArray));
  bool success = whycode_interfaces__msg__MarkerArray__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
whycode_interfaces__msg__MarkerArray__destroy(whycode_interfaces__msg__MarkerArray * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    whycode_interfaces__msg__MarkerArray__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
whycode_interfaces__msg__MarkerArray__Sequence__init(whycode_interfaces__msg__MarkerArray__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  whycode_interfaces__msg__MarkerArray * data = NULL;

  if (size) {
    data = (whycode_interfaces__msg__MarkerArray *)allocator.zero_allocate(size, sizeof(whycode_interfaces__msg__MarkerArray), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = whycode_interfaces__msg__MarkerArray__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        whycode_interfaces__msg__MarkerArray__fini(&data[i - 1]);
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
whycode_interfaces__msg__MarkerArray__Sequence__fini(whycode_interfaces__msg__MarkerArray__Sequence * array)
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
      whycode_interfaces__msg__MarkerArray__fini(&array->data[i]);
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

whycode_interfaces__msg__MarkerArray__Sequence *
whycode_interfaces__msg__MarkerArray__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  whycode_interfaces__msg__MarkerArray__Sequence * array = (whycode_interfaces__msg__MarkerArray__Sequence *)allocator.allocate(sizeof(whycode_interfaces__msg__MarkerArray__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = whycode_interfaces__msg__MarkerArray__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
whycode_interfaces__msg__MarkerArray__Sequence__destroy(whycode_interfaces__msg__MarkerArray__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    whycode_interfaces__msg__MarkerArray__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
whycode_interfaces__msg__MarkerArray__Sequence__are_equal(const whycode_interfaces__msg__MarkerArray__Sequence * lhs, const whycode_interfaces__msg__MarkerArray__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!whycode_interfaces__msg__MarkerArray__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
whycode_interfaces__msg__MarkerArray__Sequence__copy(
  const whycode_interfaces__msg__MarkerArray__Sequence * input,
  whycode_interfaces__msg__MarkerArray__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(whycode_interfaces__msg__MarkerArray);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    whycode_interfaces__msg__MarkerArray * data =
      (whycode_interfaces__msg__MarkerArray *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!whycode_interfaces__msg__MarkerArray__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          whycode_interfaces__msg__MarkerArray__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!whycode_interfaces__msg__MarkerArray__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
