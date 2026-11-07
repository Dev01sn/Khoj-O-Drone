// generated from rosidl_typesupport_fastrtps_cpp/resource/idl__rosidl_typesupport_fastrtps_cpp.hpp.em
// with input from controller_msg:msg/PIDTune.idl
// generated code does not contain a copyright notice

#ifndef CONTROLLER_MSG__MSG__DETAIL__PID_TUNE__ROSIDL_TYPESUPPORT_FASTRTPS_CPP_HPP_
#define CONTROLLER_MSG__MSG__DETAIL__PID_TUNE__ROSIDL_TYPESUPPORT_FASTRTPS_CPP_HPP_

#include "rosidl_runtime_c/message_type_support_struct.h"
#include "rosidl_typesupport_interface/macros.h"
#include "controller_msg/msg/rosidl_typesupport_fastrtps_cpp__visibility_control.h"
#include "controller_msg/msg/detail/pid_tune__struct.hpp"

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

namespace controller_msg
{

namespace msg
{

namespace typesupport_fastrtps_cpp
{

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_controller_msg
cdr_serialize(
  const controller_msg::msg::PIDTune & ros_message,
  eprosima::fastcdr::Cdr & cdr);

bool
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_controller_msg
cdr_deserialize(
  eprosima::fastcdr::Cdr & cdr,
  controller_msg::msg::PIDTune & ros_message);

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_controller_msg
get_serialized_size(
  const controller_msg::msg::PIDTune & ros_message,
  size_t current_alignment);

size_t
ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_controller_msg
max_serialized_size_PIDTune(
  bool & full_bounded,
  bool & is_plain,
  size_t current_alignment);

}  // namespace typesupport_fastrtps_cpp

}  // namespace msg

}  // namespace controller_msg

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_FASTRTPS_CPP_PUBLIC_controller_msg
const rosidl_message_type_support_t *
  ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, controller_msg, msg, PIDTune)();

#ifdef __cplusplus
}
#endif

#endif  // CONTROLLER_MSG__MSG__DETAIL__PID_TUNE__ROSIDL_TYPESUPPORT_FASTRTPS_CPP_HPP_
