// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from whycode_interfaces:srv/SetCalibMethod.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "whycode_interfaces/srv/detail/set_calib_method__rosidl_typesupport_introspection_c.h"
#include "whycode_interfaces/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "whycode_interfaces/srv/detail/set_calib_method__functions.h"
#include "whycode_interfaces/srv/detail/set_calib_method__struct.h"


#ifdef __cplusplus
extern "C"
{
#endif

void whycode_interfaces__srv__SetCalibMethod_Request__rosidl_typesupport_introspection_c__SetCalibMethod_Request_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  whycode_interfaces__srv__SetCalibMethod_Request__init(message_memory);
}

void whycode_interfaces__srv__SetCalibMethod_Request__rosidl_typesupport_introspection_c__SetCalibMethod_Request_fini_function(void * message_memory)
{
  whycode_interfaces__srv__SetCalibMethod_Request__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember whycode_interfaces__srv__SetCalibMethod_Request__rosidl_typesupport_introspection_c__SetCalibMethod_Request_message_member_array[1] = {
  {
    "method",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_INT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(whycode_interfaces__srv__SetCalibMethod_Request, method),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers whycode_interfaces__srv__SetCalibMethod_Request__rosidl_typesupport_introspection_c__SetCalibMethod_Request_message_members = {
  "whycode_interfaces__srv",  // message namespace
  "SetCalibMethod_Request",  // message name
  1,  // number of fields
  sizeof(whycode_interfaces__srv__SetCalibMethod_Request),
  whycode_interfaces__srv__SetCalibMethod_Request__rosidl_typesupport_introspection_c__SetCalibMethod_Request_message_member_array,  // message members
  whycode_interfaces__srv__SetCalibMethod_Request__rosidl_typesupport_introspection_c__SetCalibMethod_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  whycode_interfaces__srv__SetCalibMethod_Request__rosidl_typesupport_introspection_c__SetCalibMethod_Request_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t whycode_interfaces__srv__SetCalibMethod_Request__rosidl_typesupport_introspection_c__SetCalibMethod_Request_message_type_support_handle = {
  0,
  &whycode_interfaces__srv__SetCalibMethod_Request__rosidl_typesupport_introspection_c__SetCalibMethod_Request_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_whycode_interfaces
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, whycode_interfaces, srv, SetCalibMethod_Request)() {
  if (!whycode_interfaces__srv__SetCalibMethod_Request__rosidl_typesupport_introspection_c__SetCalibMethod_Request_message_type_support_handle.typesupport_identifier) {
    whycode_interfaces__srv__SetCalibMethod_Request__rosidl_typesupport_introspection_c__SetCalibMethod_Request_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &whycode_interfaces__srv__SetCalibMethod_Request__rosidl_typesupport_introspection_c__SetCalibMethod_Request_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "whycode_interfaces/srv/detail/set_calib_method__rosidl_typesupport_introspection_c.h"
// already included above
// #include "whycode_interfaces/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "whycode_interfaces/srv/detail/set_calib_method__functions.h"
// already included above
// #include "whycode_interfaces/srv/detail/set_calib_method__struct.h"


// Include directives for member types
// Member `msg`
#include "rosidl_runtime_c/string_functions.h"

#ifdef __cplusplus
extern "C"
{
#endif

void whycode_interfaces__srv__SetCalibMethod_Response__rosidl_typesupport_introspection_c__SetCalibMethod_Response_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  whycode_interfaces__srv__SetCalibMethod_Response__init(message_memory);
}

void whycode_interfaces__srv__SetCalibMethod_Response__rosidl_typesupport_introspection_c__SetCalibMethod_Response_fini_function(void * message_memory)
{
  whycode_interfaces__srv__SetCalibMethod_Response__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember whycode_interfaces__srv__SetCalibMethod_Response__rosidl_typesupport_introspection_c__SetCalibMethod_Response_message_member_array[2] = {
  {
    "success",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(whycode_interfaces__srv__SetCalibMethod_Response, success),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "msg",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(whycode_interfaces__srv__SetCalibMethod_Response, msg),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers whycode_interfaces__srv__SetCalibMethod_Response__rosidl_typesupport_introspection_c__SetCalibMethod_Response_message_members = {
  "whycode_interfaces__srv",  // message namespace
  "SetCalibMethod_Response",  // message name
  2,  // number of fields
  sizeof(whycode_interfaces__srv__SetCalibMethod_Response),
  whycode_interfaces__srv__SetCalibMethod_Response__rosidl_typesupport_introspection_c__SetCalibMethod_Response_message_member_array,  // message members
  whycode_interfaces__srv__SetCalibMethod_Response__rosidl_typesupport_introspection_c__SetCalibMethod_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  whycode_interfaces__srv__SetCalibMethod_Response__rosidl_typesupport_introspection_c__SetCalibMethod_Response_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t whycode_interfaces__srv__SetCalibMethod_Response__rosidl_typesupport_introspection_c__SetCalibMethod_Response_message_type_support_handle = {
  0,
  &whycode_interfaces__srv__SetCalibMethod_Response__rosidl_typesupport_introspection_c__SetCalibMethod_Response_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_whycode_interfaces
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, whycode_interfaces, srv, SetCalibMethod_Response)() {
  if (!whycode_interfaces__srv__SetCalibMethod_Response__rosidl_typesupport_introspection_c__SetCalibMethod_Response_message_type_support_handle.typesupport_identifier) {
    whycode_interfaces__srv__SetCalibMethod_Response__rosidl_typesupport_introspection_c__SetCalibMethod_Response_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &whycode_interfaces__srv__SetCalibMethod_Response__rosidl_typesupport_introspection_c__SetCalibMethod_Response_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "whycode_interfaces/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "whycode_interfaces/srv/detail/set_calib_method__rosidl_typesupport_introspection_c.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/service_introspection.h"

// this is intentionally not const to allow initialization later to prevent an initialization race
static rosidl_typesupport_introspection_c__ServiceMembers whycode_interfaces__srv__detail__set_calib_method__rosidl_typesupport_introspection_c__SetCalibMethod_service_members = {
  "whycode_interfaces__srv",  // service namespace
  "SetCalibMethod",  // service name
  // these two fields are initialized below on the first access
  NULL,  // request message
  // whycode_interfaces__srv__detail__set_calib_method__rosidl_typesupport_introspection_c__SetCalibMethod_Request_message_type_support_handle,
  NULL  // response message
  // whycode_interfaces__srv__detail__set_calib_method__rosidl_typesupport_introspection_c__SetCalibMethod_Response_message_type_support_handle
};

static rosidl_service_type_support_t whycode_interfaces__srv__detail__set_calib_method__rosidl_typesupport_introspection_c__SetCalibMethod_service_type_support_handle = {
  0,
  &whycode_interfaces__srv__detail__set_calib_method__rosidl_typesupport_introspection_c__SetCalibMethod_service_members,
  get_service_typesupport_handle_function,
};

// Forward declaration of request/response type support functions
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, whycode_interfaces, srv, SetCalibMethod_Request)();

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, whycode_interfaces, srv, SetCalibMethod_Response)();

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_whycode_interfaces
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, whycode_interfaces, srv, SetCalibMethod)() {
  if (!whycode_interfaces__srv__detail__set_calib_method__rosidl_typesupport_introspection_c__SetCalibMethod_service_type_support_handle.typesupport_identifier) {
    whycode_interfaces__srv__detail__set_calib_method__rosidl_typesupport_introspection_c__SetCalibMethod_service_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  rosidl_typesupport_introspection_c__ServiceMembers * service_members =
    (rosidl_typesupport_introspection_c__ServiceMembers *)whycode_interfaces__srv__detail__set_calib_method__rosidl_typesupport_introspection_c__SetCalibMethod_service_type_support_handle.data;

  if (!service_members->request_members_) {
    service_members->request_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, whycode_interfaces, srv, SetCalibMethod_Request)()->data;
  }
  if (!service_members->response_members_) {
    service_members->response_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, whycode_interfaces, srv, SetCalibMethod_Response)()->data;
  }

  return &whycode_interfaces__srv__detail__set_calib_method__rosidl_typesupport_introspection_c__SetCalibMethod_service_type_support_handle;
}
