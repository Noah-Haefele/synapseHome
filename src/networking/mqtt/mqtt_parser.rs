use crate::core::act::call::call_mqtt_event::CallMessage;

pub fn parse(topic: &str, payload: &str) -> Option<CallMessage> {
    if !topic.contains("call") && !topic.contains("broadcast") {
        eprintln!("Unhandled MQTT topic: {}", topic);
        return None;
    }

    match serde_json::from_str::<CallMessage>(payload) {
        Ok(call_message) => Some(call_message),
        Err(e) => {
            eprintln!("Invalid MQTT payload for topic {}: {} ({})", topic, payload, e);
            None
        }
    }
}
