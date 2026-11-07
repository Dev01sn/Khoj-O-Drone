#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SelectMarker_Request() -> *const std::ffi::c_void;
}

#[link(name = "whycode_interfaces__rosidl_generator_c")]
extern "C" {
    fn whycode_interfaces__srv__SelectMarker_Request__init(msg: *mut SelectMarker_Request) -> bool;
    fn whycode_interfaces__srv__SelectMarker_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SelectMarker_Request>, size: usize) -> bool;
    fn whycode_interfaces__srv__SelectMarker_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SelectMarker_Request>);
    fn whycode_interfaces__srv__SelectMarker_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SelectMarker_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SelectMarker_Request>) -> bool;
}

// Corresponds to whycode_interfaces__srv__SelectMarker_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SelectMarker_Request {
    /// x,y of seleceted marker in image coordinates
    pub point: geometry_msgs::msg::rmw::Point,

}



impl Default for SelectMarker_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !whycode_interfaces__srv__SelectMarker_Request__init(&mut msg as *mut _) {
        panic!("Call to whycode_interfaces__srv__SelectMarker_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SelectMarker_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SelectMarker_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SelectMarker_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SelectMarker_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SelectMarker_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SelectMarker_Request where Self: Sized {
  const TYPE_NAME: &'static str = "whycode_interfaces/srv/SelectMarker_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SelectMarker_Request() }
  }
}


#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SelectMarker_Response() -> *const std::ffi::c_void;
}

#[link(name = "whycode_interfaces__rosidl_generator_c")]
extern "C" {
    fn whycode_interfaces__srv__SelectMarker_Response__init(msg: *mut SelectMarker_Response) -> bool;
    fn whycode_interfaces__srv__SelectMarker_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SelectMarker_Response>, size: usize) -> bool;
    fn whycode_interfaces__srv__SelectMarker_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SelectMarker_Response>);
    fn whycode_interfaces__srv__SelectMarker_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SelectMarker_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SelectMarker_Response>) -> bool;
}

// Corresponds to whycode_interfaces__srv__SelectMarker_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SelectMarker_Response {
    /// bool feedback
    pub success: bool,

}



impl Default for SelectMarker_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !whycode_interfaces__srv__SelectMarker_Response__init(&mut msg as *mut _) {
        panic!("Call to whycode_interfaces__srv__SelectMarker_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SelectMarker_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SelectMarker_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SelectMarker_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SelectMarker_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SelectMarker_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SelectMarker_Response where Self: Sized {
  const TYPE_NAME: &'static str = "whycode_interfaces/srv/SelectMarker_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SelectMarker_Response() }
  }
}


#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetCalibMethod_Request() -> *const std::ffi::c_void;
}

#[link(name = "whycode_interfaces__rosidl_generator_c")]
extern "C" {
    fn whycode_interfaces__srv__SetCalibMethod_Request__init(msg: *mut SetCalibMethod_Request) -> bool;
    fn whycode_interfaces__srv__SetCalibMethod_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCalibMethod_Request>, size: usize) -> bool;
    fn whycode_interfaces__srv__SetCalibMethod_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCalibMethod_Request>);
    fn whycode_interfaces__srv__SetCalibMethod_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCalibMethod_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCalibMethod_Request>) -> bool;
}

// Corresponds to whycode_interfaces__srv__SetCalibMethod_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCalibMethod_Request {
    /// 0 = autocalibration
    /// 1 = manual calibration
    pub method: i8,

}



impl Default for SetCalibMethod_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !whycode_interfaces__srv__SetCalibMethod_Request__init(&mut msg as *mut _) {
        panic!("Call to whycode_interfaces__srv__SetCalibMethod_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCalibMethod_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCalibMethod_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCalibMethod_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCalibMethod_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCalibMethod_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCalibMethod_Request where Self: Sized {
  const TYPE_NAME: &'static str = "whycode_interfaces/srv/SetCalibMethod_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetCalibMethod_Request() }
  }
}


#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetCalibMethod_Response() -> *const std::ffi::c_void;
}

#[link(name = "whycode_interfaces__rosidl_generator_c")]
extern "C" {
    fn whycode_interfaces__srv__SetCalibMethod_Response__init(msg: *mut SetCalibMethod_Response) -> bool;
    fn whycode_interfaces__srv__SetCalibMethod_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCalibMethod_Response>, size: usize) -> bool;
    fn whycode_interfaces__srv__SetCalibMethod_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCalibMethod_Response>);
    fn whycode_interfaces__srv__SetCalibMethod_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCalibMethod_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCalibMethod_Response>) -> bool;
}

// Corresponds to whycode_interfaces__srv__SetCalibMethod_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCalibMethod_Response {
    /// bool feedback
    pub success: bool,

    /// information message
    pub msg: rosidl_runtime_rs::String,

}



impl Default for SetCalibMethod_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !whycode_interfaces__srv__SetCalibMethod_Response__init(&mut msg as *mut _) {
        panic!("Call to whycode_interfaces__srv__SetCalibMethod_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCalibMethod_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCalibMethod_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCalibMethod_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCalibMethod_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCalibMethod_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCalibMethod_Response where Self: Sized {
  const TYPE_NAME: &'static str = "whycode_interfaces/srv/SetCalibMethod_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetCalibMethod_Response() }
  }
}


#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetCalibPath_Request() -> *const std::ffi::c_void;
}

#[link(name = "whycode_interfaces__rosidl_generator_c")]
extern "C" {
    fn whycode_interfaces__srv__SetCalibPath_Request__init(msg: *mut SetCalibPath_Request) -> bool;
    fn whycode_interfaces__srv__SetCalibPath_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCalibPath_Request>, size: usize) -> bool;
    fn whycode_interfaces__srv__SetCalibPath_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCalibPath_Request>);
    fn whycode_interfaces__srv__SetCalibPath_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCalibPath_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCalibPath_Request>) -> bool;
}

// Corresponds to whycode_interfaces__srv__SetCalibPath_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCalibPath_Request {
    /// either "save" or "load"
    pub action: rosidl_runtime_rs::String,

    /// path to yaml file
    pub path: rosidl_runtime_rs::String,

}



impl Default for SetCalibPath_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !whycode_interfaces__srv__SetCalibPath_Request__init(&mut msg as *mut _) {
        panic!("Call to whycode_interfaces__srv__SetCalibPath_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCalibPath_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCalibPath_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCalibPath_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCalibPath_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCalibPath_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCalibPath_Request where Self: Sized {
  const TYPE_NAME: &'static str = "whycode_interfaces/srv/SetCalibPath_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetCalibPath_Request() }
  }
}


#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetCalibPath_Response() -> *const std::ffi::c_void;
}

#[link(name = "whycode_interfaces__rosidl_generator_c")]
extern "C" {
    fn whycode_interfaces__srv__SetCalibPath_Response__init(msg: *mut SetCalibPath_Response) -> bool;
    fn whycode_interfaces__srv__SetCalibPath_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCalibPath_Response>, size: usize) -> bool;
    fn whycode_interfaces__srv__SetCalibPath_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCalibPath_Response>);
    fn whycode_interfaces__srv__SetCalibPath_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCalibPath_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCalibPath_Response>) -> bool;
}

// Corresponds to whycode_interfaces__srv__SetCalibPath_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCalibPath_Response {
    /// bool feedback
    pub success: bool,

    /// information message
    pub msg: rosidl_runtime_rs::String,

}



impl Default for SetCalibPath_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !whycode_interfaces__srv__SetCalibPath_Response__init(&mut msg as *mut _) {
        panic!("Call to whycode_interfaces__srv__SetCalibPath_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCalibPath_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCalibPath_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCalibPath_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCalibPath_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCalibPath_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCalibPath_Response where Self: Sized {
  const TYPE_NAME: &'static str = "whycode_interfaces/srv/SetCalibPath_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetCalibPath_Response() }
  }
}


#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetCoords_Request() -> *const std::ffi::c_void;
}

#[link(name = "whycode_interfaces__rosidl_generator_c")]
extern "C" {
    fn whycode_interfaces__srv__SetCoords_Request__init(msg: *mut SetCoords_Request) -> bool;
    fn whycode_interfaces__srv__SetCoords_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCoords_Request>, size: usize) -> bool;
    fn whycode_interfaces__srv__SetCoords_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCoords_Request>);
    fn whycode_interfaces__srv__SetCoords_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCoords_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCoords_Request>) -> bool;
}

// Corresponds to whycode_interfaces__srv__SetCoords_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCoords_Request {
    /// 0 = camera coords
    /// 1 = 2D coords
    /// 2 = 3D coords
    pub coords: i8,

}



impl Default for SetCoords_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !whycode_interfaces__srv__SetCoords_Request__init(&mut msg as *mut _) {
        panic!("Call to whycode_interfaces__srv__SetCoords_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCoords_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCoords_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCoords_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCoords_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCoords_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCoords_Request where Self: Sized {
  const TYPE_NAME: &'static str = "whycode_interfaces/srv/SetCoords_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetCoords_Request() }
  }
}


#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetCoords_Response() -> *const std::ffi::c_void;
}

#[link(name = "whycode_interfaces__rosidl_generator_c")]
extern "C" {
    fn whycode_interfaces__srv__SetCoords_Response__init(msg: *mut SetCoords_Response) -> bool;
    fn whycode_interfaces__srv__SetCoords_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCoords_Response>, size: usize) -> bool;
    fn whycode_interfaces__srv__SetCoords_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCoords_Response>);
    fn whycode_interfaces__srv__SetCoords_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCoords_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCoords_Response>) -> bool;
}

// Corresponds to whycode_interfaces__srv__SetCoords_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCoords_Response {
    /// bool feedback
    pub success: bool,

    /// information message
    pub msg: rosidl_runtime_rs::String,

}



impl Default for SetCoords_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !whycode_interfaces__srv__SetCoords_Response__init(&mut msg as *mut _) {
        panic!("Call to whycode_interfaces__srv__SetCoords_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCoords_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCoords_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCoords_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetCoords_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCoords_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCoords_Response where Self: Sized {
  const TYPE_NAME: &'static str = "whycode_interfaces/srv/SetCoords_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetCoords_Response() }
  }
}


#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetDrawing_Request() -> *const std::ffi::c_void;
}

#[link(name = "whycode_interfaces__rosidl_generator_c")]
extern "C" {
    fn whycode_interfaces__srv__SetDrawing_Request__init(msg: *mut SetDrawing_Request) -> bool;
    fn whycode_interfaces__srv__SetDrawing_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetDrawing_Request>, size: usize) -> bool;
    fn whycode_interfaces__srv__SetDrawing_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetDrawing_Request>);
    fn whycode_interfaces__srv__SetDrawing_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetDrawing_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetDrawing_Request>) -> bool;
}

// Corresponds to whycode_interfaces__srv__SetDrawing_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetDrawing_Request {
    /// draw/hide coordinates
    pub draw_coords: bool,

    /// draw/hide segmentation outcome
    pub draw_segments: bool,

}



impl Default for SetDrawing_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !whycode_interfaces__srv__SetDrawing_Request__init(&mut msg as *mut _) {
        panic!("Call to whycode_interfaces__srv__SetDrawing_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetDrawing_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetDrawing_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetDrawing_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetDrawing_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetDrawing_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetDrawing_Request where Self: Sized {
  const TYPE_NAME: &'static str = "whycode_interfaces/srv/SetDrawing_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetDrawing_Request() }
  }
}


#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetDrawing_Response() -> *const std::ffi::c_void;
}

#[link(name = "whycode_interfaces__rosidl_generator_c")]
extern "C" {
    fn whycode_interfaces__srv__SetDrawing_Response__init(msg: *mut SetDrawing_Response) -> bool;
    fn whycode_interfaces__srv__SetDrawing_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetDrawing_Response>, size: usize) -> bool;
    fn whycode_interfaces__srv__SetDrawing_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetDrawing_Response>);
    fn whycode_interfaces__srv__SetDrawing_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetDrawing_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetDrawing_Response>) -> bool;
}

// Corresponds to whycode_interfaces__srv__SetDrawing_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetDrawing_Response {
    /// bool feedback
    pub success: bool,

}



impl Default for SetDrawing_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !whycode_interfaces__srv__SetDrawing_Response__init(&mut msg as *mut _) {
        panic!("Call to whycode_interfaces__srv__SetDrawing_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetDrawing_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetDrawing_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetDrawing_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__SetDrawing_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetDrawing_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetDrawing_Response where Self: Sized {
  const TYPE_NAME: &'static str = "whycode_interfaces/srv/SetDrawing_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__SetDrawing_Response() }
  }
}


#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__GetGuiSettings_Request() -> *const std::ffi::c_void;
}

#[link(name = "whycode_interfaces__rosidl_generator_c")]
extern "C" {
    fn whycode_interfaces__srv__GetGuiSettings_Request__init(msg: *mut GetGuiSettings_Request) -> bool;
    fn whycode_interfaces__srv__GetGuiSettings_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetGuiSettings_Request>, size: usize) -> bool;
    fn whycode_interfaces__srv__GetGuiSettings_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetGuiSettings_Request>);
    fn whycode_interfaces__srv__GetGuiSettings_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetGuiSettings_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetGuiSettings_Request>) -> bool;
}

// Corresponds to whycode_interfaces__srv__GetGuiSettings_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetGuiSettings_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetGuiSettings_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !whycode_interfaces__srv__GetGuiSettings_Request__init(&mut msg as *mut _) {
        panic!("Call to whycode_interfaces__srv__GetGuiSettings_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetGuiSettings_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__GetGuiSettings_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__GetGuiSettings_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__GetGuiSettings_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetGuiSettings_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetGuiSettings_Request where Self: Sized {
  const TYPE_NAME: &'static str = "whycode_interfaces/srv/GetGuiSettings_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__GetGuiSettings_Request() }
  }
}


#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__GetGuiSettings_Response() -> *const std::ffi::c_void;
}

#[link(name = "whycode_interfaces__rosidl_generator_c")]
extern "C" {
    fn whycode_interfaces__srv__GetGuiSettings_Response__init(msg: *mut GetGuiSettings_Response) -> bool;
    fn whycode_interfaces__srv__GetGuiSettings_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetGuiSettings_Response>, size: usize) -> bool;
    fn whycode_interfaces__srv__GetGuiSettings_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetGuiSettings_Response>);
    fn whycode_interfaces__srv__GetGuiSettings_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetGuiSettings_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetGuiSettings_Response>) -> bool;
}

// Corresponds to whycode_interfaces__srv__GetGuiSettings_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetGuiSettings_Response {
    /// draw/hide coordinates
    pub draw_coords: bool,

    /// draw/hide segmentation outcome
    pub draw_segments: bool,

    /// 0 = camera coords
    /// 1 = 2D coords
    /// 2 = 3D coords
    pub coords: i8,

}



impl Default for GetGuiSettings_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !whycode_interfaces__srv__GetGuiSettings_Response__init(&mut msg as *mut _) {
        panic!("Call to whycode_interfaces__srv__GetGuiSettings_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetGuiSettings_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__GetGuiSettings_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__GetGuiSettings_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { whycode_interfaces__srv__GetGuiSettings_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetGuiSettings_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetGuiSettings_Response where Self: Sized {
  const TYPE_NAME: &'static str = "whycode_interfaces/srv/GetGuiSettings_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__whycode_interfaces__srv__GetGuiSettings_Response() }
  }
}






#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__whycode_interfaces__srv__SelectMarker() -> *const std::ffi::c_void;
}

// Corresponds to whycode_interfaces__srv__SelectMarker
#[allow(missing_docs, non_camel_case_types)]
pub struct SelectMarker;

impl rosidl_runtime_rs::Service for SelectMarker {
    type Request = SelectMarker_Request;
    type Response = SelectMarker_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__whycode_interfaces__srv__SelectMarker() }
    }
}




#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__whycode_interfaces__srv__SetCalibMethod() -> *const std::ffi::c_void;
}

// Corresponds to whycode_interfaces__srv__SetCalibMethod
#[allow(missing_docs, non_camel_case_types)]
pub struct SetCalibMethod;

impl rosidl_runtime_rs::Service for SetCalibMethod {
    type Request = SetCalibMethod_Request;
    type Response = SetCalibMethod_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__whycode_interfaces__srv__SetCalibMethod() }
    }
}




#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__whycode_interfaces__srv__SetCalibPath() -> *const std::ffi::c_void;
}

// Corresponds to whycode_interfaces__srv__SetCalibPath
#[allow(missing_docs, non_camel_case_types)]
pub struct SetCalibPath;

impl rosidl_runtime_rs::Service for SetCalibPath {
    type Request = SetCalibPath_Request;
    type Response = SetCalibPath_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__whycode_interfaces__srv__SetCalibPath() }
    }
}




#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__whycode_interfaces__srv__SetCoords() -> *const std::ffi::c_void;
}

// Corresponds to whycode_interfaces__srv__SetCoords
#[allow(missing_docs, non_camel_case_types)]
pub struct SetCoords;

impl rosidl_runtime_rs::Service for SetCoords {
    type Request = SetCoords_Request;
    type Response = SetCoords_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__whycode_interfaces__srv__SetCoords() }
    }
}




#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__whycode_interfaces__srv__SetDrawing() -> *const std::ffi::c_void;
}

// Corresponds to whycode_interfaces__srv__SetDrawing
#[allow(missing_docs, non_camel_case_types)]
pub struct SetDrawing;

impl rosidl_runtime_rs::Service for SetDrawing {
    type Request = SetDrawing_Request;
    type Response = SetDrawing_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__whycode_interfaces__srv__SetDrawing() }
    }
}




#[link(name = "whycode_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__whycode_interfaces__srv__GetGuiSettings() -> *const std::ffi::c_void;
}

// Corresponds to whycode_interfaces__srv__GetGuiSettings
#[allow(missing_docs, non_camel_case_types)]
pub struct GetGuiSettings;

impl rosidl_runtime_rs::Service for GetGuiSettings {
    type Request = GetGuiSettings_Request;
    type Response = GetGuiSettings_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__whycode_interfaces__srv__GetGuiSettings() }
    }
}


