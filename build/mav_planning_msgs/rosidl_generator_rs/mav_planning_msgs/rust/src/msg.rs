#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to mav_planning_msgs__msg__Point2D
/// This contains the position of a 2D point.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Point2D {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f64,

}



impl Default for Point2D {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Point2D::default())
  }
}

impl rosidl_runtime_rs::Message for Point2D {
  type RmwMsg = super::msg::rmw::Point2D;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
    }
  }
}


// Corresponds to mav_planning_msgs__msg__PointCloudWithPose

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PointCloudWithPose {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sensor_pose: geometry_msgs::msg::TransformStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cloud_in_sensor_frame: sensor_msgs::msg::PointCloud2,

}



impl Default for PointCloudWithPose {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PointCloudWithPose::default())
  }
}

impl rosidl_runtime_rs::Message for PointCloudWithPose {
  type RmwMsg = super::msg::rmw::PointCloudWithPose;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        sensor_pose: geometry_msgs::msg::TransformStamped::into_rmw_message(std::borrow::Cow::Owned(msg.sensor_pose)).into_owned(),
        cloud_in_sensor_frame: sensor_msgs::msg::PointCloud2::into_rmw_message(std::borrow::Cow::Owned(msg.cloud_in_sensor_frame)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        sensor_pose: geometry_msgs::msg::TransformStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.sensor_pose)).into_owned(),
        cloud_in_sensor_frame: sensor_msgs::msg::PointCloud2::into_rmw_message(std::borrow::Cow::Borrowed(&msg.cloud_in_sensor_frame)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      sensor_pose: geometry_msgs::msg::TransformStamped::from_rmw_message(msg.sensor_pose),
      cloud_in_sensor_frame: sensor_msgs::msg::PointCloud2::from_rmw_message(msg.cloud_in_sensor_frame),
    }
  }
}


// Corresponds to mav_planning_msgs__msg__Polygon2D
/// A specification of a 2D polygon where the first and last points are assumed to be connected.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Polygon2D {

    // This member is not documented.
    #[allow(missing_docs)]
    pub points: Vec<super::msg::Point2D>,

}



impl Default for Polygon2D {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Polygon2D::default())
  }
}

impl rosidl_runtime_rs::Message for Polygon2D {
  type RmwMsg = super::msg::rmw::Polygon2D;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        points: msg.points
          .into_iter()
          .map(|elem| super::msg::Point2D::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        points: msg.points
          .iter()
          .map(|elem| super::msg::Point2D::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      points: msg.points
          .into_iter()
          .map(super::msg::Point2D::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to mav_planning_msgs__msg__PolygonWithHoles
/// A message to define a 2D polygon with holes.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolygonWithHoles {

    // This member is not documented.
    #[allow(missing_docs)]
    pub hull: super::msg::Polygon2D,


    // This member is not documented.
    #[allow(missing_docs)]
    pub holes: Vec<super::msg::Polygon2D>,

}



impl Default for PolygonWithHoles {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PolygonWithHoles::default())
  }
}

impl rosidl_runtime_rs::Message for PolygonWithHoles {
  type RmwMsg = super::msg::rmw::PolygonWithHoles;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        hull: super::msg::Polygon2D::into_rmw_message(std::borrow::Cow::Owned(msg.hull)).into_owned(),
        holes: msg.holes
          .into_iter()
          .map(|elem| super::msg::Polygon2D::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        hull: super::msg::Polygon2D::into_rmw_message(std::borrow::Cow::Borrowed(&msg.hull)).into_owned(),
        holes: msg.holes
          .iter()
          .map(|elem| super::msg::Polygon2D::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      hull: super::msg::Polygon2D::from_rmw_message(msg.hull),
      holes: msg.holes
          .into_iter()
          .map(super::msg::Polygon2D::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to mav_planning_msgs__msg__PolygonWithHolesStamped
/// A message to define a 2D polygon with holes, stamp, and altitude above ground.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolygonWithHolesStamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub altitude: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub polygon: super::msg::PolygonWithHoles,

}



impl Default for PolygonWithHolesStamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PolygonWithHolesStamped::default())
  }
}

impl rosidl_runtime_rs::Message for PolygonWithHolesStamped {
  type RmwMsg = super::msg::rmw::PolygonWithHolesStamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        altitude: msg.altitude,
        polygon: super::msg::PolygonWithHoles::into_rmw_message(std::borrow::Cow::Owned(msg.polygon)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      altitude: msg.altitude,
        polygon: super::msg::PolygonWithHoles::into_rmw_message(std::borrow::Cow::Borrowed(&msg.polygon)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      altitude: msg.altitude,
      polygon: super::msg::PolygonWithHoles::from_rmw_message(msg.polygon),
    }
  }
}


// Corresponds to mav_planning_msgs__msg__PolynomialSegment

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolynomialSegment {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// order of the polynomial + 1, should match size of x[]
    pub num_coeffs: i32,

    /// duration of the segment
    pub segment_time: builtin_interfaces::msg::Duration,

    /// coefficients for the x-axis, INCREASING order
    pub x: Vec<f64>,

    /// coefficients for the y-axis, INCREASING order
    pub y: Vec<f64>,

    /// coefficients for the z-axis, INCREASING order
    pub z: Vec<f64>,

    /// coefficients for the rotation x-vector, INCREASING order
    pub rx: Vec<f64>,

    /// coefficients for the rotation y-vector, INCREASING order
    pub ry: Vec<f64>,

    /// coefficients for the rotation z-vector, INCREASING order
    pub rz: Vec<f64>,

    /// For backwards compatibility with underactuated (4DOF) commands):
    /// coefficients for the yaw, INCREASING order
    pub yaw: Vec<f64>,

}



impl Default for PolynomialSegment {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PolynomialSegment::default())
  }
}

impl rosidl_runtime_rs::Message for PolynomialSegment {
  type RmwMsg = super::msg::rmw::PolynomialSegment;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        num_coeffs: msg.num_coeffs,
        segment_time: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.segment_time)).into_owned(),
        x: msg.x.as_slice().into(),
        y: msg.y.as_slice().into(),
        z: msg.z.as_slice().into(),
        rx: msg.rx.as_slice().into(),
        ry: msg.ry.as_slice().into(),
        rz: msg.rz.as_slice().into(),
        yaw: msg.yaw.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      num_coeffs: msg.num_coeffs,
        segment_time: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.segment_time)).into_owned(),
        x: msg.x.as_slice().into(),
        y: msg.y.as_slice().into(),
        z: msg.z.as_slice().into(),
        rx: msg.rx.as_slice().into(),
        ry: msg.ry.as_slice().into(),
        rz: msg.rz.as_slice().into(),
        yaw: msg.yaw.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      num_coeffs: msg.num_coeffs,
      segment_time: builtin_interfaces::msg::Duration::from_rmw_message(msg.segment_time),
      x: msg.x.into(),
      y: msg.y.into(),
      z: msg.z.into(),
      rx: msg.rx.into(),
      ry: msg.ry.into(),
      rz: msg.rz.into(),
      yaw: msg.yaw.into(),
    }
  }
}


// Corresponds to mav_planning_msgs__msg__PolynomialSegment4D

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolynomialSegment4D {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// order of the polynomial + 1, should match size of x[]
    pub num_coeffs: i32,

    /// duration of the segment
    pub segment_time: builtin_interfaces::msg::Duration,

    /// coefficients for the x-axis, INCREASING order
    pub x: Vec<f64>,

    /// coefficients for the y-axis, INCREASING order
    pub y: Vec<f64>,

    /// coefficients for the z-axis, INCREASING order
    pub z: Vec<f64>,

    /// coefficients for the yaw, INCREASING order
    pub yaw: Vec<f64>,

}



impl Default for PolynomialSegment4D {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PolynomialSegment4D::default())
  }
}

impl rosidl_runtime_rs::Message for PolynomialSegment4D {
  type RmwMsg = super::msg::rmw::PolynomialSegment4D;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        num_coeffs: msg.num_coeffs,
        segment_time: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.segment_time)).into_owned(),
        x: msg.x.as_slice().into(),
        y: msg.y.as_slice().into(),
        z: msg.z.as_slice().into(),
        yaw: msg.yaw.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      num_coeffs: msg.num_coeffs,
        segment_time: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.segment_time)).into_owned(),
        x: msg.x.as_slice().into(),
        y: msg.y.as_slice().into(),
        z: msg.z.as_slice().into(),
        yaw: msg.yaw.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      num_coeffs: msg.num_coeffs,
      segment_time: builtin_interfaces::msg::Duration::from_rmw_message(msg.segment_time),
      x: msg.x.into(),
      y: msg.y.into(),
      z: msg.z.into(),
      yaw: msg.yaw.into(),
    }
  }
}


// Corresponds to mav_planning_msgs__msg__PolynomialTrajectory

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolynomialTrajectory {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub segments: Vec<super::msg::PolynomialSegment>,

}



impl Default for PolynomialTrajectory {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PolynomialTrajectory::default())
  }
}

impl rosidl_runtime_rs::Message for PolynomialTrajectory {
  type RmwMsg = super::msg::rmw::PolynomialTrajectory;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        segments: msg.segments
          .into_iter()
          .map(|elem| super::msg::PolynomialSegment::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        segments: msg.segments
          .iter()
          .map(|elem| super::msg::PolynomialSegment::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      segments: msg.segments
          .into_iter()
          .map(super::msg::PolynomialSegment::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to mav_planning_msgs__msg__PolynomialTrajectory4D

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PolynomialTrajectory4D {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub segments: Vec<super::msg::PolynomialSegment4D>,

}



impl Default for PolynomialTrajectory4D {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PolynomialTrajectory4D::default())
  }
}

impl rosidl_runtime_rs::Message for PolynomialTrajectory4D {
  type RmwMsg = super::msg::rmw::PolynomialTrajectory4D;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        segments: msg.segments
          .into_iter()
          .map(|elem| super::msg::PolynomialSegment4D::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        segments: msg.segments
          .iter()
          .map(|elem| super::msg::PolynomialSegment4D::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      segments: msg.segments
          .into_iter()
          .map(super::msg::PolynomialSegment4D::from_rmw_message)
          .collect(),
    }
  }
}


