use std::path::Path;
use std::path::PathBuf;
use std::io::Write;
use std::fs;
use tempfile::NamedTempFile;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::sync::Mutex;

use crate::platform::linux::display_controller::DspCtrl;

#[derive(Serialize, Deserialize, Debug)]
pub struct InternalDisplaySettingsCache {
    // Brightness of display in percent from 0 -> 100
    pub brightness: u32,

    // Time between no touch-activity and display turing off
    #[serde(rename = "displayTime")]
    pub display_time: u32,
}

#[derive(Debug)]
pub struct DisplayManager {
    internal_display_settings_cache: InternalDisplaySettingsCache,

    internal_display_settings_path: PathBuf,

    display_controller: Arc<Mutex<DspCtrl>>,
}

/// Internal functions except for new for impl setup
impl DisplayManager {
    pub fn new(
        display_controller: Arc<Mutex<DspCtrl>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let internal_display_settings_path = setup_path()?;

        let mut manager = Self {
            display_controller,

            // Default device structure
            internal_display_settings_cache: InternalDisplaySettingsCache {
                brightness: 100,
                display_time: 60,
            },

            internal_display_settings_path,
        };

        // Load existing configuration into runtime cache.
        manager.load()?;

        // Create default internal settings if they do not exist.
        if !manager.internal_display_settings_path.is_file() {
            manager.save_internal_display_settings()?;
        }

        Ok(manager)
    }

    fn save_internal_display_settings(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string_pretty(&self.internal_display_settings_cache)?;

        save_to_disk(&json_string, &self.internal_display_settings_path)
    }

    /// Loads both internal and config data and stores it in runtime struct cache
    fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.internal_display_settings_path.is_file() {
            let internal_display_settings_json = read_from_disk(&self.internal_display_settings_path)?;
            self.internal_display_settings_cache = serde_json::from_str(&internal_display_settings_json)?;
        }

        Ok(())
    }
}

/// External functions called e.g by the grpc_server
impl DisplayManager {
    pub fn get_brightness(&self) -> u32 {
        self.internal_display_settings_cache.brightness
    }

    pub fn get_display_time(&self) -> u32 {
        self.internal_display_settings_cache.display_time
    }

    pub fn set_brightness(&mut self, val: u32) -> Result<(), Box<dyn std::error::Error>> {
        if val > 100 {
            return Err("brightness must be between 0 and 100".into());
        }
        let display_c = self
            .display_controller
            .lock()
            .map_err(|_| "Lock failed")?;

        display_c.set_display(true, val);

        self.internal_display_settings_cache.brightness = val;
        self.save_internal_display_settings()
    }

    pub fn set_display_time(&mut self, val: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.internal_display_settings_cache.display_time = val;
        self.save_internal_display_settings()
    }
}

fn setup_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let this_file_path = Path::new(file!()).canonicalize()?;

    let proj_basedir = this_file_path
        .ancestors()
        .nth(4)
        .ok_or("Could not determine base directory")?;

    let internal_display_settings_path = proj_basedir.join("internal").join("display.json");

    Ok(internal_display_settings_path)
}

fn save_to_disk(data: &str, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let parent = file_path.parent().unwrap_or_else(|| Path::new("."));

    fs::create_dir_all(parent)?;

    let mut tmp = NamedTempFile::new_in(parent)?;
    tmp.write_all(data.as_bytes())?;
    tmp.flush()?;
    tmp.persist(file_path)?;

    Ok(())
}

fn read_from_disk(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    Ok(fs::read_to_string(file_path)?)
}
