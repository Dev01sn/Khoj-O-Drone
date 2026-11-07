// generated from rosidl_typesupport_cpp/resource/idl__type_support.cpp.em
// with input from whycode_interfaces:srv/SetCalibMethod.idl
// generated code does not contain a copyright notice

#include "cstddef"
#include "rosidl_runtime_c/message_type_support_struct.h"
#include "whycode_interfaces/srv/detail/set_calib_method__struct.hpp"
#include "rosidl_typesupport_cpp/identifier.hpp"
#include "rosidl_typesupport_cpp/message_type_support.hpp"
#include "rosidl_typesupport_c/type_support_map.h"
#include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
#include "rosidl_typesupport_cpp/visibility_control.h"
#include "rosidl_typesupport_interface/macros.h"

namespace whycode_interfaces
{

namespace srv
{

namespace rosidl_typesupport_cpp
{

typedef struct _SetCalibMethod_Request_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SetCalibMethod_Request_type_support_ids_t;

static const _SetCalibMethod_Request_type_support_ids_t _SetCalibMethod_Request_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SetCalibMethod_Request_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SetCalibMethod_Request_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SetCalibMethod_Request_type_support_symbol_names_t _SetCalibMethod_Request_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, whycode_interfaces, srv, SetCalibMethod_Request)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, whycode_interfaces, srv, SetCalibMethod_Request)),
  }
};

typedef struct _SetCalibMethod_Request_type_support_data_t
{
  void * data[2];
} _SetCalibMethod_Request_type_support_data_t;

static _SetCalibMethod_Request_type_support_data_t _SetCalibMethod_Request_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SetCalibMethod_Request_message_typesupport_map = {
  2,
  "whycode_interfaces",
  &_SetCalibMethod_Request_message_typesupport_ids.typesupport_identifier[0],
  &_SetCalibMethod_Request_message_typesupport_symbol_names.symbol_name[0],
  &_SetCalibMethod_Request_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t SetCalibMethod_Request_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SetCalibMethod_Request_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace srv

}  // namespace whycode_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<whycode_interfaces::srv::SetCalibMethod_Request>()
{
  return &::whycode_interfaces::srv::rosidl_typesupport_cpp::SetCalibMethod_Request_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, whycode_interfaces, srv, SetCalibMethod_Request)() {
  return get_message_type_support_handle<whycode_interfaces::srv::SetCalibMethod_Request>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "whycode_interfaces/srv/detail/set_calib_method__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace whycode_interfaces
{

namespace srv
{

namespace rosidl_typesupport_cpp
{

typedef struct _SetCalibMethod_Response_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SetCalibMethod_Response_type_support_ids_t;

static const _SetCalibMethod_Response_type_support_ids_t _SetCalibMethod_Response_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SetCalibMethod_Response_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SetCalibMethod_Response_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SetCalibMethod_Response_type_support_symbol_names_t _SetCalibMethod_Response_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, whycode_interfaces, srv, SetCalibMethod_Response)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, whycode_interfaces, srv, SetCalibMethod_Response)),
  }
};

typedef struct _SetCalibMethod_Response_type_support_data_t
{
  void * data[2];
} _SetCalibMethod_Response_type_support_data_t;

static _SetCalibMethod_Response_type_support_data_t _SetCalibMethod_Response_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SetCalibMethod_Response_message_typesupport_map = {
  2,
  "whycode_interfaces",
  &_SetCalibMethod_Response_message_typesupport_ids.typesupport_identifier[0],
  &_SetCalibMethod_Response_message_typesupport_symbol_names.symbol_name[0],
  &_SetCalibMethod_Response_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t SetCalibMethod_Response_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SetCalibMethod_Response_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace srv

}  // namespace whycode_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<whycode_interfaces::srv::SetCalibMethod_Response>()
{
  return &::whycode_interfaces::srv::rosidl_typesupport_cpp::SetCalibMethod_Response_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, whycode_interfaces, srv, SetCalibMethod_Response)() {
  return get_message_type_support_handle<whycode_interfaces::srv::SetCalibMethod_Response>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "whycode_interfaces/srv/detail/set_calib_method__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
#include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
#include "rosidl_typesupport_cpp/service_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace whycode_interfaces
{

namespace srv
{

namespace rosidl_typesupport_cpp
{

typedef struct _SetCalibMethod_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SetCalibMethod_type_support_ids_t;

static const _SetCalibMethod_type_support_ids_t _SetCalibMethod_service_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SetCalibMethod_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SetCalibMethod_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SetCalibMethod_type_support_symbol_names_t _SetCalibMethod_service_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, whycode_interfaces, srv, SetCalibMethod)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, whycode_interfaces, srv, SetCalibMethod)),
  }
};

typedef struct _SetCalibMethod_type_support_data_t
{
  void * data[2];
} _SetCalibMethod_type_support_data_t;

static _SetCalibMethod_type_support_data_t _SetCalibMethod_service_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SetCalibMethod_service_typesupport_map = {
  2,
  "whycode_interfaces",
  &_SetCalibMethod_service_typesupport_ids.typesupport_identifier[0],
  &_SetCalibMethod_service_typesupport_symbol_names.symbol_name[0],
  &_SetCalibMethod_service_typesupport_data.data[0],
};

static const rosidl_service_type_support_t SetCalibMethod_service_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SetCalibMethod_service_typesupport_map),
  ::rosidl_typesupport_cpp::get_service_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace srv

}  // namespace whycode_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
get_service_type_support_handle<whycode_interfaces::srv::SetCalibMethod>()
{
  return &::whycode_interfaces::srv::rosidl_typesupport_cpp::SetCalibMethod_service_type_support_handle;
}

}  // namespace rosidl_typesupport_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_cpp, whycode_interfaces, srv, SetCalibMethod)() {
  return ::rosidl_typesupport_cpp::get_service_type_support_handle<whycode_interfaces::srv::SetCalibMethod>();
}

#ifdef __cplusplus
}
#endif
