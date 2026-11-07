// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from whycode_interfaces:srv/SelectMarker.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__SRV__DETAIL__SELECT_MARKER__BUILDER_HPP_
#define WHYCODE_INTERFACES__SRV__DETAIL__SELECT_MARKER__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "whycode_interfaces/srv/detail/select_marker__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace whycode_interfaces
{

namespace srv
{

namespace builder
{

class Init_SelectMarker_Request_point
{
public:
  Init_SelectMarker_Request_point()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::whycode_interfaces::srv::SelectMarker_Request point(::whycode_interfaces::srv::SelectMarker_Request::_point_type arg)
  {
    msg_.point = std::move(arg);
    return std::move(msg_);
  }

private:
  ::whycode_interfaces::srv::SelectMarker_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::whycode_interfaces::srv::SelectMarker_Request>()
{
  return whycode_interfaces::srv::builder::Init_SelectMarker_Request_point();
}

}  // namespace whycode_interfaces


namespace whycode_interfaces
{

namespace srv
{

namespace builder
{

class Init_SelectMarker_Response_success
{
public:
  Init_SelectMarker_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::whycode_interfaces::srv::SelectMarker_Response success(::whycode_interfaces::srv::SelectMarker_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return std::move(msg_);
  }

private:
  ::whycode_interfaces::srv::SelectMarker_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::whycode_interfaces::srv::SelectMarker_Response>()
{
  return whycode_interfaces::srv::builder::Init_SelectMarker_Response_success();
}

}  // namespace whycode_interfaces

#endif  // WHYCODE_INTERFACES__SRV__DETAIL__SELECT_MARKER__BUILDER_HPP_
