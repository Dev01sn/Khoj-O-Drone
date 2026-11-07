// generated from rosidl_typesupport_fastrtps_cpp/resource/idl__rosidl_typesupport_fastrtps_cpp.hpp.em
// with input from error_msg:msg/Error.idl
// generated code does not contain a copyright notice

#ifndef ERROR_MSG__MSG__DETAIL__ERROR__ROSIDL_TYPESUPPORT_FASTRTPS_CPP_HPP_
#define ERROR_MSG__MSG__DETAIL__ERROR__ROSIDL_TYPESUPPORT_FASTRTPS_CPP_HPP_

#include "rosidl_runtime_c/message_type_support_struct.h"
#include "rosidl_typesupport_interface/macros.h"
#include "error_msg/msg/rosidl_typesupport_fastrtps_cpp__visibility_control.h"
#include "error_msg/msg/detail/error__struct.hpp"

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

#include "fastcdr/Cdr.h"

namespace error_msg
{

namespace msg
{

namespace typesupport_fastrtps_cpp
{

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_error_msg
cdr_serialize(
  const error_msg::msg::Error & ros_message,
  eprosima::fastcdr::Cdr & cdr);

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_error_msg
cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  error_msg::msg::Error & ros_message);

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_error_msg
get_serialized_size(
  const error_msg::msg::Error & ros_message,
  size_t current_alignment);

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_error_msg
max_serialized_size_Error(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

}  // namespace typesupport_fastrtps_cpp

}  // namespace msg

}  // namespace error_msg

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_error_msg
const rosidl_message_type_support_t *
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, error_msg, msg, Error)();

#ifdef __cplusplus
}
#endif

#endif  // ERROR_MSG__MSG__DETAIL__ERROR__ROSIDL_TYPESUPPORT_FASTRTPS_CPP_HPP_
