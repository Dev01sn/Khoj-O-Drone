#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__Point2D() -> *const std::ffi::c_void;
}

#[link(name = "mav_planning_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_planning_msgs__msg__Point2D__init(msg: *mut Point2D) -> bool;
    fn mav_planning_msgs__msg__Point2D__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Point2D>, size: usize) -> bool;
    fn mav_planning_msgs__msg__Point2D__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Point2D>);
    fn mav_planning_msgs__msg__Point2D__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Point2D>, out_seq: *mut rosidl_runtime_rs::Sequence<Point2D>) -> bool;
}

// Corresponds to mav_planning_msgs__msg__Point2D
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This contains the position of a 2D point.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Point2D {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f64,

}



impl Default for Point2D {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_planning_msgs__msg__Point2D__init(&mut msg as *mut _) {
        panic!("Call to mav_planning_msgs__msg__Point2D__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Point2D {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__Point2D__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__Point2D__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__Point2D__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Point2D {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Point2D where Self: Sized {
  const TYPE_NAME: &'static str = "mav_planning_msgs/msg/Point2D";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__Point2D() }
  }
}


#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__PointCloudWithPose() -> *const std::ffi::c_void;
}

#[link(name = "mav_planning_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_planning_msgs__msg__PointCloudWithPose__init(msg: *mut PointCloudWithPose) -> bool;
    fn mav_planning_msgs__msg__PointCloudWithPose__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PointCloudWithPose>, size: usize) -> bool;
    fn mav_planning_msgs__msg__PointCloudWithPose__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PointCloudWithPose>);
    fn mav_planning_msgs__msg__PointCloudWithPose__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PointCloudWithPose>, out_seq: *mut rosidl_runtime_rs::Sequence<PointCloudWithPose>) -> bool;
}

// Corresponds to mav_planning_msgs__msg__PointCloudWithPose
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointCloudWithPose {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sensor_pose: geometry_msgs::msg::rmw::TransformStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cloud_in_sensor_frame: sensor_msgs::msg::rmw::PointCloud2,

}



impl Default for PointCloudWithPose {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_planning_msgs__msg__PointCloudWithPose__init(&mut msg as *mut _) {
        panic!("Call to mav_planning_msgs__msg__PointCloudWithPose__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PointCloudWithPose {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PointCloudWithPose__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PointCloudWithPose__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PointCloudWithPose__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PointCloudWithPose {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PointCloudWithPose where Self: Sized {
  const TYPE_NAME: &'static str = "mav_planning_msgs/msg/PointCloudWithPose";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__PointCloudWithPose() }
  }
}


#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__Polygon2D() -> *const std::ffi::c_void;
}

#[link(name = "mav_planning_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_planning_msgs__msg__Polygon2D__init(msg: *mut Polygon2D) -> bool;
    fn mav_planning_msgs__msg__Polygon2D__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Polygon2D>, size: usize) -> bool;
    fn mav_planning_msgs__msg__Polygon2D__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Polygon2D>);
    fn mav_planning_msgs__msg__Polygon2D__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Polygon2D>, out_seq: *mut rosidl_runtime_rs::Sequence<Polygon2D>) -> bool;
}

// Corresponds to mav_planning_msgs__msg__Polygon2D
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A specification of a 2D polygon where the first and last points are assumed to be connected.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Polygon2D {

    // This member is not documented.
    #[allow(missing_docs)]
    pub points: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Point2D>,

}



impl Default for Polygon2D {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_planning_msgs__msg__Polygon2D__init(&mut msg as *mut _) {
        panic!("Call to mav_planning_msgs__msg__Polygon2D__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Polygon2D {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__Polygon2D__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__Polygon2D__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__Polygon2D__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Polygon2D {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Polygon2D where Self: Sized {
  const TYPE_NAME: &'static str = "mav_planning_msgs/msg/Polygon2D";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__Polygon2D() }
  }
}


#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__PolygonWithHoles() -> *const std::ffi::c_void;
}

#[link(name = "mav_planning_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_planning_msgs__msg__PolygonWithHoles__init(msg: *mut PolygonWithHoles) -> bool;
    fn mav_planning_msgs__msg__PolygonWithHoles__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PolygonWithHoles>, size: usize) -> bool;
    fn mav_planning_msgs__msg__PolygonWithHoles__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PolygonWithHoles>);
    fn mav_planning_msgs__msg__PolygonWithHoles__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PolygonWithHoles>, out_seq: *mut rosidl_runtime_rs::Sequence<PolygonWithHoles>) -> bool;
}

// Corresponds to mav_planning_msgs__msg__PolygonWithHoles
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A message to define a 2D polygon with holes.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolygonWithHoles {

    // This member is not documented.
    #[allow(missing_docs)]
    pub hull: super::super::msg::rmw::Polygon2D,


    // This member is not documented.
    #[allow(missing_docs)]
    pub holes: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Polygon2D>,

}



impl Default for PolygonWithHoles {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_planning_msgs__msg__PolygonWithHoles__init(&mut msg as *mut _) {
        panic!("Call to mav_planning_msgs__msg__PolygonWithHoles__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PolygonWithHoles {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolygonWithHoles__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolygonWithHoles__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolygonWithHoles__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PolygonWithHoles {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PolygonWithHoles where Self: Sized {
  const TYPE_NAME: &'static str = "mav_planning_msgs/msg/PolygonWithHoles";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__PolygonWithHoles() }
  }
}


#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__PolygonWithHolesStamped() -> *const std::ffi::c_void;
}

#[link(name = "mav_planning_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_planning_msgs__msg__PolygonWithHolesStamped__init(msg: *mut PolygonWithHolesStamped) -> bool;
    fn mav_planning_msgs__msg__PolygonWithHolesStamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PolygonWithHolesStamped>, size: usize) -> bool;
    fn mav_planning_msgs__msg__PolygonWithHolesStamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PolygonWithHolesStamped>);
    fn mav_planning_msgs__msg__PolygonWithHolesStamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PolygonWithHolesStamped>, out_seq: *mut rosidl_runtime_rs::Sequence<PolygonWithHolesStamped>) -> bool;
}

// Corresponds to mav_planning_msgs__msg__PolygonWithHolesStamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A message to define a 2D polygon with holes, stamp, and altitude above ground.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolygonWithHolesStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub altitude: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub polygon: super::super::msg::rmw::PolygonWithHoles,

}



impl Default for PolygonWithHolesStamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_planning_msgs__msg__PolygonWithHolesStamped__init(&mut msg as *mut _) {
        panic!("Call to mav_planning_msgs__msg__PolygonWithHolesStamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PolygonWithHolesStamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolygonWithHolesStamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolygonWithHolesStamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolygonWithHolesStamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PolygonWithHolesStamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PolygonWithHolesStamped where Self: Sized {
  const TYPE_NAME: &'static str = "mav_planning_msgs/msg/PolygonWithHolesStamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__PolygonWithHolesStamped() }
  }
}


#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__PolynomialSegment() -> *const std::ffi::c_void;
}

#[link(name = "mav_planning_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_planning_msgs__msg__PolynomialSegment__init(msg: *mut PolynomialSegment) -> bool;
    fn mav_planning_msgs__msg__PolynomialSegment__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PolynomialSegment>, size: usize) -> bool;
    fn mav_planning_msgs__msg__PolynomialSegment__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PolynomialSegment>);
    fn mav_planning_msgs__msg__PolynomialSegment__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PolynomialSegment>, out_seq: *mut rosidl_runtime_rs::Sequence<PolynomialSegment>) -> bool;
}

// Corresponds to mav_planning_msgs__msg__PolynomialSegment
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolynomialSegment {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// order of the polynomial + 1, should match size of x[]
    pub num_coeffs: i32,

    /// duration of the segment
    pub segment_time: builtin_interfaces::msg::rmw::Duration,

    /// coefficients for the x-axis, INCREASING order
    pub x: rosidl_runtime_rs::Sequence<f64>,

    /// coefficients for the y-axis, INCREASING order
    pub y: rosidl_runtime_rs::Sequence<f64>,

    /// coefficients for the z-axis, INCREASING order
    pub z: rosidl_runtime_rs::Sequence<f64>,

    /// coefficients for the rotation x-vector, INCREASING order
    pub rx: rosidl_runtime_rs::Sequence<f64>,

    /// coefficients for the rotation y-vector, INCREASING order
    pub ry: rosidl_runtime_rs::Sequence<f64>,

    /// coefficients for the rotation z-vector, INCREASING order
    pub rz: rosidl_runtime_rs::Sequence<f64>,

    /// For backwards compatibility with underactuated (4DOF) commands):
    /// coefficients for the yaw, INCREASING order
    pub yaw: rosidl_runtime_rs::Sequence<f64>,

}



impl Default for PolynomialSegment {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_planning_msgs__msg__PolynomialSegment__init(&mut msg as *mut _) {
        panic!("Call to mav_planning_msgs__msg__PolynomialSegment__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PolynomialSegment {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolynomialSegment__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolynomialSegment__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolynomialSegment__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PolynomialSegment {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PolynomialSegment where Self: Sized {
  const TYPE_NAME: &'static str = "mav_planning_msgs/msg/PolynomialSegment";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__PolynomialSegment() }
  }
}


#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__PolynomialSegment4D() -> *const std::ffi::c_void;
}

#[link(name = "mav_planning_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_planning_msgs__msg__PolynomialSegment4D__init(msg: *mut PolynomialSegment4D) -> bool;
    fn mav_planning_msgs__msg__PolynomialSegment4D__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PolynomialSegment4D>, size: usize) -> bool;
    fn mav_planning_msgs__msg__PolynomialSegment4D__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PolynomialSegment4D>);
    fn mav_planning_msgs__msg__PolynomialSegment4D__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PolynomialSegment4D>, out_seq: *mut rosidl_runtime_rs::Sequence<PolynomialSegment4D>) -> bool;
}

// Corresponds to mav_planning_msgs__msg__PolynomialSegment4D
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolynomialSegment4D {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// order of the polynomial + 1, should match size of x[]
    pub num_coeffs: i32,

    /// duration of the segment
    pub segment_time: builtin_interfaces::msg::rmw::Duration,

    /// coefficients for the x-axis, INCREASING order
    pub x: rosidl_runtime_rs::Sequence<f64>,

    /// coefficients for the y-axis, INCREASING order
    pub y: rosidl_runtime_rs::Sequence<f64>,

    /// coefficients for the z-axis, INCREASING order
    pub z: rosidl_runtime_rs::Sequence<f64>,

    /// coefficients for the yaw, INCREASING order
    pub yaw: rosidl_runtime_rs::Sequence<f64>,

}



impl Default for PolynomialSegment4D {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_planning_msgs__msg__PolynomialSegment4D__init(&mut msg as *mut _) {
        panic!("Call to mav_planning_msgs__msg__PolynomialSegment4D__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PolynomialSegment4D {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolynomialSegment4D__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolynomialSegment4D__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolynomialSegment4D__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PolynomialSegment4D {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PolynomialSegment4D where Self: Sized {
  const TYPE_NAME: &'static str = "mav_planning_msgs/msg/PolynomialSegment4D";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__PolynomialSegment4D() }
  }
}


#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__PolynomialTrajectory() -> *const std::ffi::c_void;
}

#[link(name = "mav_planning_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_planning_msgs__msg__PolynomialTrajectory__init(msg: *mut PolynomialTrajectory) -> bool;
    fn mav_planning_msgs__msg__PolynomialTrajectory__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PolynomialTrajectory>, size: usize) -> bool;
    fn mav_planning_msgs__msg__PolynomialTrajectory__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PolynomialTrajectory>);
    fn mav_planning_msgs__msg__PolynomialTrajectory__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PolynomialTrajectory>, out_seq: *mut rosidl_runtime_rs::Sequence<PolynomialTrajectory>) -> bool;
}

// Corresponds to mav_planning_msgs__msg__PolynomialTrajectory
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolynomialTrajectory {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub segments: rosidl_runtime_rs::Sequence<super::super::msg::rmw::PolynomialSegment>,

}



impl Default for PolynomialTrajectory {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_planning_msgs__msg__PolynomialTrajectory__init(&mut msg as *mut _) {
        panic!("Call to mav_planning_msgs__msg__PolynomialTrajectory__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PolynomialTrajectory {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolynomialTrajectory__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolynomialTrajectory__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolynomialTrajectory__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PolynomialTrajectory {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PolynomialTrajectory where Self: Sized {
  const TYPE_NAME: &'static str = "mav_planning_msgs/msg/PolynomialTrajectory";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__PolynomialTrajectory() }
  }
}


#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__PolynomialTrajectory4D() -> *const std::ffi::c_void;
}

#[link(name = "mav_planning_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_planning_msgs__msg__PolynomialTrajectory4D__init(msg: *mut PolynomialTrajectory4D) -> bool;
    fn mav_planning_msgs__msg__PolynomialTrajectory4D__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PolynomialTrajectory4D>, size: usize) -> bool;
    fn mav_planning_msgs__msg__PolynomialTrajectory4D__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PolynomialTrajectory4D>);
    fn mav_planning_msgs__msg__PolynomialTrajectory4D__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PolynomialTrajectory4D>, out_seq: *mut rosidl_runtime_rs::Sequence<PolynomialTrajectory4D>) -> bool;
}

// Corresponds to mav_planning_msgs__msg__PolynomialTrajectory4D
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolynomialTrajectory4D {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub segments: rosidl_runtime_rs::Sequence<super::super::msg::rmw::PolynomialSegment4D>,

}



impl Default for PolynomialTrajectory4D {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_planning_msgs__msg__PolynomialTrajectory4D__init(&mut msg as *mut _) {
        panic!("Call to mav_planning_msgs__msg__PolynomialTrajectory4D__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PolynomialTrajectory4D {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolynomialTrajectory4D__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolynomialTrajectory4D__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__msg__PolynomialTrajectory4D__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PolynomialTrajectory4D {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PolynomialTrajectory4D where Self: Sized {
  const TYPE_NAME: &'static str = "mav_planning_msgs/msg/PolynomialTrajectory4D";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__msg__PolynomialTrajectory4D() }
  }
}


