use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct DspCtrl {}

impl DspCtrl {
    const PATH_POWER: &str = "/sys/class/backlight/panel_backlight@1/bl_power";
    const PATH_BRIGHT: &str = "/sys/class/backlight/panel_backlight@1/brightness";

    pub fn new() -> Self {
        Self {}
    }

    pub fn set_display(&self, display_state: bool, display_brightness: u32) {
        if Path::new(Self::PATH_POWER).is_file() {
            let power = if display_state { "1" } else { "0" };

            if let Err(e) = fs::write(Self::PATH_POWER, power) {
                eprintln!("Failed to set display power: {}", e);
            }
        } else {
            println!("Power path not found: {}", Self::PATH_POWER)
        }

        if display_state && Path::new(Self::PATH_BRIGHT).is_file() && display_brightness < 101 {
            let target_val: i32 = (display_brightness * 32 / 100) as i32;

            if let Err(e) = fs::write(Self::PATH_BRIGHT, target_val.to_string()) {
                eprintln!("Failed to set display brightness: {}", e);
            }
        } else if display_state && !Path::new(Self::PATH_BRIGHT).is_file() {
            println!("Brightness path not found: {}", Self::PATH_BRIGHT);
        } else if display_state && display_brightness > 100 {
            println!(
                "Display brightness value was given but was over 100: {}",
                display_brightness
            );
        }
    }
}
