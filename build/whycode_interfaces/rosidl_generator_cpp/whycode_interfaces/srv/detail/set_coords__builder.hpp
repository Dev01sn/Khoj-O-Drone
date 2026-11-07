// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from whycode_interfaces:srv/SetCoords.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__SRV__DETAIL__SET_COORDS__BUILDER_HPP_
#define WHYCODE_INTERFACES__SRV__DETAIL__SET_COORDS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "whycode_interfaces/srv/detail/set_coords__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace whycode_interfaces
{

namespace srv
{

namespace builder
{

class Init_SetCoords_Request_coords
{
public:
  Init_SetCoords_Request_coords()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::whycode_interfaces::srv::SetCoords_Request coords(::whycode_interfaces::srv::SetCoords_Request::_coords_type arg)
  {
    msg_.coords = std::move(arg);
    return std::move(msg_);
  }

private:
  ::whycode_interfaces::srv::SetCoords_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::whycode_interfaces::srv::SetCoords_Request>()
{
  return whycode_interfaces::srv::builder::Init_SetCoords_Request_coords();
}

}  // namespace whycode_interfaces


namespace whycode_interfaces
{

namespace srv
{

namespace builder
{

class Init_SetCoords_Response_msg
{
public:
  explicit Init_SetCoords_Response_msg(::whycode_interfaces::srv::SetCoords_Response & msg)
  : msg_(msg)
  {}
  ::whycode_interfaces::srv::SetCoords_Response msg(::whycode_interfaces::srv::SetCoords_Response::_msg_type arg)
  {
    msg_.msg = std::move(arg);
    return std::move(msg_);
  }

private:
  ::whycode_interfaces::srv::SetCoords_Response msg_;
};

class Init_SetCoords_Response_success
{
public:
  Init_SetCoords_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetCoords_Response_msg success(::whycode_interfaces::srv::SetCoords_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_SetCoords_Response_msg(msg_);
  }

private:
  ::whycode_interfaces::srv::SetCoords_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::whycode_interfaces::srv::SetCoords_Response>()
{
  return whycode_interfaces::srv::builder::Init_SetCoords_Response_success();
}

}  // namespace whycode_interfaces

#endif  // WHYCODE_INTERFACES__SRV__DETAIL__SET_COORDS__BUILDER_HPP_
