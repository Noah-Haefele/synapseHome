use std::sync::{Arc, Mutex};

use crate::core::act::call::call_mqtt_event::CallMessage;
use crate::core::act::call::call_mqtt_event::CallType;
use crate::core::act::call::call_mqtt_event::DeviceType;

use crate::core::act::audio::audio::AudioHandler;
use crate::core::api::call_signals_service::CallSignalsService;
use crate::core::state::devices::Device;
use crate::networking::mqtt::mqtt_handler::MqttHandler;

/// States of internal call (indoor stations)
enum InternalCall {
    Ringing,
    Calling,
    Connected,
}

/// State of the call handler
enum CallState {
    Idle,
    RequestAll,
    InternalCall(InternalCall),
}

pub struct CallHandler {
    call_signals_service: CallSignalsService,
    mqtt_handler: Arc<Mutex<MqttHandler>>,
    audio_handler: AudioHandler,

    call_device_id: i32,
    is_caller: bool,
    call_type: Option<CallType>,
    call_state: CallState,
    caller_device_type: Option<DeviceType>,
}

impl CallHandler {
    pub fn new(
        call_signals_service: CallSignalsService,
        mqtt_handler: Arc<Mutex<MqttHandler>>,
        audio_handler: AudioHandler,
    ) -> Self {
        Self {
            call_signals_service,
            mqtt_handler,
            audio_handler,

            call_device_id: -1,
            is_caller: false,
            call_type: None,
            call_state: CallState::Idle,
            caller_device_type: None,
        }
    }

    /// Sets call state and notifies frontend
    fn set_call_state(&mut self, state: CallState) {
        self.call_state = state;

        self.call_signals_service
            .trigger_call_state_changed(self.decide_ui_call_state());
    }

    /// Decides from CallState enum the proper CallState for qml to display correct call view
    /// Frontend wants either IDLE, CALLING, RINGING, CONNECTED
    fn decide_ui_call_state(&self) -> &'static str {
        match self.call_state {
            CallState::Idle => "IDLE",
            CallState::RequestAll => "CALLING",
            CallState::InternalCall(InternalCall::Calling) => "CALLING",
            CallState::InternalCall(InternalCall::Ringing) => "RINGING",
            CallState::InternalCall(InternalCall::Connected) => match self.caller_device_type {
                Some(DeviceType::Device) => "CONNECTED:DEVICE",
                Some(DeviceType::DoorDevice) => "CONNECTED:DOOR_DEVICE",
                None => {
                    eprintln!("No caller_device_type available");
                    "CONNECTED:UNKNOWN"
                }
            },
        }
    }
}

/// Methods that are called locally by this device
impl CallHandler {
    // callee_id is the id of the call target device. caller_id is location_id of this device.
    pub fn initiate_call(
        &mut self,
        callee_id: i32,
        caller_id: i32,
        caller_ip: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.call_device_id = callee_id;
        self.is_caller = true;
        self.call_type = Some(CallType::Direct { callee_id });
        self.caller_device_type = Some(DeviceType::Device);
        self.set_call_state(CallState::InternalCall(InternalCall::Calling));

        let mqtt_handler = self
            .mqtt_handler
            .lock()
            .map_err(|_| "Failed to lock MqttHandler")?;

        let subtopic = format!("call/device/{}", callee_id);
        let payload = CallMessage::Started {
            caller_id,
            caller_ip: caller_ip.to_string(),
            call_type: CallType::Direct { callee_id },
            caller_device_type: DeviceType::Device,
        };
        let payload_str = serde_json::to_string(&payload)?;

        mqtt_handler.publish(&subtopic, payload_str)?;

        Ok(())
    }

    pub fn initiate_call_all(
        &mut self,
        caller_id: i32,
        caller_ip: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.call_device_id = -1;
        self.is_caller = true;
        self.call_type = Some(CallType::Group);
        self.caller_device_type = Some(DeviceType::Device);
        self.set_call_state(CallState::RequestAll);

        let mqtt_handler = self
            .mqtt_handler
            .lock()
            .map_err(|_| "Failed to lock MqttHandler")?;

        let subtopic = "broadcast";
        let payload = CallMessage::Started {
            caller_id,
            caller_ip: caller_ip.to_string(),
            call_type: CallType::Group,
            caller_device_type: DeviceType::Device,
        };
        let payload_str = serde_json::to_string(&payload)?;

        mqtt_handler.publish(subtopic, payload_str)?;

        Ok(())
    }

    pub fn accept_call(
        &mut self,
        callee_id: i32,
        callee_ip: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.set_call_state(CallState::InternalCall(InternalCall::Connected));

        let mqtt_handler = self
            .mqtt_handler
            .lock()
            .map_err(|_| "Failed to lock MqttHandler")?;

        let subtopic = match &self.call_type {
            Some(CallType::Group) => "broadcast".to_string(),
            _ => format!("call/device/{}", self.call_device_id),
        };

        let payload = CallMessage::Accepted {
            caller_id: self.call_device_id,
            callee_id,
            callee_ip: callee_ip.to_string(),
        };
        let payload_str = serde_json::to_string(&payload)?;

        mqtt_handler.publish(&subtopic, payload_str)?;

        Ok(())
    }

    pub fn end_call(
        &mut self,
        my_location_id: i32,
        my_ip: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let is_request_all = matches!(self.call_state, CallState::RequestAll);

        self.set_call_state(CallState::Idle);

        let mqtt_handler = self
            .mqtt_handler
            .lock()
            .map_err(|_| "Failed to lock MqttHandler")?;

        let subtopic = if is_request_all || matches!(self.call_type, Some(CallType::Group)) {
            "broadcast".to_string()
        } else {
            format!("call/device/{}", self.call_device_id)
        };

        // Case 1: This device started a group call -> No calle_id
        // Case 2: This device started a normal call
        // Case 3: This device is being called
        let (caller_id, callee_id) = if is_request_all {
            (my_location_id, None)
        } else if self.is_caller {
            (
                my_location_id,
                if self.call_device_id != -1 {
                    Some(self.call_device_id)
                } else {
                    None
                },
            )
        } else {
            (self.call_device_id, Some(my_location_id))
        };

        let payload = CallMessage::Ended {
            caller_id,
            callee_id,
            sender_ip: my_ip.to_string(),
        };
        let payload_str = serde_json::to_string(&payload)?;

        mqtt_handler.publish(&subtopic, payload_str)?;

        self.call_device_id = -1;
        self.is_caller = false;
        self.call_type = None;
        self.caller_device_type = None;

        self.audio_handler.pause_net_audio()?;

        Ok(())
    }

    pub fn get_call_device_id(&self) -> i32 {
        self.call_device_id
    }

    pub fn get_call_label(&self, device_name: &str) -> String {
        match self.call_state {
            CallState::Idle => "".to_string(),
            CallState::RequestAll => "Calling everyone ...".to_string(),
            CallState::InternalCall(InternalCall::Calling) => format!("{} ...", device_name),
            CallState::InternalCall(InternalCall::Ringing) => format!("{} is calling", device_name),
            CallState::InternalCall(InternalCall::Connected) => device_name.to_string(),
        }
    }
}

/// Methods that are called when another device acts and sends an MQTT message
impl CallHandler {
    pub fn incoming_call(
        &mut self,
        caller_id: i32,
        caller_ip: &str,
        call_type: CallType,
        caller_device_type: DeviceType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.set_call_state(CallState::InternalCall(InternalCall::Ringing));
        self.call_device_id = caller_id;
        self.is_caller = false;
        self.call_type = Some(call_type);
        self.caller_device_type = Some(caller_device_type);

        self.audio_handler.start_net_audio(caller_ip)?;

        Ok(())
    }

    /// Should be triggered when a group call was started but another device already accepted the call
    pub fn cancel_ringing_if_matching(
        &mut self,
        caller_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if matches!(
            self.call_state,
            CallState::InternalCall(InternalCall::Ringing)
        ) && self.call_device_id == caller_id
        {
            println!(
                "Call from caller {} was accepted or cancelled by another device. Ending ringing.",
                caller_id
            );
            self.set_call_state(CallState::Idle);
            self.call_device_id = -1;
            self.is_caller = false;
            self.call_type = None;
            self.caller_device_type = None;
            self.audio_handler.pause_net_audio()?;
        }
        Ok(())
    }

    pub fn call_accepted(
        &mut self,
        caller_id: i32,
        callee_id: i32,
        callee_ip: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Group call got accepted from another device while we were waiting in RequestAll
        if matches!(self.call_state, CallState::RequestAll) {
            self.call_device_id = callee_id;
            self.call_type = Some(CallType::Group);
            self.set_call_state(CallState::InternalCall(InternalCall::Connected));
            self.audio_handler.start_net_audio(callee_ip)?;
            return Ok(());
        }

        // Direct call got accepted
        if self.call_device_id == callee_id || self.call_device_id == caller_id {
            self.set_call_state(CallState::InternalCall(InternalCall::Connected));
            self.audio_handler.start_net_audio(callee_ip)?;
        } else {
            eprintln!(
                "Received call_accepted for caller {} / callee {} but current call_device_id is {}",
                caller_id, callee_id, self.call_device_id
            );
        }

        Ok(())
    }

    pub fn call_ended(
        &mut self,
        caller_id: i32,
        callee_id: Option<i32>,
        _sender_ip: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // True if device is in some state of a call (whether call_device_id is equal with caller_id or callee_id)
        let is_active_call_participant = self.call_device_id != -1
            && (self.call_device_id == caller_id
                || callee_id.map_or(false, |id| id == self.call_device_id));

        if is_active_call_participant {
            println!(
                "Call ended between caller {} and callee {:?}",
                caller_id, callee_id
            );
            self.set_call_state(CallState::Idle);
            self.call_device_id = -1;
            self.is_caller = false;
            self.call_type = None;
            self.caller_device_type = None;
            self.audio_handler.pause_net_audio()?;
        } else {
            println!(
                "Ignoring call_ended for caller {} and callee {:?} (current call_device_id is {})",
                caller_id, callee_id, self.call_device_id
            );
        }

        Ok(())
    }
}
