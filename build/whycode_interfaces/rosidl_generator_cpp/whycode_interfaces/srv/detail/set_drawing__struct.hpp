// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from whycode_interfaces:srv/SetDrawing.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__SRV__DETAIL__SET_DRAWING__STRUCT_HPP_
#define WHYCODE_INTERFACES__SRV__DETAIL__SET_DRAWING__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__whycode_interfaces__srv__SetDrawing_Request __attribute__((deprecated))
#else
# define DEPRECATED__whycode_interfaces__srv__SetDrawing_Request __declspec(deprecated)
#endif

namespace whycode_interfaces
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SetDrawing_Request_
{
  using Type = SetDrawing_Request_<ContainerAllocator>;

  explicit SetDrawing_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->draw_coords = false;
      this->draw_segments = false;
    }
  }

  explicit SetDrawing_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->draw_coords = false;
      this->draw_segments = false;
    }
  }

  // field types and members
  using _draw_coords_type =
    bool;
  _draw_coords_type draw_coords;
  using _draw_segments_type =
    bool;
  _draw_segments_type draw_segments;

  // setters for named parameter idiom
  Type & set__draw_coords(
    const bool & _arg)
  {
    this->draw_coords = _arg;
    return *this;
  }
  Type & set__draw_segments(
    const bool & _arg)
  {
    this->draw_segments = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    whycode_interfaces::srv::SetDrawing_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const whycode_interfaces::srv::SetDrawing_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<whycode_interfaces::srv::SetDrawing_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<whycode_interfaces::srv::SetDrawing_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      whycode_interfaces::srv::SetDrawing_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<whycode_interfaces::srv::SetDrawing_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      whycode_interfaces::srv::SetDrawing_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<whycode_interfaces::srv::SetDrawing_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<whycode_interfaces::srv::SetDrawing_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<whycode_interfaces::srv::SetDrawing_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__whycode_interfaces__srv__SetDrawing_Request
    std::shared_ptr<whycode_interfaces::srv::SetDrawing_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__whycode_interfaces__srv__SetDrawing_Request
    std::shared_ptr<whycode_interfaces::srv::SetDrawing_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SetDrawing_Request_ & other) const
  {
    if (this->draw_coords != other.draw_coords) {
      return false;
    }
    if (this->draw_segments != other.draw_segments) {
      return false;
    }
    return true;
  }
  bool operator!=(const SetDrawing_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SetDrawing_Request_

// alias to use template instance with default allocator
using SetDrawing_Request =
  whycode_interfaces::srv::SetDrawing_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace whycode_interfaces


#ifndef _WIN32
# define DEPRECATED__whycode_interfaces__srv__SetDrawing_Response __attribute__((deprecated))
#else
# define DEPRECATED__whycode_interfaces__srv__SetDrawing_Response __declspec(deprecated)
#endif

namespace whycode_interfaces
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SetDrawing_Response_
{
  using Type = SetDrawing_Response_<ContainerAllocator>;

  explicit SetDrawing_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
    }
  }

  explicit SetDrawing_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
    }
  }

  // field types and members
  using _success_type =
    bool;
  _success_type success;

  // setters for named parameter idiom
  Type & set__success(
    const bool & _arg)
  {
    this->success = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    whycode_interfaces::srv::SetDrawing_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const whycode_interfaces::srv::SetDrawing_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<whycode_interfaces::srv::SetDrawing_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<whycode_interfaces::srv::SetDrawing_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      whycode_interfaces::srv::SetDrawing_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<whycode_interfaces::srv::SetDrawing_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      whycode_interfaces::srv::SetDrawing_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<whycode_interfaces::srv::SetDrawing_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<whycode_interfaces::srv::SetDrawing_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<whycode_interfaces::srv::SetDrawing_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__whycode_interfaces__srv__SetDrawing_Response
    std::shared_ptr<whycode_interfaces::srv::SetDrawing_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__whycode_interfaces__srv__SetDrawing_Response
    std::shared_ptr<whycode_interfaces::srv::SetDrawing_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SetDrawing_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    return true;
  }
  bool operator!=(const SetDrawing_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SetDrawing_Response_

// alias to use template instance with default allocator
using SetDrawing_Response =
  whycode_interfaces::srv::SetDrawing_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace whycode_interfaces

namespace whycode_interfaces
{

namespace srv
{

struct SetDrawing
{
  using Request = whycode_interfaces::srv::SetDrawing_Request;
  using Response = whycode_interfaces::srv::SetDrawing_Response;
};

}  // namespace srv

}  // namespace whycode_interfaces

#endif  // WHYCODE_INTERFACES__SRV__DETAIL__SET_DRAWING__STRUCT_HPP_
