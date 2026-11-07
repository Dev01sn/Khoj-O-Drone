#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "error_msg__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__error_msg__msg__Error() -> *const std::ffi::c_void;
}

#[link(name = "error_msg__rosidl_generator_c")]
extern "C" {
    fn error_msg__msg__Error__init(msg: *mut Error) -> bool;
    fn error_msg__msg__Error__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Error>, size: usize) -> bool;
    fn error_msg__msg__Error__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Error>);
    fn error_msg__msg__Error__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Error>, out_seq: *mut rosidl_runtime_rs::Sequence<Error>) -> bool;
}

// Corresponds to error_msg__msg__Error
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Error {

    // This member is not documented.
    #[allow(missing_docs)]
    pub roll_error: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pitch_error: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub throttle_error: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_error: f32,

}



impl Default for Error {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !error_msg__msg__Error__init(&mut msg as *mut _) {
        panic!("Call to error_msg__msg__Error__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Error {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { error_msg__msg__Error__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { error_msg__msg__Error__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { error_msg__msg__Error__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Error {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Error where Self: Sized {
  const TYPE_NAME: &'static str = "error_msg/msg/Error";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__error_msg__msg__Error() }
  }
}


