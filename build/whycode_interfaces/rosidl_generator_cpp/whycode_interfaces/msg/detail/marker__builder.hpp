// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from whycode_interfaces:msg/Marker.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__MSG__DETAIL__MARKER__BUILDER_HPP_
#define WHYCODE_INTERFACES__MSG__DETAIL__MARKER__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "whycode_interfaces/msg/detail/marker__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace whycode_interfaces
{

namespace msg
{

namespace builder
{

class Init_Marker_rotation
{
public:
  explicit Init_Marker_rotation(::whycode_interfaces::msg::Marker & msg)
  : msg_(msg)
  {}
  ::whycode_interfaces::msg::Marker rotation(::whycode_interfaces::msg::Marker::_rotation_type arg)
  {
    msg_.rotation = std::move(arg);
    return std::move(msg_);
  }

private:
  ::whycode_interfaces::msg::Marker msg_;
};

class Init_Marker_position
{
public:
  explicit Init_Marker_position(::whycode_interfaces::msg::Marker & msg)
  : msg_(msg)
  {}
  Init_Marker_rotation position(::whycode_interfaces::msg::Marker::_position_type arg)
  {
    msg_.position = std::move(arg);
    return Init_Marker_rotation(msg_);
  }

private:
  ::whycode_interfaces::msg::Marker msg_;
};

class Init_Marker_angle
{
public:
  explicit Init_Marker_angle(::whycode_interfaces::msg::Marker & msg)
  : msg_(msg)
  {}
  Init_Marker_position angle(::whycode_interfaces::msg::Marker::_angle_type arg)
  {
    msg_.angle = std::move(arg);
    return Init_Marker_position(msg_);
  }

private:
  ::whycode_interfaces::msg::Marker msg_;
};

class Init_Marker_v
{
public:
  explicit Init_Marker_v(::whycode_interfaces::msg::Marker & msg)
  : msg_(msg)
  {}
  Init_Marker_angle v(::whycode_interfaces::msg::Marker::_v_type arg)
  {
    msg_.v = std::move(arg);
    return Init_Marker_angle(msg_);
  }

private:
  ::whycode_interfaces::msg::Marker msg_;
};

class Init_Marker_u
{
public:
  explicit Init_Marker_u(::whycode_interfaces::msg::Marker & msg)
  : msg_(msg)
  {}
  Init_Marker_v u(::whycode_interfaces::msg::Marker::_u_type arg)
  {
    msg_.u = std::move(arg);
    return Init_Marker_v(msg_);
  }

private:
  ::whycode_interfaces::msg::Marker msg_;
};

class Init_Marker_size
{
public:
  explicit Init_Marker_size(::whycode_interfaces::msg::Marker & msg)
  : msg_(msg)
  {}
  Init_Marker_u size(::whycode_interfaces::msg::Marker::_size_type arg)
  {
    msg_.size = std::move(arg);
    return Init_Marker_u(msg_);
  }

private:
  ::whycode_interfaces::msg::Marker msg_;
};

class Init_Marker_id
{
public:
  Init_Marker_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Marker_size id(::whycode_interfaces::msg::Marker::_id_type arg)
  {
    msg_.id = std::move(arg);
    return Init_Marker_size(msg_);
  }

private:
  ::whycode_interfaces::msg::Marker msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::whycode_interfaces::msg::Marker>()
{
  return whycode_interfaces::msg::builder::Init_Marker_id();
}

}  // namespace whycode_interfaces

#endif  // WHYCODE_INTERFACES__MSG__DETAIL__MARKER__BUILDER_HPP_
