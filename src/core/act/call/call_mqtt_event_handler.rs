use std::sync::{Arc, Mutex, mpsc::Receiver};

use crate::core::act::call::call_handler::CallHandler;
use crate::core::act::call::call_mqtt_event::{CallMessage, CallType};
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

        let location_id = self.get_location_id()?;

        match event {
            CallMessage::Started {
                caller_id,
                caller_ip,
                call_type,
            } => {
                // Not react to own call
                if caller_id == location_id {
                    return Ok(());
                }

                // Check if call is a direct call A -> B
                if let CallType::Direct { callee_id } = call_type {
                    // Should not happen but if the call is not determined for this device
                    if callee_id != location_id {
                        return Ok(());
                    }
                }

                println!(
                    "Device with Id: {} and Ip: {} is calling (type: {:?})",
                    caller_id, caller_ip, call_type
                );
                call_handler.incoming_call(caller_id, &caller_ip, call_type)?;
            }

            CallMessage::Accepted {
                caller_id,
                callee_id,
                callee_ip,
            } => {
                // Checks if the caller is this device
                if caller_id == location_id {
                    println!(
                        "Device with Id: {} has accepted our call from Ip: {}",
                        callee_id, callee_ip
                    );
                    call_handler.call_accepted(caller_id, callee_id, &callee_ip)?;
                } else if callee_id == location_id {
                    // Not react to own acceptance
                    return Ok(());
                } else {
                    // Anybody else in the broadcast channel has accepted the call
                    println!(
                        "Device {} accepted group call from caller {}. Dismissing ringing if active.",
                        callee_id, caller_id
                    );
                    call_handler.cancel_ringing_if_matching(caller_id)?;
                }
            }

            CallMessage::Ended {
                caller_id,
                callee_id,
                my_ip,
            } => {
                if caller_id == location_id && callee_id.unwrap_or(-1) == location_id {
                    return Ok(());
                }
                println!(
                    "Call between caller {} and callee {:?} ended",
                    caller_id, callee_id
                );
                call_handler.call_ended(caller_id, &my_ip)?;
            }
        }

        Ok(())
    }
}
