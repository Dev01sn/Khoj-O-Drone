// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from whycode_interfaces:msg/Marker.idl
// generated code does not contain a copyright notice

#ifndef WHYCODE_INTERFACES__MSG__DETAIL__MARKER__STRUCT_HPP_
#define WHYCODE_INTERFACES__MSG__DETAIL__MARKER__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'position'
#include "geometry_msgs/msg/detail/pose__struct.hpp"
// Member 'rotation'
#include "geometry_msgs/msg/detail/vector3__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__whycode_interfaces__msg__Marker __attribute__((deprecated))
#else
# define DEPRECATED__whycode_interfaces__msg__Marker __declspec(deprecated)
#endif

namespace whycode_interfaces
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct Marker_
{
  using Type = Marker_<ContainerAllocator>;

  explicit Marker_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : position(_init),
    rotation(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->id = 0;
      this->size = 0l;
      this->u = 0.0f;
      this->v = 0.0f;
      this->angle = 0.0f;
    }
  }

  explicit Marker_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : position(_alloc, _init),
    rotation(_alloc, _init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->id = 0;
      this->size = 0l;
      this->u = 0.0f;
      this->v = 0.0f;
      this->angle = 0.0f;
    }
  }

  // field types and members
  using _id_type =
    int8_t;
  _id_type id;
  using _size_type =
    int32_t;
  _size_type size;
  using _u_type =
    float;
  _u_type u;
  using _v_type =
    float;
  _v_type v;
  using _angle_type =
    float;
  _angle_type angle;
  using _position_type =
    geometry_msgs::msg::Pose_<ContainerAllocator>;
  _position_type position;
  using _rotation_type =
    geometry_msgs::msg::Vector3_<ContainerAllocator>;
  _rotation_type rotation;

  // setters for named parameter idiom
  Type & set__id(
    const int8_t & _arg)
  {
    this->id = _arg;
    return *this;
  }
  Type & set__size(
    const int32_t & _arg)
  {
    this->size = _arg;
    return *this;
  }
  Type & set__u(
    const float & _arg)
  {
    this->u = _arg;
    return *this;
  }
  Type & set__v(
    const float & _arg)
  {
    this->v = _arg;
    return *this;
  }
  Type & set__angle(
    const float & _arg)
  {
    this->angle = _arg;
    return *this;
  }
  Type & set__position(
    const geometry_msgs::msg::Pose_<ContainerAllocator> & _arg)
  {
    this->position = _arg;
    return *this;
  }
  Type & set__rotation(
    const geometry_msgs::msg::Vector3_<ContainerAllocator> & _arg)
  {
    this->rotation = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    whycode_interfaces::msg::Marker_<ContainerAllocator> *;
  using ConstRawPtr =
    const whycode_interfaces::msg::Marker_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<whycode_interfaces::msg::Marker_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<whycode_interfaces::msg::Marker_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      whycode_interfaces::msg::Marker_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<whycode_interfaces::msg::Marker_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      whycode_interfaces::msg::Marker_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<whycode_interfaces::msg::Marker_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<whycode_interfaces::msg::Marker_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<whycode_interfaces::msg::Marker_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__whycode_interfaces__msg__Marker
    std::shared_ptr<whycode_interfaces::msg::Marker_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__whycode_interfaces__msg__Marker
    std::shared_ptr<whycode_interfaces::msg::Marker_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const Marker_ & other) const
  {
    if (this->id != other.id) {
      return false;
    }
    if (this->size != other.size) {
      return false;
    }
    if (this->u != other.u) {
      return false;
    }
    if (this->v != other.v) {
      return false;
    }
    if (this->angle != other.angle) {
      return false;
    }
    if (this->position != other.position) {
      return false;
    }
    if (this->rotation != other.rotation) {
      return false;
    }
    return true;
  }
  bool operator!=(const Marker_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct Marker_

// alias to use template instance with default allocator
using Marker =
  whycode_interfaces::msg::Marker_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace whycode_interfaces

#endif  // WHYCODE_INTERFACES__MSG__DETAIL__MARKER__STRUCT_HPP_
