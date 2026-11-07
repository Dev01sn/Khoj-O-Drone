
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "waypoint_navigation__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_Goal() -> *const std::ffi::c_void;
}

#[link(name = "waypoint_navigation__rosidl_generator_c")]
extern "C" {
    fn waypoint_navigation__action__NavToWaypoint_Goal__init(msg: *mut NavToWaypoint_Goal) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_Goal>, size: usize) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_Goal>);
    fn waypoint_navigation__action__NavToWaypoint_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavToWaypoint_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_Goal>) -> bool;
}

// Corresponds to waypoint_navigation__action__NavToWaypoint_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub waypoint: geometry_msgs::msg::rmw::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub hover: bool,

}



impl Default for NavToWaypoint_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !waypoint_navigation__action__NavToWaypoint_Goal__init(&mut msg as *mut _) {
        panic!("Call to waypoint_navigation__action__NavToWaypoint_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavToWaypoint_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavToWaypoint_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "waypoint_navigation/action/NavToWaypoint_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_Goal() }
  }
}


#[link(name = "waypoint_navigation__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_Result() -> *const std::ffi::c_void;
}

#[link(name = "waypoint_navigation__rosidl_generator_c")]
extern "C" {
    fn waypoint_navigation__action__NavToWaypoint_Result__init(msg: *mut NavToWaypoint_Result) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_Result>, size: usize) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_Result>);
    fn waypoint_navigation__action__NavToWaypoint_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavToWaypoint_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_Result>) -> bool;
}

// Corresponds to waypoint_navigation__action__NavToWaypoint_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub hov_time: i32,

}



impl Default for NavToWaypoint_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !waypoint_navigation__action__NavToWaypoint_Result__init(&mut msg as *mut _) {
        panic!("Call to waypoint_navigation__action__NavToWaypoint_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavToWaypoint_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavToWaypoint_Result where Self: Sized {
  const TYPE_NAME: &'static str = "waypoint_navigation/action/NavToWaypoint_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_Result() }
  }
}


#[link(name = "waypoint_navigation__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "waypoint_navigation__rosidl_generator_c")]
extern "C" {
    fn waypoint_navigation__action__NavToWaypoint_Feedback__init(msg: *mut NavToWaypoint_Feedback) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_Feedback>, size: usize) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_Feedback>);
    fn waypoint_navigation__action__NavToWaypoint_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavToWaypoint_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_Feedback>) -> bool;
}

// Corresponds to waypoint_navigation__action__NavToWaypoint_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_waypoint: geometry_msgs::msg::rmw::PoseStamped,

}



impl Default for NavToWaypoint_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !waypoint_navigation__action__NavToWaypoint_Feedback__init(&mut msg as *mut _) {
        panic!("Call to waypoint_navigation__action__NavToWaypoint_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavToWaypoint_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavToWaypoint_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "waypoint_navigation/action/NavToWaypoint_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_Feedback() }
  }
}


#[link(name = "waypoint_navigation__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "waypoint_navigation__rosidl_generator_c")]
extern "C" {
    fn waypoint_navigation__action__NavToWaypoint_FeedbackMessage__init(msg: *mut NavToWaypoint_FeedbackMessage) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_FeedbackMessage>, size: usize) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_FeedbackMessage>);
    fn waypoint_navigation__action__NavToWaypoint_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavToWaypoint_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_FeedbackMessage>) -> bool;
}

// Corresponds to waypoint_navigation__action__NavToWaypoint_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::NavToWaypoint_Feedback,

}



impl Default for NavToWaypoint_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !waypoint_navigation__action__NavToWaypoint_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to waypoint_navigation__action__NavToWaypoint_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavToWaypoint_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavToWaypoint_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "waypoint_navigation/action/NavToWaypoint_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_FeedbackMessage() }
  }
}




#[link(name = "waypoint_navigation__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "waypoint_navigation__rosidl_generator_c")]
extern "C" {
    fn waypoint_navigation__action__NavToWaypoint_SendGoal_Request__init(msg: *mut NavToWaypoint_SendGoal_Request) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_SendGoal_Request>, size: usize) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_SendGoal_Request>);
    fn waypoint_navigation__action__NavToWaypoint_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavToWaypoint_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_SendGoal_Request>) -> bool;
}

// Corresponds to waypoint_navigation__action__NavToWaypoint_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::NavToWaypoint_Goal,

}



impl Default for NavToWaypoint_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !waypoint_navigation__action__NavToWaypoint_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to waypoint_navigation__action__NavToWaypoint_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavToWaypoint_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavToWaypoint_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "waypoint_navigation/action/NavToWaypoint_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_SendGoal_Request() }
  }
}


#[link(name = "waypoint_navigation__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "waypoint_navigation__rosidl_generator_c")]
extern "C" {
    fn waypoint_navigation__action__NavToWaypoint_SendGoal_Response__init(msg: *mut NavToWaypoint_SendGoal_Response) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_SendGoal_Response>, size: usize) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_SendGoal_Response>);
    fn waypoint_navigation__action__NavToWaypoint_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavToWaypoint_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_SendGoal_Response>) -> bool;
}

// Corresponds to waypoint_navigation__action__NavToWaypoint_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for NavToWaypoint_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !waypoint_navigation__action__NavToWaypoint_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to waypoint_navigation__action__NavToWaypoint_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavToWaypoint_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavToWaypoint_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "waypoint_navigation/action/NavToWaypoint_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_SendGoal_Response() }
  }
}


#[link(name = "waypoint_navigation__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "waypoint_navigation__rosidl_generator_c")]
extern "C" {
    fn waypoint_navigation__action__NavToWaypoint_GetResult_Request__init(msg: *mut NavToWaypoint_GetResult_Request) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_GetResult_Request>, size: usize) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_GetResult_Request>);
    fn waypoint_navigation__action__NavToWaypoint_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavToWaypoint_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_GetResult_Request>) -> bool;
}

// Corresponds to waypoint_navigation__action__NavToWaypoint_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for NavToWaypoint_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !waypoint_navigation__action__NavToWaypoint_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to waypoint_navigation__action__NavToWaypoint_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavToWaypoint_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavToWaypoint_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "waypoint_navigation/action/NavToWaypoint_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_GetResult_Request() }
  }
}


#[link(name = "waypoint_navigation__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "waypoint_navigation__rosidl_generator_c")]
extern "C" {
    fn waypoint_navigation__action__NavToWaypoint_GetResult_Response__init(msg: *mut NavToWaypoint_GetResult_Response) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_GetResult_Response>, size: usize) -> bool;
    fn waypoint_navigation__action__NavToWaypoint_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_GetResult_Response>);
    fn waypoint_navigation__action__NavToWaypoint_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavToWaypoint_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<NavToWaypoint_GetResult_Response>) -> bool;
}

// Corresponds to waypoint_navigation__action__NavToWaypoint_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::NavToWaypoint_Result,

}



impl Default for NavToWaypoint_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !waypoint_navigation__action__NavToWaypoint_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to waypoint_navigation__action__NavToWaypoint_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavToWaypoint_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { waypoint_navigation__action__NavToWaypoint_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavToWaypoint_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "waypoint_navigation/action/NavToWaypoint_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__waypoint_navigation__action__NavToWaypoint_GetResult_Response() }
  }
}






#[link(name = "waypoint_navigation__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__waypoint_navigation__action__NavToWaypoint_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to waypoint_navigation__action__NavToWaypoint_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct NavToWaypoint_SendGoal;

impl rosidl_runtime_rs::Service for NavToWaypoint_SendGoal {
    type Request = NavToWaypoint_SendGoal_Request;
    type Response = NavToWaypoint_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__waypoint_navigation__action__NavToWaypoint_SendGoal() }
    }
}




#[link(name = "waypoint_navigation__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__waypoint_navigation__action__NavToWaypoint_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to waypoint_navigation__action__NavToWaypoint_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct NavToWaypoint_GetResult;

impl rosidl_runtime_rs::Service for NavToWaypoint_GetResult {
    type Request = NavToWaypoint_GetResult_Request;
    type Response = NavToWaypoint_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__waypoint_navigation__action__NavToWaypoint_GetResult() }
    }
}


