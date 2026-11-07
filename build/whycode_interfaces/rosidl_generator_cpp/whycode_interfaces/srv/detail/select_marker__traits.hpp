// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from whycode_interfaces:srv/SelectMarker.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__SRV__DETAIL__SELECT_MARKER__TRAITS_HPP_
#define WHYCODE_INTERFACES__SRV__DETAIL__SELECT_MARKER__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "whycode_interfaces/srv/detail/select_marker__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'point'
#include "geometry_msgs/msg/detail/point__traits.hpp"

namespace whycode_interfaces
{

namespace srv
{

inline void to_flow_style_yaml(
  const SelectMarker_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: point
  {
    out << "point: ";
    to_flow_style_yaml(msg.point, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const SelectMarker_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: point
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "point:\n";
    to_block_style_yaml(msg.point, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const SelectMarker_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace whycode_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use whycode_interfaces::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const whycode_interfaces::srv::SelectMarker_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  whycode_interfaces::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use whycode_interfaces::srv::to_yaml() instead")]]
inline std::string to_yaml(const whycode_interfaces::srv::SelectMarker_Request & msg)
{
  return whycode_interfaces::srv::to_yaml(msg);
}

template<>
inline const char * data_type<whycode_interfaces::srv::SelectMarker_Request>()
{
  return "whycode_interfaces::srv::SelectMarker_Request";
}

template<>
inline const char * name<whycode_interfaces::srv::SelectMarker_Request>()
{
  return "whycode_interfaces/srv/SelectMarker_Request";
}

template<>
struct has_fixed_size<whycode_interfaces::srv::SelectMarker_Request>
  : std::integral_constant<bool, has_fixed_size<geometry_msgs::msg::Point>::value> {};

template<>
struct has_bounded_size<whycode_interfaces::srv::SelectMarker_Request>
  : std::integral_constant<bool, has_bounded_size<geometry_msgs::msg::Point>::value> {};

template<>
struct is_message<whycode_interfaces::srv::SelectMarker_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace whycode_interfaces
{

namespace srv
{

inline void to_flow_style_yaml(
  const SelectMarker_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: success
  {
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const SelectMarker_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: success
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const SelectMarker_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace whycode_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use whycode_interfaces::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const whycode_interfaces::srv::SelectMarker_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  whycode_interfaces::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use whycode_interfaces::srv::to_yaml() instead")]]
inline std::string to_yaml(const whycode_interfaces::srv::SelectMarker_Response & msg)
{
  return whycode_interfaces::srv::to_yaml(msg);
}

template<>
inline const char * data_type<whycode_interfaces::srv::SelectMarker_Response>()
{
  return "whycode_interfaces::srv::SelectMarker_Response";
}

template<>
inline const char * name<whycode_interfaces::srv::SelectMarker_Response>()
{
  return "whycode_interfaces/srv/SelectMarker_Response";
}

template<>
struct has_fixed_size<whycode_interfaces::srv::SelectMarker_Response>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<whycode_interfaces::srv::SelectMarker_Response>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<whycode_interfaces::srv::SelectMarker_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<whycode_interfaces::srv::SelectMarker>()
{
  return "whycode_interfaces::srv::SelectMarker";
}

template<>
inline const char * name<whycode_interfaces::srv::SelectMarker>()
{
  return "whycode_interfaces/srv/SelectMarker";
}

template<>
struct has_fixed_size<whycode_interfaces::srv::SelectMarker>
  : std::integral_constant<
    bool,
    has_fixed_size<whycode_interfaces::srv::SelectMarker_Request>::value &&
    has_fixed_size<whycode_interfaces::srv::SelectMarker_Response>::value
  >
{
};

template<>
struct has_bounded_size<whycode_interfaces::srv::SelectMarker>
  : std::integral_constant<
    bool,
    has_bounded_size<whycode_interfaces::srv::SelectMarker_Request>::value &&
    has_bounded_size<whycode_interfaces::srv::SelectMarker_Response>::value
  >
{
};

template<>
struct is_service<whycode_interfaces::srv::SelectMarker>
  : std::true_type
{
};

template<>
struct is_service_request<whycode_interfaces::srv::SelectMarker_Request>
  : std::true_type
{
};

template<>
struct is_service_response<whycode_interfaces::srv::SelectMarker_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // WHYCODE_INTERFACES__SRV__DETAIL__SELECT_MARKER__TRAITS_HPP_
