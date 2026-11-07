// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from whycode_interfaces:msg/Marker.idl
// generated code does not contain a copyright notice
#include "whycode_interfaces/msg/detail/marker__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `position`
#include "geometry_msgs/msg/detail/pose__functions.h"
// Member `rotation`
#include "geometry_msgs/msg/detail/vector3__functions.h"

bool
whycode_interfaces__msg__Marker__init(whycode_interfaces__msg__Marker * msg)
{
  if (!msg) {
    return false;
  }
  // id
  // size
  // u
  // v
  // angle
  // position
  if (!geometry_msgs__msg__Pose__init(&msg->position)) {
    whycode_interfaces__msg__Marker__fini(msg);
    return false;
  }
  // rotation
  if (!geometry_msgs__msg__Vector3__init(&msg->rotation)) {
    whycode_interfaces__msg__Marker__fini(msg);
    return false;
  }
  return true;
}

void
whycode_interfaces__msg__Marker__fini(whycode_interfaces__msg__Marker * msg)
{
  if (!msg) {
    return;
  }
  // id
  // size
  // u
  // v
  // angle
  // position
  geometry_msgs__msg__Pose__fini(&msg->position);
  // rotation
  geometry_msgs__msg__Vector3__fini(&msg->rotation);
}

bool
whycode_interfaces__msg__Marker__are_equal(const whycode_interfaces__msg__Marker * lhs, const whycode_interfaces__msg__Marker * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // id
  if (lhs->id != rhs->id) {
    return false;
  }
  // size
  if (lhs->size != rhs->size) {
    return false;
  }
  // u
  if (lhs->u != rhs->u) {
    return false;
  }
  // v
  if (lhs->v != rhs->v) {
    return false;
  }
  // angle
  if (lhs->angle != rhs->angle) {
    return false;
  }
  // position
  if (!geometry_msgs__msg__Pose__are_equal(
      &(lhs->position), &(rhs->position)))
  {
    return false;
  }
  // rotation
  if (!geometry_msgs__msg__Vector3__are_equal(
      &(lhs->rotation), &(rhs->rotation)))
  {
    return false;
  }
  return true;
}

bool
whycode_interfaces__msg__Marker__copy(
  const whycode_interfaces__msg__Marker * input,
  whycode_interfaces__msg__Marker * output)
{
  if (!input || !output) {
    return false;
  }
  // id
  output->id = input->id;
  // size
  output->size = input->size;
  // u
  output->u = input->u;
  // v
  output->v = input->v;
  // angle
  output->angle = input->angle;
  // position
  if (!geometry_msgs__msg__Pose__copy(
      &(input->position), &(output->position)))
  {
    return false;
  }
  // rotation
  if (!geometry_msgs__msg__Vector3__copy(
      &(input->rotation), &(output->rotation)))
  {
    return false;
  }
  return true;
}

whycode_interfaces__msg__Marker *
whycode_interfaces__msg__Marker__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  whycode_interfaces__msg__Marker * msg = (whycode_interfaces__msg__Marker *)allocator.allocate(sizeof(whycode_interfaces__msg__Marker), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(whycode_interfaces__msg__Marker));
  bool success = whycode_interfaces__msg__Marker__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
whycode_interfaces__msg__Marker__destroy(whycode_interfaces__msg__Marker * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    whycode_interfaces__msg__Marker__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
whycode_interfaces__msg__Marker__Sequence__init(whycode_interfaces__msg__Marker__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  whycode_interfaces__msg__Marker * data = NULL;

  if (size) {
    data = (whycode_interfaces__msg__Marker *)allocator.zero_allocate(size, sizeof(whycode_interfaces__msg__Marker), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = whycode_interfaces__msg__Marker__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        whycode_interfaces__msg__Marker__fini(&data[i - 1]);
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
whycode_interfaces__msg__Marker__Sequence__fini(whycode_interfaces__msg__Marker__Sequence * array)
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
      whycode_interfaces__msg__Marker__fini(&array->data[i]);
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

whycode_interfaces__msg__Marker__Sequence *
whycode_interfaces__msg__Marker__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  whycode_interfaces__msg__Marker__Sequence * array = (whycode_interfaces__msg__Marker__Sequence *)allocator.allocate(sizeof(whycode_interfaces__msg__Marker__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = whycode_interfaces__msg__Marker__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
whycode_interfaces__msg__Marker__Sequence__destroy(whycode_interfaces__msg__Marker__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    whycode_interfaces__msg__Marker__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
whycode_interfaces__msg__Marker__Sequence__are_equal(const whycode_interfaces__msg__Marker__Sequence * lhs, const whycode_interfaces__msg__Marker__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!whycode_interfaces__msg__Marker__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
whycode_interfaces__msg__Marker__Sequence__copy(
  const whycode_interfaces__msg__Marker__Sequence * input,
  whycode_interfaces__msg__Marker__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(whycode_interfaces__msg__Marker);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    whycode_interfaces__msg__Marker * data =
      (whycode_interfaces__msg__Marker *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!whycode_interfaces__msg__Marker__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          whycode_interfaces__msg__Marker__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!whycode_interfaces__msg__Marker__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
