#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "controller_msg__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__controller_msg__msg__PIDTune() -> *const std::ffi::c_void;
}

#[link(name = "controller_msg__rosidl_generator_c")]
extern "C" {
    fn controller_msg__msg__PIDTune__init(msg: *mut PIDTune) -> bool;
    fn controller_msg__msg__PIDTune__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PIDTune>, size: usize) -> bool;
    fn controller_msg__msg__PIDTune__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PIDTune>);
    fn controller_msg__msg__PIDTune__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PIDTune>, out_seq: *mut rosidl_runtime_rs::Sequence<PIDTune>) -> bool;
}

// Corresponds to controller_msg__msg__PIDTune
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PIDTune {

    // This member is not documented.
    #[allow(missing_docs)]
    pub kp: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ki: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub kd: f32,

}



impl Default for PIDTune {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !controller_msg__msg__PIDTune__init(&mut msg as *mut _) {
        panic!("Call to controller_msg__msg__PIDTune__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PIDTune {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_msg__msg__PIDTune__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_msg__msg__PIDTune__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { controller_msg__msg__PIDTune__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PIDTune {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PIDTune where Self: Sized {
  const TYPE_NAME: &'static str = "controller_msg/msg/PIDTune";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__controller_msg__msg__PIDTune() }
  }
}


