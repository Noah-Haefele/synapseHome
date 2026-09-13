use crate::core::act::call::call_mqtt_event::CallMessage;

pub fn parse(topic: &str, payload: &str) -> Option<CallMessage> {
    let Some(event) = topic.split('/').nth(1) else {
        eprintln!("Invalid MQTT topic: {}", topic);
        return None;
    };

    match event {
        "call" => parse_call(payload),
        _ => None,
    }
}

fn parse_call(payload: &str) -> Option<CallMessage> {
    let Some(call_message): Option<CallMessage> = serde_json::from_str(payload).ok() else {
        eprintln!("Invalid MQTT payload: {}", payload.to_string());
        return None;
    };
    Some(call_message)
}
