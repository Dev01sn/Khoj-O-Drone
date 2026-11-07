#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to error_msg__msg__Error

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Error {

    // This member is not documented.
    #[allow(missing_docs)]
    pub roll_error: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pitch_error: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub throttle_error: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_error: f32,

}



impl Default for Error {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Error::default())
  }
}

impl rosidl_runtime_rs::Message for Error {
  type RmwMsg = super::msg::rmw::Error;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        roll_error: msg.roll_error,
        pitch_error: msg.pitch_error,
        throttle_error: msg.throttle_error,
        yaw_error: msg.yaw_error,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      roll_error: msg.roll_error,
      pitch_error: msg.pitch_error,
      throttle_error: msg.throttle_error,
      yaw_error: msg.yaw_error,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      roll_error: msg.roll_error,
      pitch_error: msg.pitch_error,
      throttle_error: msg.throttle_error,
      yaw_error: msg.yaw_error,
    }
  }
}


