// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from whycode_interfaces:msg/Marker.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__MSG__DETAIL__MARKER__TRAITS_HPP_
#define WHYCODE_INTERFACES__MSG__DETAIL__MARKER__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "whycode_interfaces/msg/detail/marker__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'position'
#include "geometry_msgs/msg/detail/pose__traits.hpp"
// Member 'rotation'
#include "geometry_msgs/msg/detail/vector3__traits.hpp"

namespace whycode_interfaces
{

namespace msg
{

inline void to_flow_style_yaml(
  const Marker & msg,
  std::ostream & out)
{
  out << "{";
  // member: id
  {
    out << "id: ";
    rosidl_generator_traits::value_to_yaml(msg.id, out);
    out << ", ";
  }

  // member: size
  {
    out << "size: ";
    rosidl_generator_traits::value_to_yaml(msg.size, out);
    out << ", ";
  }

  // member: u
  {
    out << "u: ";
    rosidl_generator_traits::value_to_yaml(msg.u, out);
    out << ", ";
  }

  // member: v
  {
    out << "v: ";
    rosidl_generator_traits::value_to_yaml(msg.v, out);
    out << ", ";
  }

  // member: angle
  {
    out << "angle: ";
    rosidl_generator_traits::value_to_yaml(msg.angle, out);
    out << ", ";
  }

  // member: position
  {
    out << "position: ";
    to_flow_style_yaml(msg.position, out);
    out << ", ";
  }

  // member: rotation
  {
    out << "rotation: ";
    to_flow_style_yaml(msg.rotation, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Marker & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "id: ";
    rosidl_generator_traits::value_to_yaml(msg.id, out);
    out << "\n";
  }

  // member: size
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "size: ";
    rosidl_generator_traits::value_to_yaml(msg.size, out);
    out << "\n";
  }

  // member: u
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "u: ";
    rosidl_generator_traits::value_to_yaml(msg.u, out);
    out << "\n";
  }

  // member: v
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "v: ";
    rosidl_generator_traits::value_to_yaml(msg.v, out);
    out << "\n";
  }

  // member: angle
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "angle: ";
    rosidl_generator_traits::value_to_yaml(msg.angle, out);
    out << "\n";
  }

  // member: position
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "position:\n";
    to_block_style_yaml(msg.position, out, indentation + 2);
  }

  // member: rotation
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "rotation:\n";
    to_block_style_yaml(msg.rotation, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Marker & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace msg

}  // namespace whycode_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use whycode_interfaces::msg::to_block_style_yaml() instead")]]
inline void to_yaml(
  const whycode_interfaces::msg::Marker & msg,
  std::ostream & out, size_t indentation = 0)
{
  whycode_interfaces::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use whycode_interfaces::msg::to_yaml() instead")]]
inline std::string to_yaml(const whycode_interfaces::msg::Marker & msg)
{
  return whycode_interfaces::msg::to_yaml(msg);
}

template<>
inline const char * data_type<whycode_interfaces::msg::Marker>()
{
  return "whycode_interfaces::msg::Marker";
}

template<>
inline const char * name<whycode_interfaces::msg::Marker>()
{
  return "whycode_interfaces/msg/Marker";
}

template<>
struct has_fixed_size<whycode_interfaces::msg::Marker>
  : std::integral_constant<bool, has_fixed_size<geometry_msgs::msg::Pose>::value && has_fixed_size<geometry_msgs::msg::Vector3>::value> {};

template<>
struct has_bounded_size<whycode_interfaces::msg::Marker>
  : std::integral_constant<bool, has_bounded_size<geometry_msgs::msg::Pose>::value && has_bounded_size<geometry_msgs::msg::Vector3>::value> {};

template<>
struct is_message<whycode_interfaces::msg::Marker>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // WHYCODE_INTERFACES__MSG__DETAIL__MARKER__TRAITS_HPP_
