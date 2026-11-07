#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to swift_msgs__msg__SwiftMsgs

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SwiftMsgs {

    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_roll: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_pitch: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_yaw: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_throttle: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_aux1: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_aux2: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_aux3: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rc_aux4: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub drone_index: i64,

}



impl Default for SwiftMsgs {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SwiftMsgs::default())
  }
}

impl rosidl_runtime_rs::Message for SwiftMsgs {
  type RmwMsg = super::msg::rmw::SwiftMsgs;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        rc_roll: msg.rc_roll,
        rc_pitch: msg.rc_pitch,
        rc_yaw: msg.rc_yaw,
        rc_throttle: msg.rc_throttle,
        rc_aux1: msg.rc_aux1,
        rc_aux2: msg.rc_aux2,
        rc_aux3: msg.rc_aux3,
        rc_aux4: msg.rc_aux4,
        drone_index: msg.drone_index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      rc_roll: msg.rc_roll,
      rc_pitch: msg.rc_pitch,
      rc_yaw: msg.rc_yaw,
      rc_throttle: msg.rc_throttle,
      rc_aux1: msg.rc_aux1,
      rc_aux2: msg.rc_aux2,
      rc_aux3: msg.rc_aux3,
      rc_aux4: msg.rc_aux4,
      drone_index: msg.drone_index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      rc_roll: msg.rc_roll,
      rc_pitch: msg.rc_pitch,
      rc_yaw: msg.rc_yaw,
      rc_throttle: msg.rc_throttle,
      rc_aux1: msg.rc_aux1,
      rc_aux2: msg.rc_aux2,
      rc_aux3: msg.rc_aux3,
      rc_aux4: msg.rc_aux4,
      drone_index: msg.drone_index,
    }
  }
}


