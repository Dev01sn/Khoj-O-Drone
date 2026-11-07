// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from whycode_interfaces:srv/SetCalibPath.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__SRV__DETAIL__SET_CALIB_PATH__STRUCT_HPP_
#define WHYCODE_INTERFACES__SRV__DETAIL__SET_CALIB_PATH__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__whycode_interfaces__srv__SetCalibPath_Request __attribute__((deprecated))
#else
# define DEPRECATED__whycode_interfaces__srv__SetCalibPath_Request __declspec(deprecated)
#endif

namespace whycode_interfaces
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SetCalibPath_Request_
{
  using Type = SetCalibPath_Request_<ContainerAllocator>;

  explicit SetCalibPath_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->action = "";
      this->path = "";
    }
  }

  explicit SetCalibPath_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : action(_alloc),
    path(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->action = "";
      this->path = "";
    }
  }

  // field types and members
  using _action_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _action_type action;
  using _path_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _path_type path;

  // setters for named parameter idiom
  Type & set__action(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->action = _arg;
    return *this;
  }
  Type & set__path(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->path = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    whycode_interfaces::srv::SetCalibPath_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const whycode_interfaces::srv::SetCalibPath_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<whycode_interfaces::srv::SetCalibPath_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<whycode_interfaces::srv::SetCalibPath_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      whycode_interfaces::srv::SetCalibPath_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<whycode_interfaces::srv::SetCalibPath_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      whycode_interfaces::srv::SetCalibPath_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<whycode_interfaces::srv::SetCalibPath_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<whycode_interfaces::srv::SetCalibPath_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<whycode_interfaces::srv::SetCalibPath_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__whycode_interfaces__srv__SetCalibPath_Request
    std::shared_ptr<whycode_interfaces::srv::SetCalibPath_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__whycode_interfaces__srv__SetCalibPath_Request
    std::shared_ptr<whycode_interfaces::srv::SetCalibPath_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SetCalibPath_Request_ & other) const
  {
    if (this->action != other.action) {
      return false;
    }
    if (this->path != other.path) {
      return false;
    }
    return true;
  }
  bool operator!=(const SetCalibPath_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SetCalibPath_Request_

// alias to use template instance with default allocator
using SetCalibPath_Request =
  whycode_interfaces::srv::SetCalibPath_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace whycode_interfaces


#ifndef _WIN32
# define DEPRECATED__whycode_interfaces__srv__SetCalibPath_Response __attribute__((deprecated))
#else
# define DEPRECATED__whycode_interfaces__srv__SetCalibPath_Response __declspec(deprecated)
#endif

namespace whycode_interfaces
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SetCalibPath_Response_
{
  using Type = SetCalibPath_Response_<ContainerAllocator>;

  explicit SetCalibPath_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->msg = "";
    }
  }

  explicit SetCalibPath_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : msg(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->msg = "";
    }
  }

  // field types and members
  using _success_type =
    bool;
  _success_type success;
  using _msg_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _msg_type msg;

  // setters for named parameter idiom
  Type & set__success(
    const bool & _arg)
  {
    this->success = _arg;
    return *this;
  }
  Type & set__msg(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->msg = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    whycode_interfaces::srv::SetCalibPath_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const whycode_interfaces::srv::SetCalibPath_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<whycode_interfaces::srv::SetCalibPath_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<whycode_interfaces::srv::SetCalibPath_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      whycode_interfaces::srv::SetCalibPath_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<whycode_interfaces::srv::SetCalibPath_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      whycode_interfaces::srv::SetCalibPath_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<whycode_interfaces::srv::SetCalibPath_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<whycode_interfaces::srv::SetCalibPath_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<whycode_interfaces::srv::SetCalibPath_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__whycode_interfaces__srv__SetCalibPath_Response
    std::shared_ptr<whycode_interfaces::srv::SetCalibPath_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__whycode_interfaces__srv__SetCalibPath_Response
    std::shared_ptr<whycode_interfaces::srv::SetCalibPath_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SetCalibPath_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->msg != other.msg) {
      return false;
    }
    return true;
  }
  bool operator!=(const SetCalibPath_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SetCalibPath_Response_

// alias to use template instance with default allocator
using SetCalibPath_Response =
  whycode_interfaces::srv::SetCalibPath_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace whycode_interfaces

namespace whycode_interfaces
{

namespace srv
{

struct SetCalibPath
{
  using Request = whycode_interfaces::srv::SetCalibPath_Request;
  using Response = whycode_interfaces::srv::SetCalibPath_Response;
};

}  // namespace srv

}  // namespace whycode_interfaces

#endif  // WHYCODE_INTERFACES__SRV__DETAIL__SET_CALIB_PATH__STRUCT_HPP_
