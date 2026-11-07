#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "mav_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__Actuators() -> *const std::ffi::c_void;
}

#[link(name = "mav_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_msgs__msg__Actuators__init(msg: *mut Actuators) -> bool;
    fn mav_msgs__msg__Actuators__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Actuators>, size: usize) -> bool;
    fn mav_msgs__msg__Actuators__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Actuators>);
    fn mav_msgs__msg__Actuators__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Actuators>, out_seq: *mut rosidl_runtime_rs::Sequence<Actuators>) -> bool;
}

// Corresponds to mav_msgs__msg__Actuators
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Actuators {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// This message defines lowest level commands to be sent to the actuator(s).
    /// Angle of the actuator in.
    /// E.g. servo angle of a control surface(not angle of the surface!), orientation-angle of a thruster.
    pub angles: rosidl_runtime_rs::Sequence<f64>,

    /// Angular velocities of the actuator in.
    /// E.g. "rpm" of rotors, propellers, thrusters
    pub angular_velocities: rosidl_runtime_rs::Sequence<f64>,

    /// Everything that does not fit the above, normalized between.
    pub normalized: rosidl_runtime_rs::Sequence<f64>,

}



impl Default for Actuators {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_msgs__msg__Actuators__init(&mut msg as *mut _) {
        panic!("Call to mav_msgs__msg__Actuators__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Actuators {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__Actuators__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__Actuators__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__Actuators__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Actuators {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Actuators where Self: Sized {
  const TYPE_NAME: &'static str = "mav_msgs/msg/Actuators";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__Actuators() }
  }
}


#[link(name = "mav_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__AttitudeThrust() -> *const std::ffi::c_void;
}

#[link(name = "mav_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_msgs__msg__AttitudeThrust__init(msg: *mut AttitudeThrust) -> bool;
    fn mav_msgs__msg__AttitudeThrust__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AttitudeThrust>, size: usize) -> bool;
    fn mav_msgs__msg__AttitudeThrust__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AttitudeThrust>);
    fn mav_msgs__msg__AttitudeThrust__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AttitudeThrust>, out_seq: *mut rosidl_runtime_rs::Sequence<AttitudeThrust>) -> bool;
}

// Corresponds to mav_msgs__msg__AttitudeThrust
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AttitudeThrust {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// Attitude expressed in the header/frame_id frame.
    pub attitude: geometry_msgs::msg::rmw::Quaternion,

    /// Thrust expressed in the body frame.
    /// For a fixed-wing, usually the x-component is used.
    /// For a multi-rotor, usually the z-component is used.
    /// Set all un-used components to 0.
    pub thrust: geometry_msgs::msg::rmw::Vector3,

}



impl Default for AttitudeThrust {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_msgs__msg__AttitudeThrust__init(&mut msg as *mut _) {
        panic!("Call to mav_msgs__msg__AttitudeThrust__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AttitudeThrust {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__AttitudeThrust__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__AttitudeThrust__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__AttitudeThrust__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AttitudeThrust {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AttitudeThrust where Self: Sized {
  const TYPE_NAME: &'static str = "mav_msgs/msg/AttitudeThrust";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__AttitudeThrust() }
  }
}


#[link(name = "mav_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__FilteredSensorData() -> *const std::ffi::c_void;
}

#[link(name = "mav_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_msgs__msg__FilteredSensorData__init(msg: *mut FilteredSensorData) -> bool;
    fn mav_msgs__msg__FilteredSensorData__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FilteredSensorData>, size: usize) -> bool;
    fn mav_msgs__msg__FilteredSensorData__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FilteredSensorData>);
    fn mav_msgs__msg__FilteredSensorData__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FilteredSensorData>, out_seq: *mut rosidl_runtime_rs::Sequence<FilteredSensorData>) -> bool;
}

// Corresponds to mav_msgs__msg__FilteredSensorData
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FilteredSensorData {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// acceleration in vehicle frame
    pub accelerometer: geometry_msgs::msg::rmw::Vector3,

    /// rotational velocity in vehicle frame
    pub gyroscope: geometry_msgs::msg::rmw::Vector3,

    /// Magnetometer measurements in vehicle frame
    pub magnetometer: geometry_msgs::msg::rmw::Vector3,

    /// Height from barometer relative to start-up point
    pub barometer: f64,

}



impl Default for FilteredSensorData {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_msgs__msg__FilteredSensorData__init(&mut msg as *mut _) {
        panic!("Call to mav_msgs__msg__FilteredSensorData__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FilteredSensorData {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__FilteredSensorData__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__FilteredSensorData__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__FilteredSensorData__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FilteredSensorData {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FilteredSensorData where Self: Sized {
  const TYPE_NAME: &'static str = "mav_msgs/msg/FilteredSensorData";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__FilteredSensorData() }
  }
}


#[link(name = "mav_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__GpsWaypoint() -> *const std::ffi::c_void;
}

#[link(name = "mav_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_msgs__msg__GpsWaypoint__init(msg: *mut GpsWaypoint) -> bool;
    fn mav_msgs__msg__GpsWaypoint__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GpsWaypoint>, size: usize) -> bool;
    fn mav_msgs__msg__GpsWaypoint__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GpsWaypoint>);
    fn mav_msgs__msg__GpsWaypoint__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GpsWaypoint>, out_seq: *mut rosidl_runtime_rs::Sequence<GpsWaypoint>) -> bool;
}

// Corresponds to mav_msgs__msg__GpsWaypoint
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GpsWaypoint {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_msgs__msg__GpsWaypoint__init(&mut msg as *mut _) {
        panic!("Call to mav_msgs__msg__GpsWaypoint__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GpsWaypoint {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__GpsWaypoint__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__GpsWaypoint__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__GpsWaypoint__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GpsWaypoint {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GpsWaypoint where Self: Sized {
  const TYPE_NAME: &'static str = "mav_msgs/msg/GpsWaypoint";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__GpsWaypoint() }
  }
}


#[link(name = "mav_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__RateThrust() -> *const std::ffi::c_void;
}

#[link(name = "mav_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_msgs__msg__RateThrust__init(msg: *mut RateThrust) -> bool;
    fn mav_msgs__msg__RateThrust__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RateThrust>, size: usize) -> bool;
    fn mav_msgs__msg__RateThrust__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RateThrust>);
    fn mav_msgs__msg__RateThrust__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RateThrust>, out_seq: *mut rosidl_runtime_rs::Sequence<RateThrust>) -> bool;
}

// Corresponds to mav_msgs__msg__RateThrust
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RateThrust {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// We use the coordinate frames with the following convention:
    ///   x: forward
    ///   y: left
    ///   z: up
    /// Roll-, pitch-, yaw-rate around body axes
    pub angular_rates: geometry_msgs::msg::rmw::Vector3,

    /// Thrust expressed in the body frame.
    /// For a fixed-wing, usually the x-component is used.
    /// For a multi-rotor, usually the z-component is used.
    /// Set all un-used components to 0.
    pub thrust: geometry_msgs::msg::rmw::Vector3,

}



impl Default for RateThrust {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_msgs__msg__RateThrust__init(&mut msg as *mut _) {
        panic!("Call to mav_msgs__msg__RateThrust__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RateThrust {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__RateThrust__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__RateThrust__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__RateThrust__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RateThrust {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RateThrust where Self: Sized {
  const TYPE_NAME: &'static str = "mav_msgs/msg/RateThrust";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__RateThrust() }
  }
}


#[link(name = "mav_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__RollPitchYawrateThrust() -> *const std::ffi::c_void;
}

#[link(name = "mav_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_msgs__msg__RollPitchYawrateThrust__init(msg: *mut RollPitchYawrateThrust) -> bool;
    fn mav_msgs__msg__RollPitchYawrateThrust__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RollPitchYawrateThrust>, size: usize) -> bool;
    fn mav_msgs__msg__RollPitchYawrateThrust__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RollPitchYawrateThrust>);
    fn mav_msgs__msg__RollPitchYawrateThrust__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RollPitchYawrateThrust>, out_seq: *mut rosidl_runtime_rs::Sequence<RollPitchYawrateThrust>) -> bool;
}

// Corresponds to mav_msgs__msg__RollPitchYawrateThrust
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RollPitchYawrateThrust {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

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
    pub thrust: geometry_msgs::msg::rmw::Vector3,

}



impl Default for RollPitchYawrateThrust {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_msgs__msg__RollPitchYawrateThrust__init(&mut msg as *mut _) {
        panic!("Call to mav_msgs__msg__RollPitchYawrateThrust__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RollPitchYawrateThrust {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__RollPitchYawrateThrust__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__RollPitchYawrateThrust__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__RollPitchYawrateThrust__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RollPitchYawrateThrust {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RollPitchYawrateThrust where Self: Sized {
  const TYPE_NAME: &'static str = "mav_msgs/msg/RollPitchYawrateThrust";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__RollPitchYawrateThrust() }
  }
}


#[link(name = "mav_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__Status() -> *const std::ffi::c_void;
}

#[link(name = "mav_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_msgs__msg__Status__init(msg: *mut Status) -> bool;
    fn mav_msgs__msg__Status__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Status>, size: usize) -> bool;
    fn mav_msgs__msg__Status__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Status>);
    fn mav_msgs__msg__Status__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Status>, out_seq: *mut rosidl_runtime_rs::Sequence<Status>) -> bool;
}

// Corresponds to mav_msgs__msg__Status
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Status {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// If values are not known / available, set to -1 or empty string.
    pub vehicle_name: rosidl_runtime_rs::String,

    /// E.g. firefly, pelican ...
    pub vehicle_type: rosidl_runtime_rs::String,

    /// Battery voltage in V.
    pub battery_voltage: f32,

    /// Command mode set on the 3 position switch on the rc.
    pub rc_command_mode: rosidl_runtime_rs::String,

    /// Reports whether the serial command interface is enabled.
    pub command_interface_enabled: bool,

    /// Flight time in s.
    pub flight_time: f32,

    /// MAV uptime in s.
    pub system_uptime: f32,

    /// MAV CPU load: 0.0 ... 1.0
    pub cpu_load: f32,

    /// Current motor status: running, stopped, starting, stopping.
    pub motor_status: rosidl_runtime_rs::String,

    /// True if vehicle is actually in air, false otherwise
    pub in_air: bool,

    /// GPS status: lock, no_lock
    pub gps_status: rosidl_runtime_rs::String,

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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_msgs__msg__Status__init(&mut msg as *mut _) {
        panic!("Call to mav_msgs__msg__Status__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Status {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__Status__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__Status__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__Status__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Status {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Status where Self: Sized {
  const TYPE_NAME: &'static str = "mav_msgs/msg/Status";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__Status() }
  }
}


#[link(name = "mav_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__TorqueThrust() -> *const std::ffi::c_void;
}

#[link(name = "mav_msgs__rosidl_generator_c")]
extern "C" {
    fn mav_msgs__msg__TorqueThrust__init(msg: *mut TorqueThrust) -> bool;
    fn mav_msgs__msg__TorqueThrust__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TorqueThrust>, size: usize) -> bool;
    fn mav_msgs__msg__TorqueThrust__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TorqueThrust>);
    fn mav_msgs__msg__TorqueThrust__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TorqueThrust>, out_seq: *mut rosidl_runtime_rs::Sequence<TorqueThrust>) -> bool;
}

// Corresponds to mav_msgs__msg__TorqueThrust
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TorqueThrust {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// We use the coordinate frames with the following convention:
    ///   x: forward
    ///   y: left
    ///   z: up
    /// Torque expressed in the body frame.
    pub torque: geometry_msgs::msg::rmw::Vector3,

    /// Thrust expressed in the body frame.
    /// For a fixed-wing, usually the x-component is used.
    /// For a multi-rotor, usually the z-component is used.
    /// Set all un-used components to 0.
    pub thrust: geometry_msgs::msg::rmw::Vector3,

}



impl Default for TorqueThrust {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mav_msgs__msg__TorqueThrust__init(&mut msg as *mut _) {
        panic!("Call to mav_msgs__msg__TorqueThrust__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TorqueThrust {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__TorqueThrust__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__TorqueThrust__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mav_msgs__msg__TorqueThrust__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TorqueThrust {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TorqueThrust where Self: Sized {
  const TYPE_NAME: &'static str = "mav_msgs/msg/TorqueThrust";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mav_msgs__msg__TorqueThrust() }
  }
}


