// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from whycode_interfaces:srv/SetDrawing.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__SRV__DETAIL__SET_DRAWING__BUILDER_HPP_
#define WHYCODE_INTERFACES__SRV__DETAIL__SET_DRAWING__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "whycode_interfaces/srv/detail/set_drawing__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace whycode_interfaces
{

namespace srv
{

namespace builder
{

class Init_SetDrawing_Request_draw_segments
{
public:
  explicit Init_SetDrawing_Request_draw_segments(::whycode_interfaces::srv::SetDrawing_Request & msg)
  : msg_(msg)
  {}
  ::whycode_interfaces::srv::SetDrawing_Request draw_segments(::whycode_interfaces::srv::SetDrawing_Request::_draw_segments_type arg)
  {
    msg_.draw_segments = std::move(arg);
    return std::move(msg_);
  }

private:
  ::whycode_interfaces::srv::SetDrawing_Request msg_;
};

class Init_SetDrawing_Request_draw_coords
{
public:
  Init_SetDrawing_Request_draw_coords()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetDrawing_Request_draw_segments draw_coords(::whycode_interfaces::srv::SetDrawing_Request::_draw_coords_type arg)
  {
    msg_.draw_coords = std::move(arg);
    return Init_SetDrawing_Request_draw_segments(msg_);
  }

private:
  ::whycode_interfaces::srv::SetDrawing_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::whycode_interfaces::srv::SetDrawing_Request>()
{
  return whycode_interfaces::srv::builder::Init_SetDrawing_Request_draw_coords();
}

}  // namespace whycode_interfaces


namespace whycode_interfaces
{

namespace srv
{

namespace builder
{

class Init_SetDrawing_Response_success
{
public:
  Init_SetDrawing_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::whycode_interfaces::srv::SetDrawing_Response success(::whycode_interfaces::srv::SetDrawing_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return std::move(msg_);
  }

private:
  ::whycode_interfaces::srv::SetDrawing_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::whycode_interfaces::srv::SetDrawing_Response>()
{
  return whycode_interfaces::srv::builder::Init_SetDrawing_Response_success();
}

}  // namespace whycode_interfaces

#endif  // WHYCODE_INTERFACES__SRV__DETAIL__SET_DRAWING__BUILDER_HPP_
