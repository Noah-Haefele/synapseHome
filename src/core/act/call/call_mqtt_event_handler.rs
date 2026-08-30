use std::sync::{Arc, Mutex, mpsc::Receiver};

use crate::core::act::call::call_handler::CallHandler;
use crate::core::act::call::call_mqtt_event::CallEvent;

pub struct CallEventHandler {
    call_handler: Arc<Mutex<CallHandler>>,
    event_receiver: Receiver<CallEvent>,
}

impl CallEventHandler {
    pub fn new(call_handler: Arc<Mutex<CallHandler>>, event_receiver: Receiver<CallEvent>) -> Self {
        Self {
            call_handler,
            event_receiver,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Ok(event) = self.event_receiver.recv() {
            self.handle_event(event)?;
        }
        Ok(())
    }

    fn handle_event(&self, event: CallEvent) -> Result<(), Box<dyn std::error::Error>> {
        let mut call_handler = self
            .call_handler
            .lock()
            .map_err(|_| "Failed to lock CallHandler")?;

        match event {
            CallEvent::Calling {
                source_device_id,
                source_ip_address,
            } => {
                println!(
                    "Device with Id: {} and Ip: {} is calling",
                    source_device_id, source_ip_address
                );
                call_handler.incoming_call(source_device_id, &source_ip_address);
            }

            CallEvent::Accepted {
                source_device_id,
                source_ip_address,
            } => {
                println!(
                    "Device with Id: {} and Ip: {} has accepted the call",
                    source_device_id, source_ip_address
                );
                call_handler.call_accepted(source_device_id, &source_ip_address);
            }

            CallEvent::End {
                source_device_id,
                source_ip_address,
            } => {
                println!(
                    "Device with Id: {} and Ip: {} has ended the call",
                    source_device_id, source_ip_address
                );
                call_handler.call_ended(source_device_id, &source_ip_address);
            }
        }

        Ok(())
    }
}
