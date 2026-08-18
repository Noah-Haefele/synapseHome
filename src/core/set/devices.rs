use std::path::Path;
use std::path::PathBuf;
use std::io::Write;
use std::fs;
use tempfile::NamedTempFile;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Device {
    name: String,

    #[serde(rename = "shortName")]
    short_name: String,

    #[serde(rename = "deviceId")]
    device_id: i32,
}

// Data for the user to set in a json
#[derive(Serialize, Deserialize, Debug)]
struct ConfigCache {
    devices: Vec<Device>,
}

// Internal data set by the UI
#[derive(Serialize, Deserialize, Debug)]
struct InternalSettingsCache {
    location_id: i32,
    pref_call_id1: i32,
    pref_call_id2: i32,
    pref_call_id3: i32,
}

pub struct DeviceManager {
    config_cache: ConfigCache,
    internal_settings_cache: InternalSettingsCache,

    config_path: PathBuf,
    internal_settings_path: PathBuf,
}

impl DeviceManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (config_path, internal_settings_path) = setup_paths()?;
        let (config_cache, internal_settings_cache) = Self::setup_defaults();

        let mut manager = Self {
            // Default device structure
            config_cache,
            internal_settings_cache,

            config_path,
            internal_settings_path,
        };

        // Load existing configuration into runtime cache.
        manager.load()?;

        // Create default config if it does not exist.
        if !manager.config_path.is_file() {
            manager.save_config()?;
        }

        // Create default internal settings if they do not exist.
        if !manager.internal_settings_path.is_file() {
            manager.save_internal_settings()?;
        }

        Ok(manager)
    }

    fn setup_defaults() -> (ConfigCache, InternalSettingsCache) {
        (
            ConfigCache {
                devices: vec![
                    Device {
                        name: "1st Floor".to_string(),
                        short_name: "1".to_string(),
                        device_id: 1,
                    },

                    Device {
                        name: "2nd Floor".to_string(),
                        short_name: "2".to_string(),
                        device_id: 2,
                    },

                    Device {
                        name: "3rd Floor".to_string(),
                        short_name: "3".to_string(),
                        device_id: 3,
                    },
                ],
            },

            InternalSettingsCache {
                location_id: 1,
                pref_call_id1: 2,
                pref_call_id2: 3,
                pref_call_id3: -1,
            },
        )
    }

    pub fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string_pretty(&self.config_cache)?;

        save_to_disk(&json_string, &self.config_path)
    }

    pub fn save_internal_settings(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string_pretty(&self.internal_settings_cache)?;

        save_to_disk(&json_string, &self.internal_settings_path)
    }

    /// Loads both internal and config data and stores it in runtime struct cache
    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config_path.is_file() {
            let config_json = read_from_disk(&self.config_path)?;
            self.config_cache = serde_json::from_str(&config_json)?;
        }
        if self.internal_settings_path.is_file() {
            let internal_settings_json = read_from_disk(&self.internal_settings_path)?;
            self.internal_settings_cache = serde_json::from_str(&internal_settings_json)?;
        }

        Ok(())
    }
}

fn setup_paths() -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    let this_file_path = Path::new(file!()).canonicalize()?;

    let proj_basedir = this_file_path
        .ancestors()
        .nth(4)
        .ok_or("Could not determine base directory")?;

    let config_path = proj_basedir.join("config").join("devices.json");
    let internal_settings_path = proj_basedir.join("internal").join("settings.json");

    Ok((
        config_path,
        internal_settings_path,
    ))
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
