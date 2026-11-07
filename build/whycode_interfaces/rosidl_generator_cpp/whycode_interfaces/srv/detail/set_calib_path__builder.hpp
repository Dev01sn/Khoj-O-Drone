// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from whycode_interfaces:srv/SetCalibPath.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__SRV__DETAIL__SET_CALIB_PATH__BUILDER_HPP_
#define WHYCODE_INTERFACES__SRV__DETAIL__SET_CALIB_PATH__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "whycode_interfaces/srv/detail/set_calib_path__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace whycode_interfaces
{

namespace srv
{

namespace builder
{

class Init_SetCalibPath_Request_path
{
public:
  explicit Init_SetCalibPath_Request_path(::whycode_interfaces::srv::SetCalibPath_Request & msg)
  : msg_(msg)
  {}
  ::whycode_interfaces::srv::SetCalibPath_Request path(::whycode_interfaces::srv::SetCalibPath_Request::_path_type arg)
  {
    msg_.path = std::move(arg);
    return std::move(msg_);
  }

private:
  ::whycode_interfaces::srv::SetCalibPath_Request msg_;
};

class Init_SetCalibPath_Request_action
{
public:
  Init_SetCalibPath_Request_action()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetCalibPath_Request_path action(::whycode_interfaces::srv::SetCalibPath_Request::_action_type arg)
  {
    msg_.action = std::move(arg);
    return Init_SetCalibPath_Request_path(msg_);
  }

private:
  ::whycode_interfaces::srv::SetCalibPath_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::whycode_interfaces::srv::SetCalibPath_Request>()
{
  return whycode_interfaces::srv::builder::Init_SetCalibPath_Request_action();
}

}  // namespace whycode_interfaces


namespace whycode_interfaces
{

namespace srv
{

namespace builder
{

class Init_SetCalibPath_Response_msg
{
public:
  explicit Init_SetCalibPath_Response_msg(::whycode_interfaces::srv::SetCalibPath_Response & msg)
  : msg_(msg)
  {}
  ::whycode_interfaces::srv::SetCalibPath_Response msg(::whycode_interfaces::srv::SetCalibPath_Response::_msg_type arg)
  {
    msg_.msg = std::move(arg);
    return std::move(msg_);
  }

private:
  ::whycode_interfaces::srv::SetCalibPath_Response msg_;
};

class Init_SetCalibPath_Response_success
{
public:
  Init_SetCalibPath_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetCalibPath_Response_msg success(::whycode_interfaces::srv::SetCalibPath_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_SetCalibPath_Response_msg(msg_);
  }

private:
  ::whycode_interfaces::srv::SetCalibPath_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::whycode_interfaces::srv::SetCalibPath_Response>()
{
  return whycode_interfaces::srv::builder::Init_SetCalibPath_Response_success();
}

}  // namespace whycode_interfaces

#endif  // WHYCODE_INTERFACES__SRV__DETAIL__SET_CALIB_PATH__BUILDER_HPP_
