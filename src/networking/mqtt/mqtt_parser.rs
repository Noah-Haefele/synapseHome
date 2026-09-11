use crate::core::act::call::call_mqtt_event::CallEvent;

pub fn parse(topic: &str, payload: &str) -> Option<CallEvent> {
    let Some(event) = topic.split('/').nth(1) else {
        eprintln!("Invalid MQTT topic: {}", topic);
        return None;
    };

    match event {
        "call" => parse_call(payload),
        _ => None,
    }
}

fn parse_call(payload: &str) -> Option<CallEvent> {
    let mut call_event = payload.split(':');

    match call_event.next()? {
        "CALLING" => {
            let source_device_id = call_event.next()?.parse::<i32>().ok()?;
            let source_ip_address = call_event.next()?.to_string();

            Some(CallEvent::Calling {
                source_device_id,
                source_ip_address,
            })
        }

        "ACCEPTED" => {
            let source_device_id = call_event.next()?.parse::<i32>().ok()?;
            let source_ip_address = call_event.next()?.to_string();

            Some(CallEvent::Accepted {
                source_device_id,
                source_ip_address,
            })
        }

        "END" => {
            let source_device_id = call_event.next()?.parse::<i32>().ok()?;
            let source_ip_address = call_event.next()?.to_string();

            Some(CallEvent::End {
                source_device_id,
                source_ip_address,
            })
        }

        _ => None,
    }
}
