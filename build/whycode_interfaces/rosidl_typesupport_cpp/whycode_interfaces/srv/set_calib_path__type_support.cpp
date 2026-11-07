// generated from rosidl_typesupport_cpp/resource/idl__type_support.cpp.em
// with input from whycode_interfaces:srv/SetCalibPath.idl
// generated code does not contain a copyright notice

#include "cstddef"
#include "rosidl_runtime_c/message_type_support_struct.h"
#include "whycode_interfaces/srv/detail/set_calib_path__struct.hpp"
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

typedef struct _SetCalibPath_Request_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SetCalibPath_Request_type_support_ids_t;

static const _SetCalibPath_Request_type_support_ids_t _SetCalibPath_Request_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SetCalibPath_Request_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SetCalibPath_Request_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SetCalibPath_Request_type_support_symbol_names_t _SetCalibPath_Request_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, whycode_interfaces, srv, SetCalibPath_Request)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, whycode_interfaces, srv, SetCalibPath_Request)),
  }
};

typedef struct _SetCalibPath_Request_type_support_data_t
{
  void * data[2];
} _SetCalibPath_Request_type_support_data_t;

static _SetCalibPath_Request_type_support_data_t _SetCalibPath_Request_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SetCalibPath_Request_message_typesupport_map = {
  2,
  "whycode_interfaces",
  &_SetCalibPath_Request_message_typesupport_ids.typesupport_identifier[0],
  &_SetCalibPath_Request_message_typesupport_symbol_names.symbol_name[0],
  &_SetCalibPath_Request_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t SetCalibPath_Request_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SetCalibPath_Request_message_typesupport_map),
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
get_message_type_support_handle<whycode_interfaces::srv::SetCalibPath_Request>()
{
  return &::whycode_interfaces::srv::rosidl_typesupport_cpp::SetCalibPath_Request_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, whycode_interfaces, srv, SetCalibPath_Request)() {
  return get_message_type_support_handle<whycode_interfaces::srv::SetCalibPath_Request>();
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
// #include "whycode_interfaces/srv/detail/set_calib_path__struct.hpp"
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

typedef struct _SetCalibPath_Response_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SetCalibPath_Response_type_support_ids_t;

static const _SetCalibPath_Response_type_support_ids_t _SetCalibPath_Response_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SetCalibPath_Response_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SetCalibPath_Response_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SetCalibPath_Response_type_support_symbol_names_t _SetCalibPath_Response_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, whycode_interfaces, srv, SetCalibPath_Response)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, whycode_interfaces, srv, SetCalibPath_Response)),
  }
};

typedef struct _SetCalibPath_Response_type_support_data_t
{
  void * data[2];
} _SetCalibPath_Response_type_support_data_t;

static _SetCalibPath_Response_type_support_data_t _SetCalibPath_Response_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SetCalibPath_Response_message_typesupport_map = {
  2,
  "whycode_interfaces",
  &_SetCalibPath_Response_message_typesupport_ids.typesupport_identifier[0],
  &_SetCalibPath_Response_message_typesupport_symbol_names.symbol_name[0],
  &_SetCalibPath_Response_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t SetCalibPath_Response_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SetCalibPath_Response_message_typesupport_map),
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
get_message_type_support_handle<whycode_interfaces::srv::SetCalibPath_Response>()
{
  return &::whycode_interfaces::srv::rosidl_typesupport_cpp::SetCalibPath_Response_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, whycode_interfaces, srv, SetCalibPath_Response)() {
  return get_message_type_support_handle<whycode_interfaces::srv::SetCalibPath_Response>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "whycode_interfaces/srv/detail/set_calib_path__struct.hpp"
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

typedef struct _SetCalibPath_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _SetCalibPath_type_support_ids_t;

static const _SetCalibPath_type_support_ids_t _SetCalibPath_service_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _SetCalibPath_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _SetCalibPath_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _SetCalibPath_type_support_symbol_names_t _SetCalibPath_service_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, whycode_interfaces, srv, SetCalibPath)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, whycode_interfaces, srv, SetCalibPath)),
  }
};

typedef struct _SetCalibPath_type_support_data_t
{
  void * data[2];
} _SetCalibPath_type_support_data_t;

static _SetCalibPath_type_support_data_t _SetCalibPath_service_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _SetCalibPath_service_typesupport_map = {
  2,
  "whycode_interfaces",
  &_SetCalibPath_service_typesupport_ids.typesupport_identifier[0],
  &_SetCalibPath_service_typesupport_symbol_names.symbol_name[0],
  &_SetCalibPath_service_typesupport_data.data[0],
};

static const rosidl_service_type_support_t SetCalibPath_service_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_SetCalibPath_service_typesupport_map),
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
get_service_type_support_handle<whycode_interfaces::srv::SetCalibPath>()
{
  return &::whycode_interfaces::srv::rosidl_typesupport_cpp::SetCalibPath_service_type_support_handle;
}

}  // namespace rosidl_typesupport_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_cpp, whycode_interfaces, srv, SetCalibPath)() {
  return ::rosidl_typesupport_cpp::get_service_type_support_handle<whycode_interfaces::srv::SetCalibPath>();
}

#ifdef __cplusplus
}
#endif
