use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


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
    sysfs_available: bool,
    buzzer_pin: u32,
    ring_pin: u32,
    open_pin: u32,
    buzz_duration_ms: u64,
    buzzer_active: Arc<Mutex<bool>>,
}

impl GpioController {
    pub fn new(config: &DoorDeviceConfig) -> Self {
        let sysfs_available = Path::new("/sys/class/gpio").exists();
        if sysfs_available {
            println!("[GPIO] Linux Sysfs GPIO detected.");
        } else {
            println!("[GPIO] Sysfs GPIO not available. Running in Mock/Simulated mode.");
        }

        let controller = Self {
            sysfs_available,
            buzzer_pin: config.buzzer_pin,
            ring_pin: config.ring_pin,
            open_pin: config.open_pin,
            buzz_duration_ms: config.buzz_duration_ms,
            buzzer_active: Arc::new(Mutex::new(false)),
        };

        controller.init_pins();
        controller
    }

    fn init_pins(&self) {
        if !self.sysfs_available {
            return;
        }

        let _ = self.export_pin(self.buzzer_pin);
        let _ = self.set_direction(self.buzzer_pin, "out");
        let _ = self.write_pin(self.buzzer_pin, 0);

        let _ = self.export_pin(self.ring_pin);
        let _ = self.set_direction(self.ring_pin, "in");

        let _ = self.export_pin(self.open_pin);
        let _ = self.set_direction(self.open_pin, "in");
    }

    fn export_pin(&self, pin: u32) -> Result<(), std::io::Error> {
        let pin_path = format!("/sys/class/gpio/gpio{}", pin);
        if !Path::new(&pin_path).exists() {
            let mut export = fs::File::create("/sys/class/gpio/export")?;
            export.write_all(pin.to_string().as_bytes())?;
        }
        Ok(())
    }

    fn set_direction(&self, pin: u32, dir: &str) -> Result<(), std::io::Error> {
        let path = format!("/sys/class/gpio/gpio{}/direction", pin);
        let mut f = fs::File::create(path)?;
        f.write_all(dir.as_bytes())?;
        Ok(())
    }

    fn write_pin(&self, pin: u32, val: u8) -> Result<(), std::io::Error> {
        let path = format!("/sys/class/gpio/gpio{}/value", pin);
        let mut f = fs::File::create(path)?;
        f.write_all(val.to_string().as_bytes())?;
        Ok(())
    }

    #[allow(dead_code)]
    fn read_pin(&self, pin: u32) -> Result<u8, std::io::Error> {
        let path = format!("/sys/class/gpio/gpio{}/value", pin);
        let content = fs::read_to_string(path)?;
        let val = content.trim().parse::<u8>().unwrap_or(0);
        Ok(val)
    }

    /// Asynchronously trigger the door buzzer pin for the configured duration.
    pub fn trigger_buzzer(&self) {
        let pin = self.buzzer_pin;
        let duration_ms = self.buzz_duration_ms;
        let sysfs = self.sysfs_available;
        let active_flag = Arc::clone(&self.buzzer_active);

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
                pin, duration_ms
            );

            if sysfs {
                let path = format!("/sys/class/gpio/gpio{}/value", pin);
                let _ = fs::write(&path, "1");
            }

            thread::sleep(Duration::from_millis(duration_ms));

            if sysfs {
                let path = format!("/sys/class/gpio/gpio{}/value", pin);
                let _ = fs::write(&path, "0");
            }

            println!("[GPIO] BUZZER DEACTIVATED on pin {}", pin);

            let mut active = active_flag.lock().unwrap();
            *active = false;
        });
    }

    /// Start listening for button presses on `ring_pin` and `open_pin`.
    pub fn start_input_listener<FRing, FOpen>(
        &self,
        on_ring: FRing,
        on_open: FOpen,
    ) where
        FRing: Fn() + Send + 'static,
        FOpen: Fn() + Send + 'static,
    {

        if !self.sysfs_available {
            println!("[GPIO] Input listener running in mock mode (no physical pins polling).");
            return;
        }

        let ring_pin = self.ring_pin;
        let open_pin = self.open_pin;

        thread::spawn(move || {
            let ring_path = format!("/sys/class/gpio/gpio{}/value", ring_pin);
            let open_path = format!("/sys/class/gpio/gpio{}/value", open_pin);

            let mut last_ring_state = fs::read_to_string(&ring_path)
                .ok()
                .and_then(|s| s.trim().parse::<u8>().ok())
                .unwrap_or(0);

            let mut last_open_state = fs::read_to_string(&open_path)
                .ok()
                .and_then(|s| s.trim().parse::<u8>().ok())
                .unwrap_or(0);

            println!(
                "[GPIO] Input listener started. Initial states -> Ring(pin {}): {}, Open(pin {}): {}",
                ring_pin, last_ring_state, open_pin, last_open_state
            );

            loop {
                thread::sleep(Duration::from_millis(50));

                if let Ok(content) = fs::read_to_string(&ring_path) {
                    if let Ok(current_state) = content.trim().parse::<u8>() {
                        if current_state != last_ring_state {
                            println!(
                                "[GPIO] Ring pin {} state changed: {} -> {}",
                                ring_pin, last_ring_state, current_state
                            );
                            // Trigger action on any active button press (either 0->1 or 1->0)
                            on_ring();
                            last_ring_state = current_state;
                        }
                    }
                }

                if let Ok(content) = fs::read_to_string(&open_path) {
                    if let Ok(current_state) = content.trim().parse::<u8>() {
                        if current_state != last_open_state {
                            println!(
                                "[GPIO] Open pin {} state changed: {} -> {}",
                                open_pin, last_open_state, current_state
                            );
                            // Trigger action on any active button press (either 0->1 or 1->0)
                            on_open();
                            last_open_state = current_state;
                        }
                    }
                }
            }
        });

    }
}
