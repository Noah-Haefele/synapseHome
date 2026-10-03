use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(feature = "door_device")]
use rppal::gpio::{Gpio, OutputPin};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoorDeviceConfig {
    pub location_id: i32,
    pub buzzer_pin: u32,
    pub ring_pin: u32,
    pub open_pin: u32,
    pub buzz_duration_ms: u64,
}

impl DoorDeviceConfig {
    pub fn load_or_default(config_path: &Path) -> Self {
        if config_path.is_file() {
            if let Ok(content) = fs::read_to_string(config_path) {
                if let Ok(cfg) = serde_json::from_str(&content) {
                    return cfg;
                }
            }
        }
        Self {
            location_id: 4,
            buzzer_pin: 18,
            ring_pin: 23,
            open_pin: 24,
            buzz_duration_ms: 3000,
        }
    }
}

pub struct GpioController {
    #[cfg(feature = "door_device")]
    buzzer_output: Arc<Mutex<Option<OutputPin>>>,
    buzzer_pin_num: u32,
    ring_pin_num: u32,
    open_pin_num: u32,
    buzz_duration_ms: u64,
    buzzer_active: Arc<Mutex<bool>>,
    rppal_active: bool,
}

impl GpioController {
    pub fn new(config: &DoorDeviceConfig) -> Self {
        #[cfg(feature = "door_device")]
        {
            if let Ok(gpio) = Gpio::new() {
                println!("[GPIO] RPPAL Raspberry Pi GPIO driver initialized successfully!");
                let output = match gpio.get(config.buzzer_pin as u8) {
                    Ok(pin) => {
                        let mut out = pin.into_output();
                        out.set_low();
                        Some(out)
                    }
                    Err(e) => {
                        eprintln!("[GPIO] Failed to claim buzzer pin {}: {}", config.buzzer_pin, e);
                        None
                    }
                };

                return Self {
                    buzzer_output: Arc::new(Mutex::new(output)),
                    buzzer_pin_num: config.buzzer_pin,
                    ring_pin_num: config.ring_pin,
                    open_pin_num: config.open_pin,
                    buzz_duration_ms: config.buzz_duration_ms,
                    buzzer_active: Arc::new(Mutex::new(false)),
                    rppal_active: true,
                };
            }
        }

        println!("[GPIO] RPPAL not available or running outside Pi. Mock mode active.");
        Self {
            #[cfg(feature = "door_device")]
            buzzer_output: Arc::new(Mutex::new(None)),
            buzzer_pin_num: config.buzzer_pin,
            ring_pin_num: config.ring_pin,
            open_pin_num: config.open_pin,
            buzz_duration_ms: config.buzz_duration_ms,
            buzzer_active: Arc::new(Mutex::new(false)),
            rppal_active: false,
        }
    }

    pub fn trigger_buzzer(&self) {
        let pin_num = self.buzzer_pin_num;
        let duration_ms = self.buzz_duration_ms;
        let active_flag = Arc::clone(&self.buzzer_active);
        #[cfg(feature = "door_device")]
        let buzzer_output = Arc::clone(&self.buzzer_output);

        thread::spawn(move || {
            {
                let mut active = active_flag.lock().unwrap();
                if *active {
                    println!("[GPIO] Buzzer already active, ignoring request.");
                    return;
                }
                *active = true;
            }

            println!(
                "[GPIO] BUZZER ACTIVATED on pin {} for {} ms",
                pin_num, duration_ms
            );

            #[cfg(feature = "door_device")]
            {
                if let Ok(mut lock) = buzzer_output.lock() {
                    if let Some(ref mut pin) = *lock {
                        pin.set_high();
                    }
                }
            }

            thread::sleep(Duration::from_millis(duration_ms));

            #[cfg(feature = "door_device")]
            {
                if let Ok(mut lock) = buzzer_output.lock() {
                    if let Some(ref mut pin) = *lock {
                        pin.set_low();
                    }
                }
            }

            println!("[GPIO] BUZZER DEACTIVATED on pin {}", pin_num);

            let mut active = active_flag.lock().unwrap();
            *active = false;
        });
    }

    pub fn start_input_listener<FRing, FOpen>(
        &self,
        on_ring: FRing,
        on_open: FOpen,
    ) where
        FRing: Fn() + Send + 'static,
        FOpen: Fn() + Send + 'static,
    {
        if !self.rppal_active {
            println!("[GPIO] Input listener running in mock mode.");
            return;
        }

        let ring_pin_num = self.ring_pin_num as u8;
        let open_pin_num = self.open_pin_num as u8;

        thread::spawn(move || {
            #[cfg(feature = "door_device")]
            {
                if let Ok(gpio) = Gpio::new() {
                    let ring_input = gpio.get(ring_pin_num).map(|p| p.into_input_pullup());
                    let open_input = gpio.get(open_pin_num).map(|p| p.into_input_pullup());

                    if let (Ok(ring), Ok(open)) = (ring_input, open_input) {

                        println!("[GPIO] RPPAL Input listener started with internal PULL-UP.");
                        println!(
                            "[GPIO] Ring pin {} & Open pin {} default to HIGH (1). Connect button/wire to GND to trigger!",
                            ring_pin_num, open_pin_num
                        );

                        let mut last_ring_level = ring.read();
                        let mut last_open_level = open.read();

                        loop {
                            thread::sleep(Duration::from_millis(50));

                            let current_ring = ring.read();
                            if current_ring != last_ring_level {
                                println!(
                                    "[GPIO] Ring pin {} level changed: {:?} -> {:?}",
                                    ring_pin_num, last_ring_level, current_ring
                                );
                                if current_ring == rppal::gpio::Level::Low {
                                    println!("[GPIO] Doorbell Ring button pressed!");
                                    on_ring();
                                }
                                last_ring_level = current_ring;
                            }

                            let current_open = open.read();
                            if current_open != last_open_level {
                                println!(
                                    "[GPIO] Open pin {} level changed: {:?} -> {:?}",
                                    open_pin_num, last_open_level, current_open
                                );
                                if current_open == rppal::gpio::Level::Low {
                                    println!("[GPIO] Local Door Open button pressed!");
                                    on_open();
                                }
                                last_open_level = current_open;
                            }
                        }
                    } else {
                        eprintln!("[GPIO] Failed to claim input pins {} or {}", ring_pin_num, open_pin_num);
                    }
                }
            }
        });
    }
}
