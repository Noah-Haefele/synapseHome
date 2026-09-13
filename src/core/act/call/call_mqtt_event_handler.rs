use std::sync::{Arc, Mutex, mpsc::Receiver};

use crate::core::act::call::call_handler::CallHandler;
use crate::core::act::call::call_mqtt_event::CallMessage;
use crate::core::state::devices::DeviceManager;

pub struct CallEventHandler {
    call_handler: Arc<Mutex<CallHandler>>,
    device_manager: Arc<Mutex<DeviceManager>>,
    event_receiver: Receiver<CallMessage>,
}

impl CallEventHandler {
    pub fn new(
        call_handler: Arc<Mutex<CallHandler>>,
        device_manager: Arc<Mutex<DeviceManager>>,
        event_receiver: Receiver<CallMessage>,
    ) -> Self {
        Self {
            call_handler,
            device_manager,
            event_receiver,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Ok(event) = self.event_receiver.recv() {
            println!("{:?}", event);
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

    fn handle_event(&self, event: CallMessage) -> Result<(), Box<dyn std::error::Error>> {
        let mut call_handler = self
            .call_handler
            .lock()
            .map_err(|_| "Failed to lock CallHandler")?;

        // Check if device id of the requesting device is not equal with this device id
        // This check is important because of the call/all topic everyone is subscribing to
        let location_id = self.get_location_id()?;
        if matches!(
            event,
            CallMessage::Started { caller_id, ..}
            | CallMessage::Accepted { caller_id, .. }
            | CallMessage::Ended { caller_id, .. }
            if caller_id == location_id
        ) {
            return Ok(());
        }

        match event {
            CallMessage::Started {
                caller_id,
                caller_ip,
                call_type,
            } => {
                println!(
                    "Device with Id: {} and Ip: {} is calling",
                    caller_id, caller_ip
                );
                call_handler.incoming_call(caller_id, &caller_ip)?;
            }

            CallMessage::Accepted {
                caller_id,
                callee_id,
                callee_ip,
            } => {
                println!(
                    "Device with Id: {} and Ip: {} has accepted the call",
                    caller_id, callee_ip
                );
                call_handler.call_accepted(caller_id, &callee_ip)?;
            }

            CallMessage::Ended {
                caller_id,
                callee_id,
                callee_ip,
            } => {
                println!(
                    "Device with Id: {} and Ip: {} has ended the call",
                    caller_id, callee_ip
                );
                call_handler.call_ended(caller_id, &callee_ip)?;
            }
        }

        Ok(())
    }
}
