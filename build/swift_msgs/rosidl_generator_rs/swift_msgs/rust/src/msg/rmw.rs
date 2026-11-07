#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "swift_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__swift_msgs__msg__SwiftMsgs() -> *const std::ffi::c_void;
}

#[link(name = "swift_msgs__rosidl_generator_c")]
extern "C" {
    fn swift_msgs__msg__SwiftMsgs__init(msg: *mut SwiftMsgs) -> bool;
    fn swift_msgs__msg__SwiftMsgs__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SwiftMsgs>, size: usize) -> bool;
    fn swift_msgs__msg__SwiftMsgs__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SwiftMsgs>);
    fn swift_msgs__msg__SwiftMsgs__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SwiftMsgs>, out_seq: *mut rosidl_runtime_rs::Sequence<SwiftMsgs>) -> bool;
}

// Corresponds to swift_msgs__msg__SwiftMsgs
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwiftMsgs {

    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_roll: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_pitch: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_yaw: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_throttle: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_aux1: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_aux2: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_aux3: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_aux4: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub drone_index: i64,

}



impl Default for SwiftMsgs {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !swift_msgs__msg__SwiftMsgs__init(&mut msg as *mut _) {
        panic!("Call to swift_msgs__msg__SwiftMsgs__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SwiftMsgs {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { swift_msgs__msg__SwiftMsgs__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { swift_msgs__msg__SwiftMsgs__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { swift_msgs__msg__SwiftMsgs__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SwiftMsgs {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SwiftMsgs where Self: Sized {
  const TYPE_NAME: &'static str = "swift_msgs/msg/SwiftMsgs";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__swift_msgs__msg__SwiftMsgs() }
  }
}


