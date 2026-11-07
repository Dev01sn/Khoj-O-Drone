#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to mav_msgs__msg__Actuators

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Actuators {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// This message defines lowest level commands to be sent to the actuator(s).
    /// Angle of the actuator in.
    /// E.g. servo angle of a control surface(not angle of the surface!), orientation-angle of a thruster.
    pub angles: Vec<f64>,

    /// Angular velocities of the actuator in.
    /// E.g. "rpm" of rotors, propellers, thrusters
    pub angular_velocities: Vec<f64>,

    /// Everything that does not fit the above, normalized between.
    pub normalized: Vec<f64>,

}



impl Default for Actuators {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Actuators::default())
  }
}

impl rosidl_runtime_rs::Message for Actuators {
  type RmwMsg = super::msg::rmw::Actuators;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        angles: msg.angles.as_slice().into(),
        angular_velocities: msg.angular_velocities.as_slice().into(),
        normalized: msg.normalized.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        angles: msg.angles.as_slice().into(),
        angular_velocities: msg.angular_velocities.as_slice().into(),
        normalized: msg.normalized.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      angles: msg.angles.into(),
      angular_velocities: msg.angular_velocities.into(),
      normalized: msg.normalized.into(),
    }
  }
}


// Corresponds to mav_msgs__msg__AttitudeThrust

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AttitudeThrust {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// Attitude expressed in the header/frame_id frame.
    pub attitude: geometry_msgs::msg::Quaternion,

    /// Thrust expressed in the body frame.
    /// For a fixed-wing, usually the x-component is used.
    /// For a multi-rotor, usually the z-component is used.
    /// Set all un-used components to 0.
    pub thrust: geometry_msgs::msg::Vector3,

}



impl Default for AttitudeThrust {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AttitudeThrust::default())
  }
}

impl rosidl_runtime_rs::Message for AttitudeThrust {
  type RmwMsg = super::msg::rmw::AttitudeThrust;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        attitude: geometry_msgs::msg::Quaternion::into_rmw_message(std::borrow::Cow::Owned(msg.attitude)).into_owned(),
        thrust: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.thrust)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        attitude: geometry_msgs::msg::Quaternion::into_rmw_message(std::borrow::Cow::Borrowed(&msg.attitude)).into_owned(),
        thrust: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.thrust)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      attitude: geometry_msgs::msg::Quaternion::from_rmw_message(msg.attitude),
      thrust: geometry_msgs::msg::Vector3::from_rmw_message(msg.thrust),
    }
  }
}


// Corresponds to mav_msgs__msg__FilteredSensorData

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FilteredSensorData {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// acceleration in vehicle frame
    pub accelerometer: geometry_msgs::msg::Vector3,

    /// rotational velocity in vehicle frame
    pub gyroscope: geometry_msgs::msg::Vector3,

    /// Magnetometer measurements in vehicle frame
    pub magnetometer: geometry_msgs::msg::Vector3,

    /// Height from barometer relative to start-up point
    pub barometer: f64,

}



impl Default for FilteredSensorData {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::FilteredSensorData::default())
  }
}

impl rosidl_runtime_rs::Message for FilteredSensorData {
  type RmwMsg = super::msg::rmw::FilteredSensorData;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        accelerometer: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.accelerometer)).into_owned(),
        gyroscope: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.gyroscope)).into_owned(),
        magnetometer: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.magnetometer)).into_owned(),
        barometer: msg.barometer,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        accelerometer: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.accelerometer)).into_owned(),
        gyroscope: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.gyroscope)).into_owned(),
        magnetometer: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.magnetometer)).into_owned(),
      barometer: msg.barometer,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      accelerometer: geometry_msgs::msg::Vector3::from_rmw_message(msg.accelerometer),
      gyroscope: geometry_msgs::msg::Vector3::from_rmw_message(msg.gyroscope),
      magnetometer: geometry_msgs::msg::Vector3::from_rmw_message(msg.magnetometer),
      barometer: msg.barometer,
    }
  }
}


// Corresponds to mav_msgs__msg__GpsWaypoint

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GpsWaypoint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// latitude in degree
    pub latitude: f64,

    /// longitude in degree
    pub longitude: f64,

    /// above start-up point
    pub altitude: f64,

    /// GPS heading
    pub heading: f64,

    /// maximum approach speed
    pub max_speed: f64,

    /// maximum approach accelerations
    pub max_acc: f64,

}



impl Default for GpsWaypoint {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::GpsWaypoint::default())
  }
}

impl rosidl_runtime_rs::Message for GpsWaypoint {
  type RmwMsg = super::msg::rmw::GpsWaypoint;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        latitude: msg.latitude,
        longitude: msg.longitude,
        altitude: msg.altitude,
        heading: msg.heading,
        max_speed: msg.max_speed,
        max_acc: msg.max_acc,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      latitude: msg.latitude,
      longitude: msg.longitude,
      altitude: msg.altitude,
      heading: msg.heading,
      max_speed: msg.max_speed,
      max_acc: msg.max_acc,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      latitude: msg.latitude,
      longitude: msg.longitude,
      altitude: msg.altitude,
      heading: msg.heading,
      max_speed: msg.max_speed,
      max_acc: msg.max_acc,
    }
  }
}


// Corresponds to mav_msgs__msg__RateThrust

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RateThrust {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// We use the coordinate frames with the following convention:
    ///   x: forward
    ///   y: left
    ///   z: up
    /// Roll-, pitch-, yaw-rate around body axes
    pub angular_rates: geometry_msgs::msg::Vector3,

    /// Thrust expressed in the body frame.
    /// For a fixed-wing, usually the x-component is used.
    /// For a multi-rotor, usually the z-component is used.
    /// Set all un-used components to 0.
    pub thrust: geometry_msgs::msg::Vector3,

}



impl Default for RateThrust {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RateThrust::default())
  }
}

impl rosidl_runtime_rs::Message for RateThrust {
  type RmwMsg = super::msg::rmw::RateThrust;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        angular_rates: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.angular_rates)).into_owned(),
        thrust: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.thrust)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        angular_rates: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.angular_rates)).into_owned(),
        thrust: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.thrust)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      angular_rates: geometry_msgs::msg::Vector3::from_rmw_message(msg.angular_rates),
      thrust: geometry_msgs::msg::Vector3::from_rmw_message(msg.thrust),
    }
  }
}


// Corresponds to mav_msgs__msg__RollPitchYawrateThrust

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RollPitchYawrateThrust {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// We use the coordinate frames with the following convention:
    ///   x: forward
    ///   y: left
    ///   z: up
    /// rotation convention (z-y'-x''):
    /// yaw rotates around fixed frame's z axis
    /// pitch rotates around new y-axis (y')
    /// roll rotates around new x-axis (x'')
    /// This is a convenience-message to support that low-level (microcontroller-based) state
    /// estimators may not have knowledge about the absolute yaw.
    /// Roll- and pitch-angle should be specified in the header/frame_id frame
    /// Roll angle
    pub roll: f64,

    /// Pitch angle
    pub pitch: f64,

    /// Yaw rate around z-axis
    pub yaw_rate: f64,

    /// Thrust expressed in the body frame.
    /// For a fixed-wing, usually the x-component is used.
    /// For a multi-rotor, usually the z-component is used.
    /// Set all un-used components to 0.
    pub thrust: geometry_msgs::msg::Vector3,

}



impl Default for RollPitchYawrateThrust {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RollPitchYawrateThrust::default())
  }
}

impl rosidl_runtime_rs::Message for RollPitchYawrateThrust {
  type RmwMsg = super::msg::rmw::RollPitchYawrateThrust;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        roll: msg.roll,
        pitch: msg.pitch,
        yaw_rate: msg.yaw_rate,
        thrust: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.thrust)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      roll: msg.roll,
      pitch: msg.pitch,
      yaw_rate: msg.yaw_rate,
        thrust: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.thrust)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      roll: msg.roll,
      pitch: msg.pitch,
      yaw_rate: msg.yaw_rate,
      thrust: geometry_msgs::msg::Vector3::from_rmw_message(msg.thrust),
    }
  }
}


// Corresponds to mav_msgs__msg__Status

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Status {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// If values are not known / available, set to -1 or empty string.
    pub vehicle_name: std::string::String,

    /// E.g. firefly, pelican ...
    pub vehicle_type: std::string::String,

    /// Battery voltage in V.
    pub battery_voltage: f32,

    /// Command mode set on the 3 position switch on the rc.
    pub rc_command_mode: std::string::String,

    /// Reports whether the serial command interface is enabled.
    pub command_interface_enabled: bool,

    /// Flight time in s.
    pub flight_time: f32,

    /// MAV uptime in s.
    pub system_uptime: f32,

    /// MAV CPU load: 0.0 ... 1.0
    pub cpu_load: f32,

    /// Current motor status: running, stopped, starting, stopping.
    pub motor_status: std::string::String,

    /// True if vehicle is actually in air, false otherwise
    pub in_air: bool,

    /// GPS status: lock, no_lock
    pub gps_status: std::string::String,

    /// Number of visible satellites
    pub gps_num_satellites: i32,

}

impl Status {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RC_COMMAND_ATTITUDE: &'static str = "attitude_thrust";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RC_COMMAND_ATTITUDE_HEIGHT: &'static str = "attitude_height";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RC_COMMAND_POSITION: &'static str = "position";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOTOR_STATUS_RUNNING: &'static str = "running";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOTOR_STATUS_STOPPED: &'static str = "stopped";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOTOR_STATUS_STARTING: &'static str = "starting";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOTOR_STATUS_STOPPING: &'static str = "stopping";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const GPS_STATUS_LOCK: &'static str = "lock";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const GPS_STATUS_NO_LOCK: &'static str = "no_lock";

}


impl Default for Status {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Status::default())
  }
}

impl rosidl_runtime_rs::Message for Status {
  type RmwMsg = super::msg::rmw::Status;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        vehicle_name: msg.vehicle_name.as_str().into(),
        vehicle_type: msg.vehicle_type.as_str().into(),
        battery_voltage: msg.battery_voltage,
        rc_command_mode: msg.rc_command_mode.as_str().into(),
        command_interface_enabled: msg.command_interface_enabled,
        flight_time: msg.flight_time,
        system_uptime: msg.system_uptime,
        cpu_load: msg.cpu_load,
        motor_status: msg.motor_status.as_str().into(),
        in_air: msg.in_air,
        gps_status: msg.gps_status.as_str().into(),
        gps_num_satellites: msg.gps_num_satellites,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        vehicle_name: msg.vehicle_name.as_str().into(),
        vehicle_type: msg.vehicle_type.as_str().into(),
      battery_voltage: msg.battery_voltage,
        rc_command_mode: msg.rc_command_mode.as_str().into(),
      command_interface_enabled: msg.command_interface_enabled,
      flight_time: msg.flight_time,
      system_uptime: msg.system_uptime,
      cpu_load: msg.cpu_load,
        motor_status: msg.motor_status.as_str().into(),
      in_air: msg.in_air,
        gps_status: msg.gps_status.as_str().into(),
      gps_num_satellites: msg.gps_num_satellites,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      vehicle_name: msg.vehicle_name.to_string(),
      vehicle_type: msg.vehicle_type.to_string(),
      battery_voltage: msg.battery_voltage,
      rc_command_mode: msg.rc_command_mode.to_string(),
      command_interface_enabled: msg.command_interface_enabled,
      flight_time: msg.flight_time,
      system_uptime: msg.system_uptime,
      cpu_load: msg.cpu_load,
      motor_status: msg.motor_status.to_string(),
      in_air: msg.in_air,
      gps_status: msg.gps_status.to_string(),
      gps_num_satellites: msg.gps_num_satellites,
    }
  }
}


// Corresponds to mav_msgs__msg__TorqueThrust

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TorqueThrust {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// We use the coordinate frames with the following convention:
    ///   x: forward
    ///   y: left
    ///   z: up
    /// Torque expressed in the body frame.
    pub torque: geometry_msgs::msg::Vector3,

    /// Thrust expressed in the body frame.
    /// For a fixed-wing, usually the x-component is used.
    /// For a multi-rotor, usually the z-component is used.
    /// Set all un-used components to 0.
    pub thrust: geometry_msgs::msg::Vector3,

}



impl Default for TorqueThrust {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TorqueThrust::default())
  }
}

impl rosidl_runtime_rs::Message for TorqueThrust {
  type RmwMsg = super::msg::rmw::TorqueThrust;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        torque: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.torque)).into_owned(),
        thrust: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.thrust)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        torque: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.torque)).into_owned(),
        thrust: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.thrust)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      torque: geometry_msgs::msg::Vector3::from_rmw_message(msg.torque),
      thrust: geometry_msgs::msg::Vector3::from_rmw_message(msg.thrust),
    }
  }
}


