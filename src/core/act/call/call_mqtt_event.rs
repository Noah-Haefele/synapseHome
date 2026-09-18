use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "callType", rename_all = "snake_case")]
pub enum CallType {
    Direct { callee_id: i32 }, // Id of target device
    Group,                     // No target device id necessary because everyone is target
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "deviceType", rename_all = "snake_case")]
pub enum DeviceType {
    Device,     // Normal device (with display)
    DoorDevice, // Door device (with door and buzzer)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum CallMessage {
    Started {
        #[serde(rename = "callerId")]
        caller_id: i32, // Id of device initiating call
        #[serde(rename = "callerIp")]
        caller_ip: String, // IP of device initiating call
        #[serde(rename = "callType")]
        call_type: CallType, // CallType: either Direct (Two devices) or Group (Multiple intended targets)
        #[serde(rename = "deviceType")]
        caller_device_type: DeviceType, // => Tells the device_type of the caller
    },
    Accepted {
        #[serde(rename = "callerId")]
        caller_id: i32,
        #[serde(rename = "calleeId")]
        callee_id: i32,
        #[serde(rename = "calleeIp")]
        callee_ip: String,
    },
    Ended {
        #[serde(rename = "callerId")]
        caller_id: i32,
        #[serde(rename = "calleeId")]
        callee_id: Option<i32>, // Can be None (e.g. unanswered group-call)
        #[serde(rename = "senderIp")]
        sender_ip: String,
    },
}
