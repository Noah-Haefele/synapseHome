use std::sync::{Arc, Mutex};

use crate::core::act::call::call_mqtt_event::CallMessage;
use crate::core::act::call::call_mqtt_event::CallType;

use crate::core::act::audio::audio::AudioHandler;
use crate::core::api::call_signals_service::CallSignalsService;
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
    call_type: Option<CallType>,
    call_state: CallState,
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
            call_type: None,
            call_state: CallState::Idle,
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
            CallState::InternalCall(InternalCall::Connected) => "CONNECTED",
        }
    }
}

/// Methods that are called locally by this device
impl CallHandler {
    // Target_device_id is the id of the call target. Location_id is location id set in the settings
    pub fn initiate_call(
        &mut self,
        target_device_id: i32,
        location_id: i32,
        this_ip_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.call_device_id = target_device_id;
        self.call_type = Some(CallType::Direct {
            callee_id: target_device_id,
        });
        self.set_call_state(CallState::InternalCall(InternalCall::Calling));

        let mqtt_handler = self
            .mqtt_handler
            .lock()
            .map_err(|_| "Failed to lock MqttHandler")?;

        let subtopic = format!("call/device/{}", target_device_id);
        let payload = CallMessage::Started {
            caller_id: location_id,
            caller_ip: this_ip_address.to_string(),
            call_type: CallType::Direct {
                callee_id: target_device_id,
            },
        };
        let payload_str = serde_json::to_string(&payload)?;

        mqtt_handler.publish(&subtopic, payload_str)?;

        Ok(())
    }

    pub fn initiate_call_all(
        &mut self,
        location_id: i32,
        this_ip_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.call_device_id = -1;
        self.call_type = Some(CallType::Group);
        self.set_call_state(CallState::RequestAll);

        let mqtt_handler = self
            .mqtt_handler
            .lock()
            .map_err(|_| "Failed to lock MqttHandler")?;

        let subtopic = "broadcast";
        let payload = CallMessage::Started {
            caller_id: location_id,
            caller_ip: this_ip_address.to_string(),
            call_type: CallType::Group,
        };
        let payload_str = serde_json::to_string(&payload)?;

        mqtt_handler.publish(subtopic, payload_str)?;

        Ok(())
    }

    pub fn accept_call(
        &mut self,
        location_id: i32,
        this_ip_address: &str,
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
            callee_id: location_id,
            callee_ip: this_ip_address.to_string(),
        };
        let payload_str = serde_json::to_string(&payload)?;

        mqtt_handler.publish(&subtopic, payload_str)?;

        Ok(())
    }

    pub fn end_call(
        &mut self,
        location_id: i32,
        this_ip_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let is_request_all = matches!(self.call_state, CallState::RequestAll);
        let is_calling = matches!(
            self.call_state,
            CallState::InternalCall(InternalCall::Calling)
        );

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

        // Checks if the caller is this device by checking if this devices is making a request (either to everyone or to one device)
        let caller_id = if is_request_all || is_calling {
            location_id
        } else {
            // caller_id will be the device, this device is calling
            self.call_device_id
        };

        // Checks if the device is making a reqeust to everyone meaning there is no target (no callee)
        let callee_id: Option<i32> = if is_request_all {
            None
        } else if is_calling {
            // Checks if the device is calling some device -> Its ID will be the callee_id
            Some(self.call_device_id)
        } else {
            // callee_id will be the device itself -> Location ID
            Some(location_id)
        };

        let payload = CallMessage::Ended {
            caller_id,
            callee_id,
            my_ip: this_ip_address.to_string(),
        };
        let payload_str = serde_json::to_string(&payload)?;

        mqtt_handler.publish(&subtopic, payload_str)?;

        self.call_device_id = -1;
        self.call_type = None;

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

/// Methods that are called when another device acts and sends a mqtt message
impl CallHandler {
    pub fn incoming_call(
        &mut self,
        source_device_id: i32,
        source_ip_address: &str,
        call_type: CallType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.set_call_state(CallState::InternalCall(InternalCall::Ringing));
        self.call_device_id = source_device_id;
        self.call_type = Some(call_type);

        self.audio_handler.start_net_audio(source_ip_address)?;

        Ok(())
    }

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
            self.call_type = None;
            self.audio_handler.pause_net_audio()?;
        }
        Ok(())
    }

    pub fn call_accepted(
        &mut self,
        source_device_id: i32,
        callee_id: i32,
        source_ip_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Group call got accepted from another device
        if matches!(self.call_state, CallState::RequestAll) {
            self.call_device_id = callee_id;
            self.call_type = Some(CallType::Group);
            self.set_call_state(CallState::InternalCall(InternalCall::Connected));
            self.audio_handler.start_net_audio(source_ip_address)?;
            return Ok(());
        }

        // Normal device call got accepted
        if self.call_device_id == source_device_id || self.call_device_id == callee_id {
            self.set_call_state(CallState::InternalCall(InternalCall::Connected));
            self.audio_handler.start_net_audio(source_ip_address)?;
        } else {
            eprintln!(
                "Received call_accepted from device {}/{} but current call_device_id is {}",
                source_device_id, callee_id, self.call_device_id
            );
        }

        Ok(())
    }

    pub fn call_ended(
        &mut self,
        source_device_id: i32,
        _source_ip_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.call_device_id == source_device_id
            || matches!(
                self.call_state,
                CallState::InternalCall(InternalCall::Ringing)
            )
        {
            self.set_call_state(CallState::Idle);
            self.call_device_id = -1;
            self.call_type = None;
            self.audio_handler.pause_net_audio()?;
        } else {
            eprintln!(
                "Received call_ended from device {} but current call_device_id is {}",
                source_device_id, self.call_device_id
            );
        }

        Ok(())
    }
}
