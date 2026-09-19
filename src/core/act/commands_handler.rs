use std::sync::{Arc, Mutex};

use crate::core::act::mqtt_event::MqttEvent;
use crate::networking::mqtt::mqtt_handler::MqttHandler;

pub struct CommandsHandler {
    mqtt_handler: Arc<Mutex<MqttHandler>>,
}

impl CommandsHandler {
    pub fn new(mqtt_handler: Arc<Mutex<MqttHandler>>) -> Self {
        Self { mqtt_handler }
    }

    pub fn open_door(&self, target_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mqtt_handler = self
            .mqtt_handler
            .lock()
            .map_err(|_| "Failed to lock MqttHandler")?;

        let subtopic = format!("call/device/{}", target_id);
        let payload = MqttEvent::OpenDoor;
        let payload_str = serde_json::to_string(&payload)?;

        mqtt_handler.publish(&subtopic, payload_str)?;

        Ok(())
    }
}
