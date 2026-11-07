#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to whycode_interfaces__srv__SelectMarker_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SelectMarker_Request {
    /// x,y of seleceted marker in image coordinates
    pub point: geometry_msgs::msg::Point,

}



impl Default for SelectMarker_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SelectMarker_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SelectMarker_Request {
  type RmwMsg = super::srv::rmw::SelectMarker_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        point: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.point)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        point: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.point)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      point: geometry_msgs::msg::Point::from_rmw_message(msg.point),
    }
  }
}


// Corresponds to whycode_interfaces__srv__SelectMarker_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SelectMarker_Response {
    /// bool feedback
    pub success: bool,

}



impl Default for SelectMarker_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SelectMarker_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SelectMarker_Response {
  type RmwMsg = super::srv::rmw::SelectMarker_Response;

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


// Corresponds to whycode_interfaces__srv__SetCalibMethod_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCalibMethod_Request {
    /// 0 = autocalibration
    /// 1 = manual calibration
    pub method: i8,

}



impl Default for SetCalibMethod_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCalibMethod_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetCalibMethod_Request {
  type RmwMsg = super::srv::rmw::SetCalibMethod_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        method: msg.method,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      method: msg.method,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      method: msg.method,
    }
  }
}


// Corresponds to whycode_interfaces__srv__SetCalibMethod_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCalibMethod_Response {
    /// bool feedback
    pub success: bool,

    /// information message
    pub msg: std::string::String,

}



impl Default for SetCalibMethod_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCalibMethod_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetCalibMethod_Response {
  type RmwMsg = super::srv::rmw::SetCalibMethod_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        msg: msg.msg.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        msg: msg.msg.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      msg: msg.msg.to_string(),
    }
  }
}


// Corresponds to whycode_interfaces__srv__SetCalibPath_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCalibPath_Request {
    /// either "save" or "load"
    pub action: std::string::String,

    /// path to yaml file
    pub path: std::string::String,

}



impl Default for SetCalibPath_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCalibPath_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetCalibPath_Request {
  type RmwMsg = super::srv::rmw::SetCalibPath_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        action: msg.action.as_str().into(),
        path: msg.path.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        action: msg.action.as_str().into(),
        path: msg.path.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      action: msg.action.to_string(),
      path: msg.path.to_string(),
    }
  }
}


// Corresponds to whycode_interfaces__srv__SetCalibPath_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCalibPath_Response {
    /// bool feedback
    pub success: bool,

    /// information message
    pub msg: std::string::String,

}



impl Default for SetCalibPath_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCalibPath_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetCalibPath_Response {
  type RmwMsg = super::srv::rmw::SetCalibPath_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        msg: msg.msg.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        msg: msg.msg.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      msg: msg.msg.to_string(),
    }
  }
}


// Corresponds to whycode_interfaces__srv__SetCoords_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCoords_Request {
    /// 0 = camera coords
    /// 1 = 2D coords
    /// 2 = 3D coords
    pub coords: i8,

}



impl Default for SetCoords_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCoords_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetCoords_Request {
  type RmwMsg = super::srv::rmw::SetCoords_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        coords: msg.coords,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      coords: msg.coords,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      coords: msg.coords,
    }
  }
}


// Corresponds to whycode_interfaces__srv__SetCoords_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCoords_Response {
    /// bool feedback
    pub success: bool,

    /// information message
    pub msg: std::string::String,

}



impl Default for SetCoords_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCoords_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetCoords_Response {
  type RmwMsg = super::srv::rmw::SetCoords_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        msg: msg.msg.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        msg: msg.msg.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      msg: msg.msg.to_string(),
    }
  }
}


// Corresponds to whycode_interfaces__srv__SetDrawing_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetDrawing_Request {
    /// draw/hide coordinates
    pub draw_coords: bool,

    /// draw/hide segmentation outcome
    pub draw_segments: bool,

}



impl Default for SetDrawing_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetDrawing_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetDrawing_Request {
  type RmwMsg = super::srv::rmw::SetDrawing_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        draw_coords: msg.draw_coords,
        draw_segments: msg.draw_segments,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      draw_coords: msg.draw_coords,
      draw_segments: msg.draw_segments,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      draw_coords: msg.draw_coords,
      draw_segments: msg.draw_segments,
    }
  }
}


// Corresponds to whycode_interfaces__srv__SetDrawing_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetDrawing_Response {
    /// bool feedback
    pub success: bool,

}



impl Default for SetDrawing_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetDrawing_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetDrawing_Response {
  type RmwMsg = super::srv::rmw::SetDrawing_Response;

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


// Corresponds to whycode_interfaces__srv__GetGuiSettings_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetGuiSettings_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetGuiSettings_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetGuiSettings_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetGuiSettings_Request {
  type RmwMsg = super::srv::rmw::GetGuiSettings_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to whycode_interfaces__srv__GetGuiSettings_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetGuiSettings_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetGuiSettings_Response {
  type RmwMsg = super::srv::rmw::GetGuiSettings_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        draw_coords: msg.draw_coords,
        draw_segments: msg.draw_segments,
        coords: msg.coords,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      draw_coords: msg.draw_coords,
      draw_segments: msg.draw_segments,
      coords: msg.coords,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      draw_coords: msg.draw_coords,
      draw_segments: msg.draw_segments,
      coords: msg.coords,
    }
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


