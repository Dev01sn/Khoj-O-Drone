#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__msg__Marker() -> *const std::ffi::c_void;
}

#[link(name = "whycode_interfaces__rosidl_generator_c")]
extern "C" {
    fn whycode_interfaces__msg__Marker__init(msg: *mut Marker) -> bool;
    fn whycode_interfaces__msg__Marker__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Marker>, size: usize) -> bool;
    fn whycode_interfaces__msg__Marker__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Marker>);
    fn whycode_interfaces__msg__Marker__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Marker>, out_seq: *mut rosidl_runtime_rs::Sequence<Marker>) -> bool;
}

// Corresponds to whycode_interfaces__msg__Marker
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Marker {
    /// ID
    pub id: i8,

    /// in pixels
    pub size: i32,

    /// camera coordinates
    pub u: f32,

    /// camera coordinates
    pub v: f32,

    /// rotation around surface normal
    pub angle: f32,

    /// pose
    pub position: geometry_msgs::msg::rmw::Pose,

    /// euler angles roll, pitch, yaw
    pub rotation: geometry_msgs::msg::rmw::Vector3,

}



impl Default for Marker {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !whycode_interfaces__msg__Marker__init(&mut msg as *mut _) {
        panic!("Call to whycode_interfaces__msg__Marker__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Marker {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__msg__Marker__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__msg__Marker__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__msg__Marker__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Marker {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Marker where Self: Sized {
  const TYPE_NAME: &'static str = "whycode_interfaces/msg/Marker";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__msg__Marker() }
  }
}


#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__msg__MarkerArray() -> *const std::ffi::c_void;
}

#[link(name = "whycode_interfaces__rosidl_generator_c")]
extern "C" {
    fn whycode_interfaces__msg__MarkerArray__init(msg: *mut MarkerArray) -> bool;
    fn whycode_interfaces__msg__MarkerArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MarkerArray>, size: usize) -> bool;
    fn whycode_interfaces__msg__MarkerArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MarkerArray>);
    fn whycode_interfaces__msg__MarkerArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MarkerArray>, out_seq: *mut rosidl_runtime_rs::Sequence<MarkerArray>) -> bool;
}

// Corresponds to whycode_interfaces__msg__MarkerArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MarkerArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub markers: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Marker>,

}



impl Default for MarkerArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !whycode_interfaces__msg__MarkerArray__init(&mut msg as *mut _) {
        panic!("Call to whycode_interfaces__msg__MarkerArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MarkerArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__msg__MarkerArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__msg__MarkerArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__msg__MarkerArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MarkerArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MarkerArray where Self: Sized {
  const TYPE_NAME: &'static str = "whycode_interfaces/msg/MarkerArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__msg__MarkerArray() }
  }
}


