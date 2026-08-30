use std::sync::{Arc, Mutex};

use crate::networking::mqtt::mqtt_handler::MqttHandler;

pub struct CallSetup {
    mqtt_handler: Arc<Mutex<MqttHandler>>,
}

impl CallSetup {
    pub fn new(
        mqtt_handler: Arc<Mutex<MqttHandler>>,
        location_id: i32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut call_setup = Self { mqtt_handler };

        call_setup.subscribe_to_call_message(location_id)?;

        Ok(call_setup)
    }

    pub fn subscribe_to_call_message(
        &mut self,
        location_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // First unsubscribe old topics
        // This is e.g neccesairy when the device location id chages
        let mut mqtt_handler = self
            .mqtt_handler
            .lock()
            .map_err(|_| "Failed to lock MqttHandler")?;

        for topic in mqtt_handler.get_subtopics() {
            mqtt_handler.unsubscribe(topic)?;
        }

        // Make subtopic and subscribe
        let subtopic = format!("call/{}", location_id);

        mqtt_handler.subscribe(subtopic)?;

        Ok(())
    }
}
