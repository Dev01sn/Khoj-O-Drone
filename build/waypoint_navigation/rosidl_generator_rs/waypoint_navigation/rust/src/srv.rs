#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to waypoint_navigation__srv__GetWaypoints_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWaypoints_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub get_waypoints: bool,

}



impl Default for GetWaypoints_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetWaypoints_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetWaypoints_Request {
  type RmwMsg = super::srv::rmw::GetWaypoints_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        get_waypoints: msg.get_waypoints,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      get_waypoints: msg.get_waypoints,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      get_waypoints: msg.get_waypoints,
    }
  }
}


// Corresponds to waypoint_navigation__srv__GetWaypoints_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetWaypoints_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub waypoints: geometry_msgs::msg::PoseArray,


    // This member is not documented.
    #[allow(missing_docs)]
    pub marker_ids: Vec<std::string::String>,

}



impl Default for GetWaypoints_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetWaypoints_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetWaypoints_Response {
  type RmwMsg = super::srv::rmw::GetWaypoints_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        waypoints: geometry_msgs::msg::PoseArray::into_rmw_message(std::borrow::Cow::Owned(msg.waypoints)).into_owned(),
        marker_ids: msg.marker_ids
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        waypoints: geometry_msgs::msg::PoseArray::into_rmw_message(std::borrow::Cow::Borrowed(&msg.waypoints)).into_owned(),
        marker_ids: msg.marker_ids
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      waypoints: geometry_msgs::msg::PoseArray::from_rmw_message(msg.waypoints),
      marker_ids: msg.marker_ids
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}






#[link(name = "waypoint_navigation__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__waypoint_navigation__srv__GetWaypoints() -> *const std::ffi::c_void;
}

// Corresponds to waypoint_navigation__srv__GetWaypoints
#[allow(missing_docs, non_camel_case_types)]
pub struct GetWaypoints;

impl rosidl_runtime_rs::Service for GetWaypoints {
    type Request = GetWaypoints_Request;
    type Response = GetWaypoints_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__waypoint_navigation__srv__GetWaypoints() }
    }
}


