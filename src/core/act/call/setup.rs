use std::sync::{Arc, Mutex};

use crate::core::state::devices::DeviceManager;
use crate::networking::mqtt::mqtt_handler::MqttHandler;

pub struct CallSetup {
    mqtt_handler: MqttHandler,
    device_manager: Arc<Mutex<DeviceManager>>,
}

impl CallSetup {
    pub fn new(
        mqtt_handler: MqttHandler,
        device_manager: Arc<Mutex<DeviceManager>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut call_setup = Self {
            mqtt_handler,
            device_manager,
        };

        call_setup.subscribe_to_call_message()?;

        Ok(call_setup)
    }

    pub fn subscribe_to_call_message(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // First unsubscribe old topics
        // This is e.g neccesairy when the device location id chages
        for topic in self.mqtt_handler.get_subtopics() {
            self.mqtt_handler.unsubscribe(topic)?;
        }

        // Make subtopic and subscribe
        let location_id = {
            let device_manager = self
                .device_manager
                .lock()
                .map_err(|e| format!("Failed to lock DeviceManager: {}", e))?;

            device_manager.get_location_id()
        };
        let subtopic = format!("call/{}", location_id);

        self.mqtt_handler.subscribe(subtopic)?;

        Ok(())
    }
}
