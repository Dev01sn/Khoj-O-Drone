// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from controller_msg:msg/PIDTune.idl
// generated code does not contain a copyright notice

#ifndef CONTROLLER_MSG__MSG__DETAIL__PID_TUNE__BUILDER_HPP_
#define CONTROLLER_MSG__MSG__DETAIL__PID_TUNE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "controller_msg/msg/detail/pid_tune__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace controller_msg
{

namespace msg
{

namespace builder
{

class Init_PIDTune_kd
{
public:
  explicit Init_PIDTune_kd(::controller_msg::msg::PIDTune & msg)
  : msg_(msg)
  {}
  ::controller_msg::msg::PIDTune kd(::controller_msg::msg::PIDTune::_kd_type arg)
  {
    msg_.kd = std::move(arg);
    return std::move(msg_);
  }

private:
  ::controller_msg::msg::PIDTune msg_;
};

class Init_PIDTune_ki
{
public:
  explicit Init_PIDTune_ki(::controller_msg::msg::PIDTune & msg)
  : msg_(msg)
  {}
  Init_PIDTune_kd ki(::controller_msg::msg::PIDTune::_ki_type arg)
  {
    msg_.ki = std::move(arg);
    return Init_PIDTune_kd(msg_);
  }

private:
  ::controller_msg::msg::PIDTune msg_;
};

class Init_PIDTune_kp
{
public:
  Init_PIDTune_kp()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_PIDTune_ki kp(::controller_msg::msg::PIDTune::_kp_type arg)
  {
    msg_.kp = std::move(arg);
    return Init_PIDTune_ki(msg_);
  }

private:
  ::controller_msg::msg::PIDTune msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::controller_msg::msg::PIDTune>()
{
  return controller_msg::msg::builder::Init_PIDTune_kp();
}

}  // namespace controller_msg

#endif  // CONTROLLER_MSG__MSG__DETAIL__PID_TUNE__BUILDER_HPP_
