use std::path::Path;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use std::io::Write;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Device {
    name: Box<str>,

    #[serde(rename = "shortName")]
    short_name: Box<str>,

    #[serde(rename = "deviceId")]
    device_id: i32,
}

// Data for the user to set in a json
struct ConfigCache {
    devices: Vec<Device>,
}

pub struct DeviceManager {
    config_cache: ConfigCache,

    config_path: PathBuf,
}

impl DeviceManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = setup_paths()?;

        Ok(Self {
            // Create default floor structure
            config_cache: ConfigCache {
                devices: vec![
                    Device {
                        name: "1st Floor".to_string().into(),
                        short_name: "1".to_string().into(),
                        device_id: 1,
                    },

                    Device {
                        name: "2nd Floor".to_string().into(),
                        short_name: "2".to_string().into(),
                        device_id: 2,
                    },

                    Device {
                        name: "3rd Floor".to_string().into(),
                        short_name: "3".to_string().into(),
                        device_id: 3,
                    },
                ],
            },

            config_path,
        })
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string_pretty(&self.config_cache.devices)?;

        save_to_disk(&json_string, &self.config_path)?;

        Ok(())
    }
}

fn setup_paths() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let this_file_path = Path::new(file!()).canonicalize()?;

    let proj_basedir = this_file_path
        .ancestors()
        .nth(4)
        .ok_or("Could not determine base directory")?;

    let config_path = proj_basedir.join("config").join("devices.json");

    Ok(config_path)
}

fn save_to_disk(data: &str, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let parent = file_path.parent().unwrap_or_else(|| Path::new("."));

    let mut tmp = NamedTempFile::new_in(parent)?;
    tmp.write_all(data.as_bytes())?;
    tmp.flush()?;
    tmp.persist(file_path)?;

    Ok(())
}
