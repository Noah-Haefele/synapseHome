use std::sync::{Arc, Mutex};

use crate::core::api::grpc_call_server::LiveSignalsService;
use crate::networking::mqtt::mqtt_handler::MqttHandler;

pub struct CallHandler {
    grpc_call_signal_api: LiveSignalsService,
    mqtt_handler: Arc<Mutex<MqttHandler>>,

    call_target_device_id: i32,
}

impl CallHandler {
    pub fn new(
        grpc_call_signal_api: LiveSignalsService,
        mqtt_handler: Arc<Mutex<MqttHandler>>,
    ) -> Self {
        Self {
            grpc_call_signal_api,
            mqtt_handler,

            call_target_device_id: -1,
        }
    }

    // Target_device_id is the id of the call target. Location_id is location id set in the settings
    pub fn initiate_call(
        &mut self,
        target_device_id: i32,
        location_id: i32,
        this_ip_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.call_target_device_id = target_device_id;
        self.grpc_call_signal_api
            .trigger_call_state_changed("CALLING");

        let mqtt_handler = self
            .mqtt_handler
            .lock()
            .map_err(|_| "Failed to lock MqttHandler")?;

        let subtopic = format!("call/{}", target_device_id);
        let topic = format!("CALLING:{}:{}", location_id, this_ip_address);

        mqtt_handler.publish(&subtopic, topic)?;

        Ok(())
    }

    pub fn accept_call(&self) {}

    pub fn end_call(&mut self) {
        self.call_target_device_id = -1;
        self.grpc_call_signal_api.trigger_call_state_changed("IDLE");
    }
}
