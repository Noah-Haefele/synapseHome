use std::sync::{Arc, Mutex};

use crate::core::act::audio::audio::AudioHandler;
use crate::core::api::call_signals_service::CallSignalsService;
use crate::networking::mqtt::mqtt_handler::MqttHandler;

pub struct CallHandler {
    call_signals_service: CallSignalsService,
    mqtt_handler: Arc<Mutex<MqttHandler>>,
    audio_handler: AudioHandler,

    call_device_id: i32,
    call_state: String,
}

/// Methods that are called local meaning in this software by this device
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
            call_state: "IDLE".to_string(),
        }
    }

    // Target_device_id is the id of the call target. Location_id is location id set in the settings
    pub fn initiate_call(
        &mut self,
        target_device_id: i32,
        location_id: i32,
        this_ip_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.call_device_id = target_device_id;
        self.call_state = "CALLING".to_string();

        self.call_signals_service
            .trigger_call_state_changed(&self.call_state);

        let mqtt_handler = self
            .mqtt_handler
            .lock()
            .map_err(|_| "Failed to lock MqttHandler")?;

        let subtopic = format!("call/{}", target_device_id);
        let topic = format!("CALLING:{}:{}", location_id, this_ip_address);

        mqtt_handler.publish(&subtopic, topic)?;

        Ok(())
    }

    pub fn accept_call(
        &mut self,
        location_id: i32,
        this_ip_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.call_state = "CONNECTED".to_string();

        self.call_signals_service
            .trigger_call_state_changed(&self.call_state);

        let mqtt_handler = self
            .mqtt_handler
            .lock()
            .map_err(|_| "Failed to lock MqttHandler")?;

        let subtopic = format!("call/{}", self.call_device_id);
        let topic = format!("ACCEPTED:{}:{}", location_id, this_ip_address);

        mqtt_handler.publish(&subtopic, topic)?;

        Ok(())
    }

    pub fn end_call(
        &mut self,
        location_id: i32,
        this_ip_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.call_state = "IDLE".to_string();

        self.call_signals_service
            .trigger_call_state_changed(&self.call_state);

        let mqtt_handler = self
            .mqtt_handler
            .lock()
            .map_err(|_| "Failed to lock MqttHandler")?;

        let subtopic = format!("call/{}", self.call_device_id);
        let topic = format!("END:{}:{}", location_id, this_ip_address);

        mqtt_handler.publish(&subtopic, topic)?;

        self.call_device_id = -1;

        self.audio_handler.pause_net_audio()?;

        Ok(())
    }

    pub fn get_call_device_id(&self) -> i32 {
        self.call_device_id
    }

    pub fn get_call_label(&self, device_name: &str) -> String {
        match self.call_state.as_str() {
            "IDLE" => format!(
                "Error: Application wantet call label although no call was started. Device-name: {}",
                device_name
            ),
            "CALLING" => format!("{} ...", device_name),
            "RINGING" => format!("{} is calling", device_name),
            "CONNECTED" => device_name.to_string(),
            _ => unreachable!("Invalid call state: {}", self.call_state),
        }
    }
}

/// Methods that are called when another device acts and sends a mqtt message
impl CallHandler {
    pub fn incoming_call(
        &mut self,
        source_device_id: i32,
        source_ip_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.call_state = "RINGING".to_string();
        self.call_device_id = source_device_id;

        self.call_signals_service
            .trigger_call_state_changed(&self.call_state);

        self.audio_handler.start_net_audio(source_ip_address)?;

        Ok(())
    }

    pub fn call_accepted(
        &mut self,
        source_device_id: i32,
        source_ip_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.call_device_id != source_device_id {
            panic!(
                "The call device id is not equal with the device id given via mqtt by the call device id"
            );
        }

        self.call_state = "CONNECTED".to_string();

        self.call_signals_service
            .trigger_call_state_changed(&self.call_state);

        self.audio_handler.start_net_audio(source_ip_address)?;

        Ok(())
    }

    pub fn call_ended(
        &mut self,
        source_device_id: i32,
        source_ip_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.call_device_id != source_device_id {
            panic!(
                "The call device id is not equal with the device id given via mqtt by the call device id"
            );
        }

        self.call_state = "IDLE".to_string();

        self.call_signals_service
            .trigger_call_state_changed(&self.call_state);

        self.audio_handler.pause_net_audio()?;

        Ok(())
    }
}
