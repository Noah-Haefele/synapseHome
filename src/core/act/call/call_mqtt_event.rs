use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CallType {
    Direct { callee_id: i32 }, // Id of target device
    Group,                     // No target device id neccessairy because everyone is target
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CallMessage {
    Started {
        caller_id: i32,      // Id of device initiating call
        caller_ip: String,   // IP of device initiating call
        call_type: CallType, // CallType: either Direct (Two devices) or Group (Multiple intended targets)
    },
    Accepted {
        caller_id: i32,
        callee_id: i32,
        callee_ip: String,
    },
    Ended {
        caller_id: i32,
        callee_id: i32,
        callee_ip: String,
    },
}
