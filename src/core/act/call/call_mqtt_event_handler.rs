use std::sync::{Arc, Mutex, mpsc::Receiver};

use crate::core::act::call::call_handler::CallHandler;
use crate::core::act::mqtt_event::{CallType, MqttEvent};
use crate::core::state::devices::DeviceManager;

pub struct CallEventHandler {
    call_handler: Arc<Mutex<CallHandler>>,
    device_manager: Arc<Mutex<DeviceManager>>,
    event_receiver: Receiver<MqttEvent>,
}

impl CallEventHandler {
    pub fn new(
        call_handler: Arc<Mutex<CallHandler>>,
        device_manager: Arc<Mutex<DeviceManager>>,
        event_receiver: Receiver<MqttEvent>,
    ) -> Self {
        Self {
            call_handler,
            device_manager,
            event_receiver,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Ok(event) = self.event_receiver.recv() {
            println!("Incomming event: {:?}", event);
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

    fn handle_event(&self, event: MqttEvent) -> Result<(), Box<dyn std::error::Error>> {
        let mut call_handler = self
            .call_handler
            .lock()
            .map_err(|_| "Failed to lock CallHandler")?;

        let location_id = self.get_location_id()?;

        match event {
            MqttEvent::Started {
                caller_id,
                caller_ip,
                call_type,
                caller_device_type,
            } => {
                // Ignore own call started message
                if caller_id == location_id {
                    return Ok(());
                }

                // If direct call A -> B, ensure this device is the target callee
                if let CallType::Direct { callee_id } = call_type {
                    if callee_id != location_id {
                        return Ok(());
                    }
                }

                // Checks if caller_device_type matches the device_type written in the devices.json
                let config_device_type = {
                    self.device_manager
                        .lock()
                        .map_err(|e| e.to_string())?
                        .get_device_type(caller_id)
                };
                if config_device_type != Some(caller_device_type) {
                    eprintln!(
                        "The callers device_type is not matching its device_type written in the devices.json"
                    );
                    return Ok(());
                }

                println!(
                    "Device with Id: {} and Ip: {} is calling (type: {:?})",
                    caller_id, caller_ip, call_type
                );
                call_handler.incoming_call(caller_id, &caller_ip, call_type, caller_device_type)?;
            }

            MqttEvent::Accepted {
                caller_id,
                callee_id,
                callee_ip,
            } => {
                if caller_id == location_id {
                    println!(
                        "Device with Id: {} has accepted our call from Ip: {}",
                        callee_id, callee_ip
                    );
                    call_handler.call_accepted(caller_id, callee_id, &callee_ip)?;
                } else if callee_id == location_id {
                    // Accepted locally by this device, ignore echo
                    return Ok(());
                } else {
                    // Another device in broadcast channel accepted the call
                    println!(
                        "Device {} accepted group call from caller {}. Dismissing ringing if active.",
                        callee_id, caller_id
                    );
                    call_handler.cancel_ringing_if_matching(caller_id)?;
                }
            }

            MqttEvent::Ended {
                caller_id,
                callee_id,
                sender_ip,
            } => {
                if caller_id == location_id && callee_id.unwrap_or(-1) == location_id {
                    return Ok(());
                }
                println!(
                    "Call ended message received (caller: {}, callee: {:?}, sender_ip: {})",
                    caller_id, callee_id, sender_ip
                );
                call_handler.call_ended(caller_id, callee_id, &sender_ip)?;
            }

            // Note: Temporary solution only!!!
            _ => return Ok(()),
        }

        Ok(())
    }
}
