use crate::core::act::mqtt_event::MqttEvent;

pub fn parse(topic: &str, payload: &str) -> Option<MqttEvent> {
    if !topic.contains("call") && !topic.contains("broadcast") {
        eprintln!("Unhandled MQTT topic: {}", topic);
        return None;
    }

    match serde_json::from_str::<MqttEvent>(payload) {
        Ok(call_message) => Some(call_message),
        Err(e) => {
            eprintln!(
                "Invalid MQTT payload for topic {}: {} ({})",
                topic, payload, e
            );
            None
        }
    }
}
