
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to waypoint_navigation__action__NavToWaypoint_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub waypoint: geometry_msgs::msg::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub hover: bool,

}



impl Default for NavToWaypoint_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavToWaypoint_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_Goal {
  type RmwMsg = super::action::rmw::NavToWaypoint_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        waypoint: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.waypoint)).into_owned(),
        hover: msg.hover,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        waypoint: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.waypoint)).into_owned(),
      hover: msg.hover,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      waypoint: geometry_msgs::msg::Pose::from_rmw_message(msg.waypoint),
      hover: msg.hover,
    }
  }
}


// Corresponds to waypoint_navigation__action__NavToWaypoint_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub hov_time: i32,

}



impl Default for NavToWaypoint_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavToWaypoint_Result::default())
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_Result {
  type RmwMsg = super::action::rmw::NavToWaypoint_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        hov_time: msg.hov_time,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      hov_time: msg.hov_time,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      hov_time: msg.hov_time,
    }
  }
}


// Corresponds to waypoint_navigation__action__NavToWaypoint_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_waypoint: geometry_msgs::msg::PoseStamped,

}



impl Default for NavToWaypoint_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavToWaypoint_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_Feedback {
  type RmwMsg = super::action::rmw::NavToWaypoint_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_waypoint: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.current_waypoint)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_waypoint: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.current_waypoint)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      current_waypoint: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.current_waypoint),
    }
  }
}


// Corresponds to waypoint_navigation__action__NavToWaypoint_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::NavToWaypoint_Feedback,

}



impl Default for NavToWaypoint_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavToWaypoint_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_FeedbackMessage {
  type RmwMsg = super::action::rmw::NavToWaypoint_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::NavToWaypoint_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::NavToWaypoint_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::NavToWaypoint_Feedback::from_rmw_message(msg.feedback),
    }
  }
}






// Corresponds to waypoint_navigation__action__NavToWaypoint_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::NavToWaypoint_Goal,

}



impl Default for NavToWaypoint_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavToWaypoint_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_SendGoal_Request {
  type RmwMsg = super::action::rmw::NavToWaypoint_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::NavToWaypoint_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::NavToWaypoint_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::NavToWaypoint_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to waypoint_navigation__action__NavToWaypoint_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for NavToWaypoint_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavToWaypoint_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_SendGoal_Response {
  type RmwMsg = super::action::rmw::NavToWaypoint_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to waypoint_navigation__action__NavToWaypoint_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for NavToWaypoint_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavToWaypoint_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_GetResult_Request {
  type RmwMsg = super::action::rmw::NavToWaypoint_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to waypoint_navigation__action__NavToWaypoint_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavToWaypoint_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::NavToWaypoint_Result,

}



impl Default for NavToWaypoint_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::NavToWaypoint_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for NavToWaypoint_GetResult_Response {
  type RmwMsg = super::action::rmw::NavToWaypoint_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::NavToWaypoint_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::NavToWaypoint_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::NavToWaypoint_Result::from_rmw_message(msg.result),
    }
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






#[link(name = "waypoint_navigation__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__waypoint_navigation__action__NavToWaypoint() -> *const std::ffi::c_void;
}

// Corresponds to waypoint_navigation__action__NavToWaypoint
#[allow(missing_docs, non_camel_case_types)]
pub struct NavToWaypoint;

impl rosidl_runtime_rs::Action for NavToWaypoint {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = NavToWaypoint_Goal;

  /// The result message defined in the action definition.
  type Result = NavToWaypoint_Result;

  /// The feedback message defined in the action definition.
  type Feedback = NavToWaypoint_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::NavToWaypoint_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::NavToWaypoint_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::NavToWaypoint_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__waypoint_navigation__action__NavToWaypoint() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::NavToWaypoint_Goal,
  ) -> super::action::rmw::NavToWaypoint_SendGoal_Request {
   super::action::rmw::NavToWaypoint_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::NavToWaypoint_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::NavToWaypoint_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::NavToWaypoint_SendGoal_Response {
   super::action::rmw::NavToWaypoint_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::NavToWaypoint_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::NavToWaypoint_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::NavToWaypoint_Feedback,
  ) -> super::action::rmw::NavToWaypoint_FeedbackMessage {
    let mut message = super::action::rmw::NavToWaypoint_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::NavToWaypoint_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::NavToWaypoint_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::NavToWaypoint_GetResult_Request {
   super::action::rmw::NavToWaypoint_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::NavToWaypoint_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::NavToWaypoint_Result,
  ) -> super::action::rmw::NavToWaypoint_GetResult_Response {
   super::action::rmw::NavToWaypoint_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::NavToWaypoint_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::NavToWaypoint_Result,
  ) {
    (response.status, response.result)
  }
}


