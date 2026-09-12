use std::sync::{Arc, Mutex, mpsc::Receiver};

use crate::core::act::call::call_handler::CallHandler;
use crate::core::act::call::call_mqtt_event::CallEvent;
use crate::core::state::devices::DeviceManager;

pub struct CallEventHandler {
    call_handler: Arc<Mutex<CallHandler>>,
    device_manager: Arc<Mutex<DeviceManager>>,
    event_receiver: Receiver<CallEvent>,
}

impl CallEventHandler {
    pub fn new(
        call_handler: Arc<Mutex<CallHandler>>,
        device_manager: Arc<Mutex<DeviceManager>>,
        event_receiver: Receiver<CallEvent>,
    ) -> Self {
        Self {
            call_handler,
            device_manager,
            event_receiver,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Ok(event) = self.event_receiver.recv() {
            self.handle_event(event)?;
        }
        Ok(())
    }

    fn get_location_id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let device_manager = self
            .device_manager
            .lock()
            .map_err(|_| "Failed to lock DeviceManager")?;

        Ok(device_manager.get_location_id())
    }

    fn handle_event(&self, event: CallEvent) -> Result<(), Box<dyn std::error::Error>> {
        let mut call_handler = self
            .call_handler
            .lock()
            .map_err(|_| "Failed to lock CallHandler")?;

        // Check if device id of the requesting device is not equal with this device id
        // This check is important because of the call/all topic everyone is subscribing to
        let location_id = self.get_location_id()?;
        if matches!(
            event,
            CallEvent::Calling { source_device_id, .. }
            | CallEvent::Accepted { source_device_id, .. }
            | CallEvent::End { source_device_id, .. }
            if source_device_id == location_id
        ) {
            return Ok(());
        }

        match event {
            CallEvent::Calling {
                source_device_id,
                source_ip_address,
            } => {
                println!(
                    "Device with Id: {} and Ip: {} is calling",
                    source_device_id, source_ip_address
                );
                call_handler.incoming_call(source_device_id, &source_ip_address)?;
            }

            CallEvent::Accepted {
                source_device_id,
                source_ip_address,
            } => {
                println!(
                    "Device with Id: {} and Ip: {} has accepted the call",
                    source_device_id, source_ip_address
                );
                call_handler.call_accepted(source_device_id, &source_ip_address)?;
            }

            CallEvent::End {
                source_device_id,
                source_ip_address,
            } => {
                println!(
                    "Device with Id: {} and Ip: {} has ended the call",
                    source_device_id, source_ip_address
                );
                call_handler.call_ended(source_device_id, &source_ip_address)?;
            }
        }

        Ok(())
    }
}
