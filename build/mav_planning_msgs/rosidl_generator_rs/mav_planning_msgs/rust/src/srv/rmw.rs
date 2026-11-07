#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__srv__PlannerService_Request() -> *const std::ffi::c_void;
}

#[link(name = "mav_planning_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_planning_msgs__srv__PlannerService_Request__init(msg: *mut PlannerService_Request) -> bool;
    fn mav_planning_msgs__srv__PlannerService_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlannerService_Request>, size: usize) -> bool;
    fn mav_planning_msgs__srv__PlannerService_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlannerService_Request>);
    fn mav_planning_msgs__srv__PlannerService_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlannerService_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PlannerService_Request>) -> bool;
}

// Corresponds to mav_planning_msgs__srv__PlannerService_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlannerService_Request {
    /// start pose for the planner
    pub start_pose: geometry_msgs::msg::rmw::PoseStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub start_velocity: geometry_msgs::msg::rmw::Vector3,

    /// start pose for the planner
    pub goal_pose: geometry_msgs::msg::rmw::PoseStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_velocity: geometry_msgs::msg::rmw::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bounding_box: geometry_msgs::msg::rmw::Vector3,

}



impl Default for PlannerService_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_planning_msgs__srv__PlannerService_Request__init(&mut msg as *mut _) {
        panic!("Call to mav_planning_msgs__srv__PlannerService_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlannerService_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__PlannerService_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__PlannerService_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__PlannerService_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlannerService_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlannerService_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mav_planning_msgs/srv/PlannerService_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__srv__PlannerService_Request() }
  }
}


#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__srv__PlannerService_Response() -> *const std::ffi::c_void;
}

#[link(name = "mav_planning_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_planning_msgs__srv__PlannerService_Response__init(msg: *mut PlannerService_Response) -> bool;
    fn mav_planning_msgs__srv__PlannerService_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlannerService_Response>, size: usize) -> bool;
    fn mav_planning_msgs__srv__PlannerService_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlannerService_Response>);
    fn mav_planning_msgs__srv__PlannerService_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlannerService_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PlannerService_Response>) -> bool;
}

// Corresponds to mav_planning_msgs__srv__PlannerService_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlannerService_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

    /// Either contains a polynomial trajectory:
    pub polynomial_plan: super::super::msg::rmw::PolynomialTrajectory,


    // This member is not documented.
    #[allow(missing_docs)]
    pub polynomial_plan_4d: super::super::msg::rmw::PolynomialTrajectory4D,

    /// or a MultiDOFJointTrajectory containing a sampled path (or straight-line
    /// waypoints, depending on the planner).
    /// Only one of these should be non-empty.
    pub sampled_plan: trajectory_msgs::msg::rmw::MultiDOFJointTrajectory,

}



impl Default for PlannerService_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_planning_msgs__srv__PlannerService_Response__init(&mut msg as *mut _) {
        panic!("Call to mav_planning_msgs__srv__PlannerService_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlannerService_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__PlannerService_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__PlannerService_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__PlannerService_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlannerService_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlannerService_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mav_planning_msgs/srv/PlannerService_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__srv__PlannerService_Response() }
  }
}


#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__srv__PolygonService_Request() -> *const std::ffi::c_void;
}

#[link(name = "mav_planning_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_planning_msgs__srv__PolygonService_Request__init(msg: *mut PolygonService_Request) -> bool;
    fn mav_planning_msgs__srv__PolygonService_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PolygonService_Request>, size: usize) -> bool;
    fn mav_planning_msgs__srv__PolygonService_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PolygonService_Request>);
    fn mav_planning_msgs__srv__PolygonService_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PolygonService_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PolygonService_Request>) -> bool;
}

// Corresponds to mav_planning_msgs__srv__PolygonService_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolygonService_Request {
    /// The new polygon.
    pub polygon: super::super::msg::rmw::PolygonWithHolesStamped,

}



impl Default for PolygonService_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_planning_msgs__srv__PolygonService_Request__init(&mut msg as *mut _) {
        panic!("Call to mav_planning_msgs__srv__PolygonService_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PolygonService_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__PolygonService_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__PolygonService_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__PolygonService_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PolygonService_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PolygonService_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mav_planning_msgs/srv/PolygonService_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__srv__PolygonService_Request() }
  }
}


#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__srv__PolygonService_Response() -> *const std::ffi::c_void;
}

#[link(name = "mav_planning_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_planning_msgs__srv__PolygonService_Response__init(msg: *mut PolygonService_Response) -> bool;
    fn mav_planning_msgs__srv__PolygonService_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PolygonService_Response>, size: usize) -> bool;
    fn mav_planning_msgs__srv__PolygonService_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PolygonService_Response>);
    fn mav_planning_msgs__srv__PolygonService_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PolygonService_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PolygonService_Response>) -> bool;
}

// Corresponds to mav_planning_msgs__srv__PolygonService_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolygonService_Response {
    /// True on success, false on polygon error.
    pub success: bool,

}



impl Default for PolygonService_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_planning_msgs__srv__PolygonService_Response__init(&mut msg as *mut _) {
        panic!("Call to mav_planning_msgs__srv__PolygonService_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PolygonService_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__PolygonService_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__PolygonService_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__PolygonService_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PolygonService_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PolygonService_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mav_planning_msgs/srv/PolygonService_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__srv__PolygonService_Response() }
  }
}


#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__srv__ChangeNameService_Request() -> *const std::ffi::c_void;
}

#[link(name = "mav_planning_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_planning_msgs__srv__ChangeNameService_Request__init(msg: *mut ChangeNameService_Request) -> bool;
    fn mav_planning_msgs__srv__ChangeNameService_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ChangeNameService_Request>, size: usize) -> bool;
    fn mav_planning_msgs__srv__ChangeNameService_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ChangeNameService_Request>);
    fn mav_planning_msgs__srv__ChangeNameService_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ChangeNameService_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ChangeNameService_Request>) -> bool;
}

// Corresponds to mav_planning_msgs__srv__ChangeNameService_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChangeNameService_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,

}



impl Default for ChangeNameService_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_planning_msgs__srv__ChangeNameService_Request__init(&mut msg as *mut _) {
        panic!("Call to mav_planning_msgs__srv__ChangeNameService_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ChangeNameService_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__ChangeNameService_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__ChangeNameService_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__ChangeNameService_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ChangeNameService_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ChangeNameService_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mav_planning_msgs/srv/ChangeNameService_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__srv__ChangeNameService_Request() }
  }
}


#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__srv__ChangeNameService_Response() -> *const std::ffi::c_void;
}

#[link(name = "mav_planning_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_planning_msgs__srv__ChangeNameService_Response__init(msg: *mut ChangeNameService_Response) -> bool;
    fn mav_planning_msgs__srv__ChangeNameService_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ChangeNameService_Response>, size: usize) -> bool;
    fn mav_planning_msgs__srv__ChangeNameService_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ChangeNameService_Response>);
    fn mav_planning_msgs__srv__ChangeNameService_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ChangeNameService_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ChangeNameService_Response>) -> bool;
}

// Corresponds to mav_planning_msgs__srv__ChangeNameService_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChangeNameService_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for ChangeNameService_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_planning_msgs__srv__ChangeNameService_Response__init(&mut msg as *mut _) {
        panic!("Call to mav_planning_msgs__srv__ChangeNameService_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ChangeNameService_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__ChangeNameService_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__ChangeNameService_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_planning_msgs__srv__ChangeNameService_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ChangeNameService_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ChangeNameService_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mav_planning_msgs/srv/ChangeNameService_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_planning_msgs__srv__ChangeNameService_Response() }
  }
}






#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mav_planning_msgs__srv__PlannerService() -> *const std::ffi::c_void;
}

// Corresponds to mav_planning_msgs__srv__PlannerService
#[allow(missing_docs, non_camel_case_types)]
pub struct PlannerService;

impl rosidl_runtime_rs::Service for PlannerService {
    type Request = PlannerService_Request;
    type Response = PlannerService_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mav_planning_msgs__srv__PlannerService() }
    }
}




#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mav_planning_msgs__srv__PolygonService() -> *const std::ffi::c_void;
}

// Corresponds to mav_planning_msgs__srv__PolygonService
#[allow(missing_docs, non_camel_case_types)]
pub struct PolygonService;

impl rosidl_runtime_rs::Service for PolygonService {
    type Request = PolygonService_Request;
    type Response = PolygonService_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mav_planning_msgs__srv__PolygonService() }
    }
}




#[link(name = "mav_planning_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mav_planning_msgs__srv__ChangeNameService() -> *const std::ffi::c_void;
}

// Corresponds to mav_planning_msgs__srv__ChangeNameService
#[allow(missing_docs, non_camel_case_types)]
pub struct ChangeNameService;

impl rosidl_runtime_rs::Service for ChangeNameService {
    type Request = ChangeNameService_Request;
    type Response = ChangeNameService_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mav_planning_msgs__srv__ChangeNameService() }
    }
}


