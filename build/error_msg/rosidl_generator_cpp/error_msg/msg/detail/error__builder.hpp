// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from error_msg:msg/Error.idl
// generated code does not contain a copyright notice

#ifndef ERROR_MSG__MSG__DETAIL__ERROR__BUILDER_HPP_
#define ERROR_MSG__MSG__DETAIL__ERROR__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "error_msg/msg/detail/error__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace error_msg
{

namespace msg
{

namespace builder
{

class Init_Error_yaw_error
{
public:
  explicit Init_Error_yaw_error(::error_msg::msg::Error & msg)
  : msg_(msg)
  {}
  ::error_msg::msg::Error yaw_error(::error_msg::msg::Error::_yaw_error_type arg)
  {
    msg_.yaw_error = std::move(arg);
    return std::move(msg_);
  }

private:
  ::error_msg::msg::Error msg_;
};

class Init_Error_throttle_error
{
public:
  explicit Init_Error_throttle_error(::error_msg::msg::Error & msg)
  : msg_(msg)
  {}
  Init_Error_yaw_error throttle_error(::error_msg::msg::Error::_throttle_error_type arg)
  {
    msg_.throttle_error = std::move(arg);
    return Init_Error_yaw_error(msg_);
  }

private:
  ::error_msg::msg::Error msg_;
};

class Init_Error_pitch_error
{
public:
  explicit Init_Error_pitch_error(::error_msg::msg::Error & msg)
  : msg_(msg)
  {}
  Init_Error_throttle_error pitch_error(::error_msg::msg::Error::_pitch_error_type arg)
  {
    msg_.pitch_error = std::move(arg);
    return Init_Error_throttle_error(msg_);
  }

private:
  ::error_msg::msg::Error msg_;
};

class Init_Error_roll_error
{
public:
  Init_Error_roll_error()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Error_pitch_error roll_error(::error_msg::msg::Error::_roll_error_type arg)
  {
    msg_.roll_error = std::move(arg);
    return Init_Error_pitch_error(msg_);
  }

private:
  ::error_msg::msg::Error msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::error_msg::msg::Error>()
{
  return error_msg::msg::builder::Init_Error_roll_error();
}

}  // namespace error_msg

#endif  // ERROR_MSG__MSG__DETAIL__ERROR__BUILDER_HPP_
