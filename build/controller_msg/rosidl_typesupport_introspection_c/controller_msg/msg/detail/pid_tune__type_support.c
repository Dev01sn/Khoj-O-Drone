// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from controller_msg:msg/PIDTune.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "controller_msg/msg/detail/pid_tune__rosidl_typesupport_introspection_c.h"
#include "controller_msg/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "controller_msg/msg/detail/pid_tune__functions.h"
#include "controller_msg/msg/detail/pid_tune__struct.h"


#ifdef __cplusplus
extern "C"
{
#endif

void controller_msg__msg__PIDTune__rosidl_typesupport_introspection_c__PIDTune_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  controller_msg__msg__PIDTune__init(message_memory);
}

void controller_msg__msg__PIDTune__rosidl_typesupport_introspection_c__PIDTune_fini_function(void * message_memory)
{
  controller_msg__msg__PIDTune__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember controller_msg__msg__PIDTune__rosidl_typesupport_introspection_c__PIDTune_message_member_array[3] = {
  {
    "kp",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(controller_msg__msg__PIDTune, kp),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "ki",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(controller_msg__msg__PIDTune, ki),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "kd",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(controller_msg__msg__PIDTune, kd),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers controller_msg__msg__PIDTune__rosidl_typesupport_introspection_c__PIDTune_message_members = {
  "controller_msg__msg",  // message namespace
  "PIDTune",  // message name
  3,  // number of fields
  sizeof(controller_msg__msg__PIDTune),
  controller_msg__msg__PIDTune__rosidl_typesupport_introspection_c__PIDTune_message_member_array,  // message members
  controller_msg__msg__PIDTune__rosidl_typesupport_introspection_c__PIDTune_init_function,  // function to initialize message memory (memory has to be allocated)
  controller_msg__msg__PIDTune__rosidl_typesupport_introspection_c__PIDTune_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t controller_msg__msg__PIDTune__rosidl_typesupport_introspection_c__PIDTune_message_type_support_handle = {
  0,
  &controller_msg__msg__PIDTune__rosidl_typesupport_introspection_c__PIDTune_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_controller_msg
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, controller_msg, msg, PIDTune)() {
  if (!controller_msg__msg__PIDTune__rosidl_typesupport_introspection_c__PIDTune_message_type_support_handle.typesupport_identifier) {
    controller_msg__msg__PIDTune__rosidl_typesupport_introspection_c__PIDTune_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &controller_msg__msg__PIDTune__rosidl_typesupport_introspection_c__PIDTune_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
