// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from whycode_interfaces:srv/GetGuiSettings.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__SRV__DETAIL__GET_GUI_SETTINGS__TRAITS_HPP_
#define WHYCODE_INTERFACES__SRV__DETAIL__GET_GUI_SETTINGS__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "whycode_interfaces/srv/detail/get_gui_settings__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace whycode_interfaces
{

namespace srv
{

inline void to_flow_style_yaml(
  const GetGuiSettings_Request & msg,
  std::ostream & out)
{
  (void)msg;
  out << "null";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const GetGuiSettings_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  (void)msg;
  (void)indentation;
  out << "null\n";
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const GetGuiSettings_Request & msg, bool use_flow_style = false)
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
  const whycode_interfaces::srv::GetGuiSettings_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  whycode_interfaces::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use whycode_interfaces::srv::to_yaml() instead")]]
inline std::string to_yaml(const whycode_interfaces::srv::GetGuiSettings_Request & msg)
{
  return whycode_interfaces::srv::to_yaml(msg);
}

template<>
inline const char * data_type<whycode_interfaces::srv::GetGuiSettings_Request>()
{
  return "whycode_interfaces::srv::GetGuiSettings_Request";
}

template<>
inline const char * name<whycode_interfaces::srv::GetGuiSettings_Request>()
{
  return "whycode_interfaces/srv/GetGuiSettings_Request";
}

template<>
struct has_fixed_size<whycode_interfaces::srv::GetGuiSettings_Request>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<whycode_interfaces::srv::GetGuiSettings_Request>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<whycode_interfaces::srv::GetGuiSettings_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace whycode_interfaces
{

namespace srv
{

inline void to_flow_style_yaml(
  const GetGuiSettings_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: draw_coords
  {
    out << "draw_coords: ";
    rosidl_generator_traits::value_to_yaml(msg.draw_coords, out);
    out << ", ";
  }

  // member: draw_segments
  {
    out << "draw_segments: ";
    rosidl_generator_traits::value_to_yaml(msg.draw_segments, out);
    out << ", ";
  }

  // member: coords
  {
    out << "coords: ";
    rosidl_generator_traits::value_to_yaml(msg.coords, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const GetGuiSettings_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: draw_coords
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "draw_coords: ";
    rosidl_generator_traits::value_to_yaml(msg.draw_coords, out);
    out << "\n";
  }

  // member: draw_segments
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "draw_segments: ";
    rosidl_generator_traits::value_to_yaml(msg.draw_segments, out);
    out << "\n";
  }

  // member: coords
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "coords: ";
    rosidl_generator_traits::value_to_yaml(msg.coords, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const GetGuiSettings_Response & msg, bool use_flow_style = false)
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
  const whycode_interfaces::srv::GetGuiSettings_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  whycode_interfaces::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use whycode_interfaces::srv::to_yaml() instead")]]
inline std::string to_yaml(const whycode_interfaces::srv::GetGuiSettings_Response & msg)
{
  return whycode_interfaces::srv::to_yaml(msg);
}

template<>
inline const char * data_type<whycode_interfaces::srv::GetGuiSettings_Response>()
{
  return "whycode_interfaces::srv::GetGuiSettings_Response";
}

template<>
inline const char * name<whycode_interfaces::srv::GetGuiSettings_Response>()
{
  return "whycode_interfaces/srv/GetGuiSettings_Response";
}

template<>
struct has_fixed_size<whycode_interfaces::srv::GetGuiSettings_Response>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<whycode_interfaces::srv::GetGuiSettings_Response>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<whycode_interfaces::srv::GetGuiSettings_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<whycode_interfaces::srv::GetGuiSettings>()
{
  return "whycode_interfaces::srv::GetGuiSettings";
}

template<>
inline const char * name<whycode_interfaces::srv::GetGuiSettings>()
{
  return "whycode_interfaces/srv/GetGuiSettings";
}

template<>
struct has_fixed_size<whycode_interfaces::srv::GetGuiSettings>
  : std::integral_constant<
    bool,
    has_fixed_size<whycode_interfaces::srv::GetGuiSettings_Request>::value &&
    has_fixed_size<whycode_interfaces::srv::GetGuiSettings_Response>::value
  >
{
};

template<>
struct has_bounded_size<whycode_interfaces::srv::GetGuiSettings>
  : std::integral_constant<
    bool,
    has_bounded_size<whycode_interfaces::srv::GetGuiSettings_Request>::value &&
    has_bounded_size<whycode_interfaces::srv::GetGuiSettings_Response>::value
  >
{
};

template<>
struct is_service<whycode_interfaces::srv::GetGuiSettings>
  : std::true_type
{
};

template<>
struct is_service_request<whycode_interfaces::srv::GetGuiSettings_Request>
  : std::true_type
{
};

template<>
struct is_service_response<whycode_interfaces::srv::GetGuiSettings_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // WHYCODE_INTERFACES__SRV__DETAIL__GET_GUI_SETTINGS__TRAITS_HPP_
