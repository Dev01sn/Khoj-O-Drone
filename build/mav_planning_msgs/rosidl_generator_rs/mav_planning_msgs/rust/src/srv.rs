#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to mav_planning_msgs__srv__PlannerService_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlannerService_Request {
    /// start pose for the planner
    pub start_pose: geometry_msgs::msg::PoseStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub start_velocity: geometry_msgs::msg::Vector3,

    /// start pose for the planner
    pub goal_pose: geometry_msgs::msg::PoseStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_velocity: geometry_msgs::msg::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bounding_box: geometry_msgs::msg::Vector3,

}



impl Default for PlannerService_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PlannerService_Request::default())
  }
}

impl rosidl_runtime_rs::Message for PlannerService_Request {
  type RmwMsg = super::srv::rmw::PlannerService_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        start_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.start_pose)).into_owned(),
        start_velocity: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.start_velocity)).into_owned(),
        goal_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.goal_pose)).into_owned(),
        goal_velocity: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.goal_velocity)).into_owned(),
        bounding_box: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.bounding_box)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        start_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.start_pose)).into_owned(),
        start_velocity: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.start_velocity)).into_owned(),
        goal_pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_pose)).into_owned(),
        goal_velocity: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_velocity)).into_owned(),
        bounding_box: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.bounding_box)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      start_pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.start_pose),
      start_velocity: geometry_msgs::msg::Vector3::from_rmw_message(msg.start_velocity),
      goal_pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.goal_pose),
      goal_velocity: geometry_msgs::msg::Vector3::from_rmw_message(msg.goal_velocity),
      bounding_box: geometry_msgs::msg::Vector3::from_rmw_message(msg.bounding_box),
    }
  }
}


// Corresponds to mav_planning_msgs__srv__PlannerService_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlannerService_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

    /// Either contains a polynomial trajectory:
    pub polynomial_plan: super::msg::PolynomialTrajectory,


    // This member is not documented.
    #[allow(missing_docs)]
    pub polynomial_plan_4d: super::msg::PolynomialTrajectory4D,

    /// or a MultiDOFJointTrajectory containing a sampled path (or straight-line
    /// waypoints, depending on the planner).
    /// Only one of these should be non-empty.
    pub sampled_plan: trajectory_msgs::msg::MultiDOFJointTrajectory,

}



impl Default for PlannerService_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PlannerService_Response::default())
  }
}

impl rosidl_runtime_rs::Message for PlannerService_Response {
  type RmwMsg = super::srv::rmw::PlannerService_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        polynomial_plan: super::msg::PolynomialTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.polynomial_plan)).into_owned(),
        polynomial_plan_4d: super::msg::PolynomialTrajectory4D::into_rmw_message(std::borrow::Cow::Owned(msg.polynomial_plan_4d)).into_owned(),
        sampled_plan: trajectory_msgs::msg::MultiDOFJointTrajectory::into_rmw_message(std::borrow::Cow::Owned(msg.sampled_plan)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        polynomial_plan: super::msg::PolynomialTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.polynomial_plan)).into_owned(),
        polynomial_plan_4d: super::msg::PolynomialTrajectory4D::into_rmw_message(std::borrow::Cow::Borrowed(&msg.polynomial_plan_4d)).into_owned(),
        sampled_plan: trajectory_msgs::msg::MultiDOFJointTrajectory::into_rmw_message(std::borrow::Cow::Borrowed(&msg.sampled_plan)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      polynomial_plan: super::msg::PolynomialTrajectory::from_rmw_message(msg.polynomial_plan),
      polynomial_plan_4d: super::msg::PolynomialTrajectory4D::from_rmw_message(msg.polynomial_plan_4d),
      sampled_plan: trajectory_msgs::msg::MultiDOFJointTrajectory::from_rmw_message(msg.sampled_plan),
    }
  }
}


// Corresponds to mav_planning_msgs__srv__PolygonService_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolygonService_Request {
    /// The new polygon.
    pub polygon: super::msg::PolygonWithHolesStamped,

}



impl Default for PolygonService_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PolygonService_Request::default())
  }
}

impl rosidl_runtime_rs::Message for PolygonService_Request {
  type RmwMsg = super::srv::rmw::PolygonService_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        polygon: super::msg::PolygonWithHolesStamped::into_rmw_message(std::borrow::Cow::Owned(msg.polygon)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        polygon: super::msg::PolygonWithHolesStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.polygon)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      polygon: super::msg::PolygonWithHolesStamped::from_rmw_message(msg.polygon),
    }
  }
}


// Corresponds to mav_planning_msgs__srv__PolygonService_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolygonService_Response {
    /// True on success, false on polygon error.
    pub success: bool,

}



impl Default for PolygonService_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PolygonService_Response::default())
  }
}

impl rosidl_runtime_rs::Message for PolygonService_Response {
  type RmwMsg = super::srv::rmw::PolygonService_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to mav_planning_msgs__srv__ChangeNameService_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChangeNameService_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,

}



impl Default for ChangeNameService_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ChangeNameService_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ChangeNameService_Request {
  type RmwMsg = super::srv::rmw::ChangeNameService_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
    }
  }
}


// Corresponds to mav_planning_msgs__srv__ChangeNameService_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChangeNameService_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for ChangeNameService_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ChangeNameService_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ChangeNameService_Response {
  type RmwMsg = super::srv::rmw::ChangeNameService_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
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


