// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from whycode_interfaces:srv/GetGuiSettings.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__SRV__DETAIL__GET_GUI_SETTINGS__BUILDER_HPP_
#define WHYCODE_INTERFACES__SRV__DETAIL__GET_GUI_SETTINGS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "whycode_interfaces/srv/detail/get_gui_settings__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace whycode_interfaces
{

namespace srv
{


}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::whycode_interfaces::srv::GetGuiSettings_Request>()
{
  return ::whycode_interfaces::srv::GetGuiSettings_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace whycode_interfaces


namespace whycode_interfaces
{

namespace srv
{

namespace builder
{

class Init_GetGuiSettings_Response_coords
{
public:
  explicit Init_GetGuiSettings_Response_coords(::whycode_interfaces::srv::GetGuiSettings_Response & msg)
  : msg_(msg)
  {}
  ::whycode_interfaces::srv::GetGuiSettings_Response coords(::whycode_interfaces::srv::GetGuiSettings_Response::_coords_type arg)
  {
    msg_.coords = std::move(arg);
    return std::move(msg_);
  }

private:
  ::whycode_interfaces::srv::GetGuiSettings_Response msg_;
};

class Init_GetGuiSettings_Response_draw_segments
{
public:
  explicit Init_GetGuiSettings_Response_draw_segments(::whycode_interfaces::srv::GetGuiSettings_Response & msg)
  : msg_(msg)
  {}
  Init_GetGuiSettings_Response_coords draw_segments(::whycode_interfaces::srv::GetGuiSettings_Response::_draw_segments_type arg)
  {
    msg_.draw_segments = std::move(arg);
    return Init_GetGuiSettings_Response_coords(msg_);
  }

private:
  ::whycode_interfaces::srv::GetGuiSettings_Response msg_;
};

class Init_GetGuiSettings_Response_draw_coords
{
public:
  Init_GetGuiSettings_Response_draw_coords()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GetGuiSettings_Response_draw_segments draw_coords(::whycode_interfaces::srv::GetGuiSettings_Response::_draw_coords_type arg)
  {
    msg_.draw_coords = std::move(arg);
    return Init_GetGuiSettings_Response_draw_segments(msg_);
  }

private:
  ::whycode_interfaces::srv::GetGuiSettings_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::whycode_interfaces::srv::GetGuiSettings_Response>()
{
  return whycode_interfaces::srv::builder::Init_GetGuiSettings_Response_draw_coords();
}

}  // namespace whycode_interfaces

#endif  // WHYCODE_INTERFACES__SRV__DETAIL__GET_GUI_SETTINGS__BUILDER_HPP_
