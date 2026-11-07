// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from error_msg:msg/Error.idl
// generated code does not contain a copyright notice

#ifndef ERROR_MSG__MSG__DETAIL__ERROR__TRAITS_HPP_
#define ERROR_MSG__MSG__DETAIL__ERROR__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "error_msg/msg/detail/error__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace error_msg
{

namespace msg
{

inline void to_flow_style_yaml(
  const Error & msg,
  std::ostream & out)
{
  out << "{";
  // member: roll_error
  {
    out << "roll_error: ";
    rosidl_generator_traits::value_to_yaml(msg.roll_error, out);
    out << ", ";
  }

  // member: pitch_error
  {
    out << "pitch_error: ";
    rosidl_generator_traits::value_to_yaml(msg.pitch_error, out);
    out << ", ";
  }

  // member: throttle_error
  {
    out << "throttle_error: ";
    rosidl_generator_traits::value_to_yaml(msg.throttle_error, out);
    out << ", ";
  }

  // member: yaw_error
  {
    out << "yaw_error: ";
    rosidl_generator_traits::value_to_yaml(msg.yaw_error, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Error & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: roll_error
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "roll_error: ";
    rosidl_generator_traits::value_to_yaml(msg.roll_error, out);
    out << "\n";
  }

  // member: pitch_error
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "pitch_error: ";
    rosidl_generator_traits::value_to_yaml(msg.pitch_error, out);
    out << "\n";
  }

  // member: throttle_error
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "throttle_error: ";
    rosidl_generator_traits::value_to_yaml(msg.throttle_error, out);
    out << "\n";
  }

  // member: yaw_error
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "yaw_error: ";
    rosidl_generator_traits::value_to_yaml(msg.yaw_error, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Error & msg, bool use_flow_style = false)
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

}  // namespace error_msg

namespace rosidl_generator_traits
{

[[deprecated("use error_msg::msg::to_block_style_yaml() instead")]]
inline void to_yaml(
  const error_msg::msg::Error & msg,
  std::ostream & out, size_t indentation = 0)
{
  error_msg::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use error_msg::msg::to_yaml() instead")]]
inline std::string to_yaml(const error_msg::msg::Error & msg)
{
  return error_msg::msg::to_yaml(msg);
}

template<>
inline const char * data_type<error_msg::msg::Error>()
{
  return "error_msg::msg::Error";
}

template<>
inline const char * name<error_msg::msg::Error>()
{
  return "error_msg/msg/Error";
}

template<>
struct has_fixed_size<error_msg::msg::Error>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<error_msg::msg::Error>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<error_msg::msg::Error>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // ERROR_MSG__MSG__DETAIL__ERROR__TRAITS_HPP_
