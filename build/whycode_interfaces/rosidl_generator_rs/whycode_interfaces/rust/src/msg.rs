#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to whycode_interfaces__msg__Marker

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Marker {
    /// ID
    pub id: i8,

    /// in pixels
    pub size: i32,

    /// camera coordinates
    pub u: f32,

    /// camera coordinates
    pub v: f32,

    /// rotation around surface normal
    pub angle: f32,

    /// pose
    pub position: geometry_msgs::msg::Pose,

    /// euler angles roll, pitch, yaw
    pub rotation: geometry_msgs::msg::Vector3,

}



impl Default for Marker {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Marker::default())
  }
}

impl rosidl_runtime_rs::Message for Marker {
  type RmwMsg = super::msg::rmw::Marker;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        size: msg.size,
        u: msg.u,
        v: msg.v,
        angle: msg.angle,
        position: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.position)).into_owned(),
        rotation: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.rotation)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      size: msg.size,
      u: msg.u,
      v: msg.v,
      angle: msg.angle,
        position: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.position)).into_owned(),
        rotation: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.rotation)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      size: msg.size,
      u: msg.u,
      v: msg.v,
      angle: msg.angle,
      position: geometry_msgs::msg::Pose::from_rmw_message(msg.position),
      rotation: geometry_msgs::msg::Vector3::from_rmw_message(msg.rotation),
    }
  }
}


// Corresponds to whycode_interfaces__msg__MarkerArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MarkerArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub markers: Vec<super::msg::Marker>,

}



impl Default for MarkerArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MarkerArray::default())
  }
}

impl rosidl_runtime_rs::Message for MarkerArray {
  type RmwMsg = super::msg::rmw::MarkerArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        markers: msg.markers
          .into_iter()
          .map(|elem| super::msg::Marker::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        markers: msg.markers
          .iter()
          .map(|elem| super::msg::Marker::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      markers: msg.markers
          .into_iter()
          .map(super::msg::Marker::from_rmw_message)
          .collect(),
    }
  }
}


