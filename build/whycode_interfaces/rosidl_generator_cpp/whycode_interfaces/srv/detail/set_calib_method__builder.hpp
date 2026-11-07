// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from whycode_interfaces:srv/SetCalibMethod.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__SRV__DETAIL__SET_CALIB_METHOD__BUILDER_HPP_
#define WHYCODE_INTERFACES__SRV__DETAIL__SET_CALIB_METHOD__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "whycode_interfaces/srv/detail/set_calib_method__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace whycode_interfaces
{

namespace srv
{

namespace builder
{

class Init_SetCalibMethod_Request_method
{
public:
  Init_SetCalibMethod_Request_method()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::whycode_interfaces::srv::SetCalibMethod_Request method(::whycode_interfaces::srv::SetCalibMethod_Request::_method_type arg)
  {
    msg_.method = std::move(arg);
    return std::move(msg_);
  }

private:
  ::whycode_interfaces::srv::SetCalibMethod_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::whycode_interfaces::srv::SetCalibMethod_Request>()
{
  return whycode_interfaces::srv::builder::Init_SetCalibMethod_Request_method();
}

}  // namespace whycode_interfaces


namespace whycode_interfaces
{

namespace srv
{

namespace builder
{

class Init_SetCalibMethod_Response_msg
{
public:
  explicit Init_SetCalibMethod_Response_msg(::whycode_interfaces::srv::SetCalibMethod_Response & msg)
  : msg_(msg)
  {}
  ::whycode_interfaces::srv::SetCalibMethod_Response msg(::whycode_interfaces::srv::SetCalibMethod_Response::_msg_type arg)
  {
    msg_.msg = std::move(arg);
    return std::move(msg_);
  }

private:
  ::whycode_interfaces::srv::SetCalibMethod_Response msg_;
};

class Init_SetCalibMethod_Response_success
{
public:
  Init_SetCalibMethod_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetCalibMethod_Response_msg success(::whycode_interfaces::srv::SetCalibMethod_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_SetCalibMethod_Response_msg(msg_);
  }

private:
  ::whycode_interfaces::srv::SetCalibMethod_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::whycode_interfaces::srv::SetCalibMethod_Response>()
{
  return whycode_interfaces::srv::builder::Init_SetCalibMethod_Response_success();
}

}  // namespace whycode_interfaces

#endif  // WHYCODE_INTERFACES__SRV__DETAIL__SET_CALIB_METHOD__BUILDER_HPP_
