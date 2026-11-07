// generated from rosidl_typesupport_fastrtps_c/resource/idl__type_support_c.cpp.em
// with input from whycode_interfaces:srv/SetDrawing.idl
// generated code does not contain a copyright notice
#include "whycode_interfaces/srv/detail/set_drawing__rosidl_typesupport_fastrtps_c.h"


#include <cassert>
#include <limits>
#include <string>
#include "rosidl_typesupport_fastrtps_c/identifier.h"
#include "rosidl_typesupport_fastrtps_c/wstring_conversion.hpp"
#include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
#include "whycode_interfaces/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
#include "whycode_interfaces/srv/detail/set_drawing__struct.h"
#include "whycode_interfaces/srv/detail/set_drawing__functions.h"
#include "fastcdr/Cdr.h"

#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-parameter"
# ifdef __clang__
#  pragma clang diagnostic ignored "-Wdeprecated-register"
#  pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
# endif
#endif
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif

// includes and forward declarations of message dependencies and their conversion functions

#if defined(__cplusplus)
extern "C"
{
#endif


// forward declare type support functions


using _SetDrawing_Request__ros_msg_type = whycode_interfaces__srv__SetDrawing_Request;

static bool _SetDrawing_Request__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  const _SetDrawing_Request__ros_msg_type * ros_message = static_cast<const _SetDrawing_Request__ros_msg_type *>(untyped_ros_message);
  // Field name: draw_coords
  {
    cdr << (ros_message->draw_coords ? true : false);
  }

  // Field name: draw_segments
  {
    cdr << (ros_message->draw_segments ? true : false);
  }

  return true;
}

static bool _SetDrawing_Request__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  _SetDrawing_Request__ros_msg_type * ros_message = static_cast<_SetDrawing_Request__ros_msg_type *>(untyped_ros_message);
  // Field name: draw_coords
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->draw_coords = tmp ? true : false;
  }

  // Field name: draw_segments
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->draw_segments = tmp ? true : false;
  }

  return true;
}  // NOLINT(readability/fn_size)

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_whycode_interfaces
size_t get_serialized_size_whycode_interfaces__srv__SetDrawing_Request(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _SetDrawing_Request__ros_msg_type * ros_message = static_cast<const _SetDrawing_Request__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // field.name draw_coords
  {
    size_t item_size = sizeof(ros_message->draw_coords);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }
  // field.name draw_segments
  {
    size_t item_size = sizeof(ros_message->draw_segments);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  return current_alignment - initial_alignment;
}

static uint32_t _SetDrawing_Request__get_serialized_size(const void * untyped_ros_message)
{
  return static_cast<uint32_t>(
    get_serialized_size_whycode_interfaces__srv__SetDrawing_Request(
      untyped_ros_message, 0));
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_whycode_interfaces
size_t max_serialized_size_whycode_interfaces__srv__SetDrawing_Request(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment)
{
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  size_t last_member_size = 0;
  (void)last_member_size;
  (void)padding;
  (void)wchar_size;

  full_bounded = true;
  is_plain = true;

  // member: draw_coords
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }
  // member: draw_segments
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  size_t ret_val = current_alignment - initial_alignment;
  if (is_plain) {
    // All members are plain, and type is not empty.
    // We still need to check that the in-memory alignment
    // is the same as the CDR mandated alignment.
    using DataType = whycode_interfaces__srv__SetDrawing_Request;
    is_plain =
      (
      offsetof(DataType, draw_segments) +
      last_member_size
      ) == ret_val;
  }

  return ret_val;
}

static size_t _SetDrawing_Request__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_whycode_interfaces__srv__SetDrawing_Request(
    full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}


static message_type_support_callbacks_t __callbacks_SetDrawing_Request = {
  "whycode_interfaces::srv",
  "SetDrawing_Request",
  _SetDrawing_Request__cdr_serialize,
  _SetDrawing_Request__cdr_deserialize,
  _SetDrawing_Request__get_serialized_size,
  _SetDrawing_Request__max_serialized_size
};

static rosidl_message_type_support_t _SetDrawing_Request__type_support = {
  rosidl_typesupport_fastrtps_c__identifier,
  &__callbacks_SetDrawing_Request,
  get_message_typesupport_handle_function,
};

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, whycode_interfaces, srv, SetDrawing_Request)() {
  return &_SetDrawing_Request__type_support;
}

#if defined(__cplusplus)
}
#endif

// already included above
// #include <cassert>
// already included above
// #include <limits>
// already included above
// #include <string>
// already included above
// #include "rosidl_typesupport_fastrtps_c/identifier.h"
// already included above
// #include "rosidl_typesupport_fastrtps_c/wstring_conversion.hpp"
// already included above
// #include "rosidl_typesupport_fastrtps_cpp/message_type_support.h"
// already included above
// #include "whycode_interfaces/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
// already included above
// #include "whycode_interfaces/srv/detail/set_drawing__struct.h"
// already included above
// #include "whycode_interfaces/srv/detail/set_drawing__functions.h"
// already included above
// #include "fastcdr/Cdr.h"

#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-parameter"
# ifdef __clang__
#  pragma clang diagnostic ignored "-Wdeprecated-register"
#  pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
# endif
#endif
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif

// includes and forward declarations of message dependencies and their conversion functions

#if defined(__cplusplus)
extern "C"
{
#endif


// forward declare type support functions


using _SetDrawing_Response__ros_msg_type = whycode_interfaces__srv__SetDrawing_Response;

static bool _SetDrawing_Response__cdr_serialize(
  const void * untyped_ros_message,
  eprosima::fastcdr::Cdr & cdr)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  const _SetDrawing_Response__ros_msg_type * ros_message = static_cast<const _SetDrawing_Response__ros_msg_type *>(untyped_ros_message);
  // Field name: success
  {
    cdr << (ros_message->success ? true : false);
  }

  return true;
}

static bool _SetDrawing_Response__cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  void * untyped_ros_message)
{
  if (!untyped_ros_message) {
    fprintf(stderr, "ros message handle is null\n");
    return false;
  }
  _SetDrawing_Response__ros_msg_type * ros_message = static_cast<_SetDrawing_Response__ros_msg_type *>(untyped_ros_message);
  // Field name: success
  {
    uint8_t tmp;
    cdr >> tmp;
    ros_message->success = tmp ? true : false;
  }

  return true;
}  // NOLINT(readability/fn_size)

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_whycode_interfaces
size_t get_serialized_size_whycode_interfaces__srv__SetDrawing_Response(
  const void * untyped_ros_message,
  size_t current_alignment)
{
  const _SetDrawing_Response__ros_msg_type * ros_message = static_cast<const _SetDrawing_Response__ros_msg_type *>(untyped_ros_message);
  (void)ros_message;
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  (void)padding;
  (void)wchar_size;

  // field.name success
  {
    size_t item_size = sizeof(ros_message->success);
    current_alignment += item_size +
      eprosima::fastcdr::Cdr::alignment(current_alignment, item_size);
  }

  return current_alignment - initial_alignment;
}

static uint32_t _SetDrawing_Response__get_serialized_size(const void * untyped_ros_message)
{
  return static_cast<uint32_t>(
    get_serialized_size_whycode_interfaces__srv__SetDrawing_Response(
      untyped_ros_message, 0));
}

ROSIDL_TYPESUPPORT_FASTRTPS_C_PUBLIC_whycode_interfaces
size_t max_serialized_size_whycode_interfaces__srv__SetDrawing_Response(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment)
{
  size_t initial_alignment = current_alignment;

  const size_t padding = 4;
  const size_t wchar_size = 4;
  size_t last_member_size = 0;
  (void)last_member_size;
  (void)padding;
  (void)wchar_size;

  full_bounded = true;
  is_plain = true;

  // member: success
  {
    size_t array_size = 1;

    last_member_size = array_size * sizeof(uint8_t);
    current_alignment += array_size * sizeof(uint8_t);
  }

  size_t ret_val = current_alignment - initial_alignment;
  if (is_plain) {
    // All members are plain, and type is not empty.
    // We still need to check that the in-memory alignment
    // is the same as the CDR mandated alignment.
    using DataType = whycode_interfaces__srv__SetDrawing_Response;
    is_plain =
      (
      offsetof(DataType, success) +
      last_member_size
      ) == ret_val;
  }

  return ret_val;
}

static size_t _SetDrawing_Response__max_serialized_size(char & bounds_info)
{
  bool full_bounded;
  bool is_plain;
  size_t ret_val;

  ret_val = max_serialized_size_whycode_interfaces__srv__SetDrawing_Response(
    full_bounded, is_plain, 0);

  bounds_info =
    is_plain ? ROSIDL_TYPESUPPORT_FASTRTPS_PLAIN_TYPE :
    full_bounded ? ROSIDL_TYPESUPPORT_FASTRTPS_BOUNDED_TYPE : ROSIDL_TYPESUPPORT_FASTRTPS_UNBOUNDED_TYPE;
  return ret_val;
}


static message_type_support_callbacks_t __callbacks_SetDrawing_Response = {
  "whycode_interfaces::srv",
  "SetDrawing_Response",
  _SetDrawing_Response__cdr_serialize,
  _SetDrawing_Response__cdr_deserialize,
  _SetDrawing_Response__get_serialized_size,
  _SetDrawing_Response__max_serialized_size
};

static rosidl_message_type_support_t _SetDrawing_Response__type_support = {
  rosidl_typesupport_fastrtps_c__identifier,
  &__callbacks_SetDrawing_Response,
  get_message_typesupport_handle_function,
};

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, whycode_interfaces, srv, SetDrawing_Response)() {
  return &_SetDrawing_Response__type_support;
}

#if defined(__cplusplus)
}
#endif

#include "rosidl_typesupport_fastrtps_cpp/service_type_support.h"
#include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "rosidl_typesupport_fastrtps_c/identifier.h"
// already included above
// #include "whycode_interfaces/msg/rosidl_typesupport_fastrtps_c__visibility_control.h"
#include "whycode_interfaces/srv/set_drawing.h"

#if defined(__cplusplus)
extern "C"
{
#endif

static service_type_support_callbacks_t SetDrawing__callbacks = {
  "whycode_interfaces::srv",
  "SetDrawing",
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, whycode_interfaces, srv, SetDrawing_Request)(),
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, whycode_interfaces, srv, SetDrawing_Response)(),
};

static rosidl_service_type_support_t SetDrawing__handle = {
  rosidl_typesupport_fastrtps_c__identifier,
  &SetDrawing__callbacks,
  get_service_typesupport_handle_function,
};

const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_c, whycode_interfaces, srv, SetDrawing)() {
  return &SetDrawing__handle;
}

#if defined(__cplusplus)
}
#endif
