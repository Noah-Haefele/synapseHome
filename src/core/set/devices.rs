use std::path::Path;
use std::path::PathBuf;
use std::io::Write;
use std::fs;
use tempfile::NamedTempFile;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Device {
    #[serde(rename = "deviceName")]
    device_name: String,

    #[serde(rename = "deviceShortName")]
    device_short_name: String,

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
                        device_name: "1st Floor".to_string(),
                        device_short_name: "1".to_string(),
                        device_id: 1,
                    },

                    Device {
                        device_name: "2nd Floor".to_string(),
                        device_short_name: "2".to_string(),
                        device_id: 2,
                    },

                    Device {
                        device_name: "3rd Floor".to_string(),
                        device_short_name: "3".to_string(),
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
    fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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

#[cxx::bridge]
mod ffi {
    struct DeviceData {
        device_name: String,
        device_short_name: String,
        device_id: i32,
    }
    
    extern "Rust" {
        type DeviceManager;
        
        fn get_all_devices(self: &DeviceManager) -> Vec<DeviceData>;
        fn get_location_id(self: &DeviceManager) -> i32;
        fn get_device_name(self: &DeviceManager, device_id: i32) -> String;
        fn get_device_short_name(self: &DeviceManager, device_id: i32) -> String;
        fn get_pref_call_id(self: &DeviceManager, num: i32) -> i32;
        fn get_pref_model(self: &DeviceManager) -> Vec<DeviceData>;
        fn get_pref_icon_path(self: &DeviceManager, num: i32) -> String;
        fn set_location_id(self: &mut DeviceManager, device_id: i32);
        fn set_pref_call_id(self: &mut DeviceManager, num: i32, device_id: i32);
    }
}

impl DeviceManager {
    fn get_all_devices(&self) -> Vec<ffi::DeviceData> {
        self.config_cache
            .devices
            .iter()
            .map(|device| ffi::DeviceData {
                device_name: device.device_name.clone(),
                device_short_name: device.device_short_name.clone(),
                device_id: device.device_id,
            })
            .collect()
    }

    fn get_location_id(&self) -> i32 {
        self.internal_settings_cache.location_id
    }

    fn get_device_config(&self, device_id: i32,) -> Option<&Device> {
        self.config_cache
            .devices
            .iter()
            .find(|device| device.device_id == device_id)
    }

    fn get_device_name(&self, device_id: i32) -> String {
        self.get_device_config(device_id)
            .map(|device| device.device_name.clone())
            .unwrap_or_else(|| "Unknown...".to_string())
    }

    fn get_device_short_name(&self, device_id: i32) -> String {
        if device_id == -1 {
            return String::new();
        }

        self.get_device_config(device_id)
            .map(|device| device.device_short_name.clone())
            .unwrap_or_else(|| "?".to_string())
    }

    fn get_pref_call_id(&self, num: i32) -> i32 {
        match num {
            1 => self.internal_settings_cache.pref_call_id1,
            2 => self.internal_settings_cache.pref_call_id2,
            3 => self.internal_settings_cache.pref_call_id3,
            _ => -1,
        }
    }

    fn get_pref_model(&self) -> Vec<ffi::DeviceData> {
        let location_id = self.get_location_id();

        let mut model = vec![
            ffi::DeviceData {
                device_name: "Unassigned".to_string(),
                device_short_name: String::new(),
                device_id: -1,
            }
        ];

        model.extend(
            self.config_cache
                .devices
                .iter()
                .filter(|device| device.device_id != location_id)
                .map(|device| ffi::DeviceData {
                    device_name: device.device_name.clone(),
                    device_short_name: device.device_short_name.clone(),
                    device_id: device.device_id,
                })
        );

        model
    }

    fn get_pref_icon_path(&self, num: i32) -> String {
        let pref_call_id = self.get_pref_call_id(num);

        if pref_call_id == -1 {
            String::new()
        } else {
            "qrc:/qt/qml/UiBridge/assets/icons/button/call.svg".to_string()
        }
    }

    fn set_location_id(&mut self, device_id: i32) {
        if self.get_location_id() != device_id {
            self.internal_settings_cache.location_id = device_id;
            
            if self.internal_settings_cache.pref_call_id1 == device_id {
                self.internal_settings_cache.pref_call_id1 = -1;
            }

            if self.internal_settings_cache.pref_call_id2 == device_id {
                self.internal_settings_cache.pref_call_id2 = -1;
            }

            if self.internal_settings_cache.pref_call_id3 == device_id {
                self.internal_settings_cache.pref_call_id3 = -1;
            }
        }
    }

    fn set_pref_call_id(&mut self, num: i32, device_id: i32) {
        if self.get_pref_call_id(num) == device_id {
            return
        }

        if device_id != -1 {
            for i in [1, 2, 3] {
                if i != num && self.get_pref_call_id(i) == device_id {
                    match i {
                        1 => self.internal_settings_cache.pref_call_id1 = -1,
                        2 => self.internal_settings_cache.pref_call_id2 = -1,
                        3 => self.internal_settings_cache.pref_call_id3 = -1,
                        _ => {},
                    }
                }
            }
        }

        match num {
            1 => self.internal_settings_cache.pref_call_id1 = device_id,
            2 => self.internal_settings_cache.pref_call_id2 = device_id,
            3 => self.internal_settings_cache.pref_call_id3 = device_id,
            _ => {},
        }
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
