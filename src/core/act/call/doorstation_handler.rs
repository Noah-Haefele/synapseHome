use std::sync::{Arc, Mutex, mpsc::Receiver};

use crate::core::act::call::call_handler::CallHandler;
use crate::core::act::mqtt_event::{CallType, MqttEvent};

use crate::core::state::devices::DeviceManager;
use crate::platform::linux::gpio::GpioController;

pub struct DoorstationHandler {
    call_handler: Arc<Mutex<CallHandler>>,
    device_manager: Arc<Mutex<DeviceManager>>,
    gpio_controller: Arc<GpioController>,
    event_receiver: Receiver<MqttEvent>,
}

impl DoorstationHandler {
    pub fn new(
        call_handler: Arc<Mutex<CallHandler>>,
        device_manager: Arc<Mutex<DeviceManager>>,
        gpio_controller: Arc<GpioController>,
        event_receiver: Receiver<MqttEvent>,
    ) -> Self {
        Self {
            call_handler,
            device_manager,
            gpio_controller,
            event_receiver,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Ok(event) = self.event_receiver.recv() {
            println!("[DoorstationHandler] Incoming MQTT event: {:?}", event);
            if let Err(e) = self.handle_event(event) {
                eprintln!("[DoorstationHandler] Error handling event: {}", e);
            }
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
        let location_id = self.get_location_id()?;

        match event {
            MqttEvent::Started {
                caller_id,
                caller_ip,
                call_type,
                caller_device_type,
            } => {
                if caller_id == location_id {
                    return Ok(());
                }
                if let CallType::Direct { callee_id } = call_type {
                    if callee_id != location_id {
                        return Ok(());
                    }
                }
                println!(
                    "[DoorstationHandler] Incoming call from caller {} (IP: {})",
                    caller_id, caller_ip
                );
                let mut call_handler = self
                    .call_handler
                    .lock()
                    .map_err(|_| "Failed to lock CallHandler")?;
                call_handler.incoming_call(caller_id, &caller_ip, call_type, caller_device_type)?;
            }

            MqttEvent::Accepted {
                caller_id,
                callee_id,
                callee_ip,
            } => {
                let mut call_handler = self
                    .call_handler
                    .lock()
                    .map_err(|_| "Failed to lock CallHandler")?;
                if caller_id == location_id {
                    println!(
                        "[DoorstationHandler] Our call was accepted by indoor station {} (IP: {})",
                        callee_id, callee_ip
                    );
                    call_handler.call_accepted(caller_id, callee_id, &callee_ip)?;
                } else if callee_id != location_id {
                    call_handler.cancel_ringing_if_matching(caller_id)?;
                }
            }

            MqttEvent::Ended {
                caller_id,
                callee_id,
                sender_ip,
            } => {
                let mut call_handler = self
                    .call_handler
                    .lock()
                    .map_err(|_| "Failed to lock CallHandler")?;
                call_handler.call_ended(caller_id, callee_id, &sender_ip)?;
            }

            MqttEvent::OpenDoor => {
                println!("[DoorstationHandler] Received OpenDoor MQTT command! Triggering buzzer...");
                self.gpio_controller.trigger_buzzer();
            }
        }

        Ok(())
    }
}
