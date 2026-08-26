use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    #[serde(rename = "deviceName")]
    pub device_name: String,

    #[serde(rename = "deviceShortName")]
    pub device_short_name: String,

    #[serde(rename = "deviceId")]
    pub device_id: i32,
}

// Data for the user to set in a json
#[derive(Serialize, Deserialize, Debug, Default)]
struct ConfigCache {
    devices: Vec<Device>,
}

// Internal data set by the UI
#[derive(Serialize, Deserialize, Debug, Default)]
struct InternalSettingsCache {
    location_id: i32,
    pref1_call_id: i32,
    pref2_call_id: i32,
    pref3_call_id: i32,
}

#[derive(Debug, Default)]
pub struct DeviceManager {
    config_cache: ConfigCache,
    internal_settings_cache: InternalSettingsCache,

    config_path: PathBuf,
    internal_settings_path: PathBuf,
}

/// Internal functions except for new for impl setup
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
                pref1_call_id: 2,
                pref2_call_id: 3,
                pref3_call_id: -1,
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

/// External functions called e.g by the grpc_server
impl DeviceManager {
    pub fn get_all_devices(&self) -> Vec<Device> {
        self.config_cache
            .devices
            .iter()
            .map(|device| Device {
                device_name: device.device_name.clone(),
                device_short_name: device.device_short_name.clone(),
                device_id: device.device_id,
            })
            .collect()
    }

    pub fn get_location_id(&self) -> i32 {
        self.internal_settings_cache.location_id
    }

    fn get_device_config(&self, device_id: i32) -> Option<&Device> {
        self.config_cache
            .devices
            .iter()
            .find(|device| device.device_id == device_id)
    }

    // Used to display device_name when incoming or outgoing call
    pub fn get_device_name(&self, device_id: i32) -> String {
        self.get_device_config(device_id)
            .map(|device| device.device_name.clone())
            .unwrap_or_else(|| "Unknown...".to_string())
    }

    // Used for little label on each call icon
    pub fn get_device_short_name(&self, num: i32) -> String {
        let pref_call_id = self.get_pref_call_id(num);

        if pref_call_id == -1 {
            return String::new();
        }

        self.get_device_config(pref_call_id)
            .map(|device| device.device_short_name.clone())
            .unwrap_or_else(|| "?".to_string())
    }

    pub fn get_pref_call_id(&self, num: i32) -> i32 {
        match num {
            1 => self.internal_settings_cache.pref1_call_id,
            2 => self.internal_settings_cache.pref2_call_id,
            3 => self.internal_settings_cache.pref3_call_id,
            _ => -1,
        }
    }

    pub fn get_pref_model(&self) -> Vec<Device> {
        let location_id = self.get_location_id();

        let mut model = vec![Device {
            device_name: "Unassigned".to_string(),
            device_short_name: String::new(),
            device_id: -1,
        }];

        model.extend(
            self.config_cache
                .devices
                .iter()
                .filter(|device| device.device_id != location_id)
                .map(|device| Device {
                    device_name: device.device_name.clone(),
                    device_short_name: device.device_short_name.clone(),
                    device_id: device.device_id,
                }),
        );

        model
    }

    pub fn get_pref_icon_path(&self, num: i32) -> String {
        let pref_call_id = self.get_pref_call_id(num);

        if pref_call_id == -1 {
            String::new()
        } else {
            "qrc:/qt/qml/UiBridge/assets/icons/button/call.svg".to_string()
        }
    }

    pub fn set_location_id(&mut self, device_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        if self.get_location_id() != device_id {
            self.internal_settings_cache.location_id = device_id;

            if self.internal_settings_cache.pref1_call_id == device_id {
                self.internal_settings_cache.pref1_call_id = -1;
            }

            if self.internal_settings_cache.pref2_call_id == device_id {
                self.internal_settings_cache.pref2_call_id = -1;
            }

            if self.internal_settings_cache.pref3_call_id == device_id {
                self.internal_settings_cache.pref3_call_id = -1;
            }
        }

        self.save_internal_settings()
    }

    pub fn set_pref_call_id(
        &mut self,
        num: i32,
        device_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.get_pref_call_id(num) == device_id {
            return Ok(());
        }

        if device_id != -1 {
            for i in [1, 2, 3] {
                if i != num && self.get_pref_call_id(i) == device_id {
                    match i {
                        1 => self.internal_settings_cache.pref1_call_id = -1,
                        2 => self.internal_settings_cache.pref2_call_id = -1,
                        3 => self.internal_settings_cache.pref3_call_id = -1,
                        _ => {}
                    }
                }
            }
        }

        match num {
            1 => self.internal_settings_cache.pref1_call_id = device_id,
            2 => self.internal_settings_cache.pref2_call_id = device_id,
            3 => self.internal_settings_cache.pref3_call_id = device_id,
            _ => {}
        }

        self.save_internal_settings()
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

    Ok((config_path, internal_settings_path))
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
