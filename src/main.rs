mod core;

use std::thread;
use std::time::Duration;

use crate::core::set::devices::DeviceManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_manager = DeviceManager::new()?;

    // Just for debuging now
    loop {
        thread::sleep(Duration::from_secs(5));
        let _ = config_manager.save_config();
        let _ = config_manager.save_internal_settings();
        println!("Data saved");
    }
}