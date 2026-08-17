mod core;

use std::thread;
use std::time::Duration;

use crate::core::set::devices::ConfigManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_manager = ConfigManager::new()?;

    // Just for debuging now
    loop {
        let _ = config_manager.save();
        thread::sleep(Duration::from_secs(10));
    }
}