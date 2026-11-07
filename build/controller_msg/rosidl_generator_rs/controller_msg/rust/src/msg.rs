#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to controller_msg__msg__PIDTune

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PIDTune {

    // This member is not documented.
    #[allow(missing_docs)]
    pub kp: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ki: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub kd: f32,

}



impl Default for PIDTune {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PIDTune::default())
  }
}

impl rosidl_runtime_rs::Message for PIDTune {
  type RmwMsg = super::msg::rmw::PIDTune;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        kp: msg.kp,
        ki: msg.ki,
        kd: msg.kd,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      kp: msg.kp,
      ki: msg.ki,
      kd: msg.kd,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      kp: msg.kp,
      ki: msg.ki,
      kd: msg.kd,
    }
  }
}


