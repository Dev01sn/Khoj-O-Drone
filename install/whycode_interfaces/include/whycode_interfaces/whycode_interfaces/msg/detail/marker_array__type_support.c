// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from whycode_interfaces:msg/MarkerArray.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "whycode_interfaces/msg/detail/marker_array__rosidl_typesupport_introspection_c.h"
#include "whycode_interfaces/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "whycode_interfaces/msg/detail/marker_array__functions.h"
#include "whycode_interfaces/msg/detail/marker_array__struct.h"


// Include directives for member types
// Member `header`
#include "std_msgs/msg/header.h"
// Member `header`
#include "std_msgs/msg/detail/header__rosidl_typesupport_introspection_c.h"
// Member `markers`
#include "whycode_interfaces/msg/marker.h"
// Member `markers`
#include "whycode_interfaces/msg/detail/marker__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__MarkerArray_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  whycode_interfaces__msg__MarkerArray__init(message_memory);
}

void whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__MarkerArray_fini_function(void * message_memory)
{
  whycode_interfaces__msg__MarkerArray__fini(message_memory);
}

size_t whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__size_function__MarkerArray__markers(
  const void * untyped_member)
{
  const whycode_interfaces__msg__Marker__Sequence * member =
    (const whycode_interfaces__msg__Marker__Sequence *)(untyped_member);
  return member->size;
}

const void * whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__get_const_function__MarkerArray__markers(
  const void * untyped_member, size_t index)
{
  const whycode_interfaces__msg__Marker__Sequence * member =
    (const whycode_interfaces__msg__Marker__Sequence *)(untyped_member);
  return &member->data[index];
}

void * whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__get_function__MarkerArray__markers(
  void * untyped_member, size_t index)
{
  whycode_interfaces__msg__Marker__Sequence * member =
    (whycode_interfaces__msg__Marker__Sequence *)(untyped_member);
  return &member->data[index];
}

void whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__fetch_function__MarkerArray__markers(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const whycode_interfaces__msg__Marker * item =
    ((const whycode_interfaces__msg__Marker *)
    whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__get_const_function__MarkerArray__markers(untyped_member, index));
  whycode_interfaces__msg__Marker * value =
    (whycode_interfaces__msg__Marker *)(untyped_value);
  *value = *item;
}

void whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__assign_function__MarkerArray__markers(
  void * untyped_member, size_t index, const void * untyped_value)
{
  whycode_interfaces__msg__Marker * item =
    ((whycode_interfaces__msg__Marker *)
    whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__get_function__MarkerArray__markers(untyped_member, index));
  const whycode_interfaces__msg__Marker * value =
    (const whycode_interfaces__msg__Marker *)(untyped_value);
  *item = *value;
}

bool whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__resize_function__MarkerArray__markers(
  void * untyped_member, size_t size)
{
  whycode_interfaces__msg__Marker__Sequence * member =
    (whycode_interfaces__msg__Marker__Sequence *)(untyped_member);
  whycode_interfaces__msg__Marker__Sequence__fini(member);
  return whycode_interfaces__msg__Marker__Sequence__init(member, size);
}

static rosidl_typesupport_introspection_c__MessageMember whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__MarkerArray_message_member_array[2] = {
  {
    "header",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(whycode_interfaces__msg__MarkerArray, header),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "markers",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(whycode_interfaces__msg__MarkerArray, markers),  // bytes offset in struct
    NULL,  // default value
    whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__size_function__MarkerArray__markers,  // size() function pointer
    whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__get_const_function__MarkerArray__markers,  // get_const(index) function pointer
    whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__get_function__MarkerArray__markers,  // get(index) function pointer
    whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__fetch_function__MarkerArray__markers,  // fetch(index, &value) function pointer
    whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__assign_function__MarkerArray__markers,  // assign(index, value) function pointer
    whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__resize_function__MarkerArray__markers  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__MarkerArray_message_members = {
  "whycode_interfaces__msg",  // message namespace
  "MarkerArray",  // message name
  2,  // number of fields
  sizeof(whycode_interfaces__msg__MarkerArray),
  whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__MarkerArray_message_member_array,  // message members
  whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__MarkerArray_init_function,  // function to initialize message memory (memory has to be allocated)
  whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__MarkerArray_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__MarkerArray_message_type_support_handle = {
  0,
  &whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__MarkerArray_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_whycode_interfaces
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, whycode_interfaces, msg, MarkerArray)() {
  whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__MarkerArray_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, std_msgs, msg, Header)();
  whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__MarkerArray_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, whycode_interfaces, msg, Marker)();
  if (!whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__MarkerArray_message_type_support_handle.typesupport_identifier) {
    whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__MarkerArray_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &whycode_interfaces__msg__MarkerArray__rosidl_typesupport_introspection_c__MarkerArray_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
