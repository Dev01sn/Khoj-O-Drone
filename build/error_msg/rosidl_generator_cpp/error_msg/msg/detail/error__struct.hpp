// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from error_msg:msg/Error.idl
// generated code does not contain a copyright notice

#ifndef ERROR_MSG__MSG__DETAIL__ERROR__STRUCT_HPP_
#define ERROR_MSG__MSG__DETAIL__ERROR__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__error_msg__msg__Error __attribute__((deprecated))
#else
# define DEPRECATED__error_msg__msg__Error __declspec(deprecated)
#endif

namespace error_msg
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct Error_
{
  using Type = Error_<ContainerAllocator>;

  explicit Error_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->roll_error = 0.0f;
      this->pitch_error = 0.0f;
      this->throttle_error = 0.0f;
      this->yaw_error = 0.0f;
    }
  }

  explicit Error_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->roll_error = 0.0f;
      this->pitch_error = 0.0f;
      this->throttle_error = 0.0f;
      this->yaw_error = 0.0f;
    }
  }

  // field types and members
  using _roll_error_type =
    float;
  _roll_error_type roll_error;
  using _pitch_error_type =
    float;
  _pitch_error_type pitch_error;
  using _throttle_error_type =
    float;
  _throttle_error_type throttle_error;
  using _yaw_error_type =
    float;
  _yaw_error_type yaw_error;

  // setters for named parameter idiom
  Type & set__roll_error(
    const float & _arg)
  {
    this->roll_error = _arg;
    return *this;
  }
  Type & set__pitch_error(
    const float & _arg)
  {
    this->pitch_error = _arg;
    return *this;
  }
  Type & set__throttle_error(
    const float & _arg)
  {
    this->throttle_error = _arg;
    return *this;
  }
  Type & set__yaw_error(
    const float & _arg)
  {
    this->yaw_error = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    error_msg::msg::Error_<ContainerAllocator> *;
  using ConstRawPtr =
    const error_msg::msg::Error_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<error_msg::msg::Error_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<error_msg::msg::Error_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      error_msg::msg::Error_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<error_msg::msg::Error_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      error_msg::msg::Error_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<error_msg::msg::Error_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<error_msg::msg::Error_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<error_msg::msg::Error_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__error_msg__msg__Error
    std::shared_ptr<error_msg::msg::Error_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__error_msg__msg__Error
    std::shared_ptr<error_msg::msg::Error_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const Error_ & other) const
  {
    if (this->roll_error != other.roll_error) {
      return false;
    }
    if (this->pitch_error != other.pitch_error) {
      return false;
    }
    if (this->throttle_error != other.throttle_error) {
      return false;
    }
    if (this->yaw_error != other.yaw_error) {
      return false;
    }
    return true;
  }
  bool operator!=(const Error_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct Error_

// alias to use template instance with default allocator
using Error =
  error_msg::msg::Error_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace error_msg

#endif  // ERROR_MSG__MSG__DETAIL__ERROR__STRUCT_HPP_
