use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[derive(Serialize, Deserialize, Debug)]
pub struct MqttConfigCache {
    // Id to identify the device connecting to a MQTT broker
    pub id: String,

    // Ip address of the MQTT broker
    #[serde(rename = "brokerIp")]
    pub broker_ip: String,

    // Port of the MQTT broker
    #[serde(rename = "brokerPort")]
    pub broker_port: u32,
}

#[derive(Debug)]
pub struct MqttConfig {
    mqtt_config_cache: MqttConfigCache,

    mqtt_config_path: PathBuf,
}

/// Internal functions except for new for impl setup
impl MqttConfig {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mqtt_config_path = setup_path()?;

        let hostname = hostname::get()?.to_string_lossy().into_owned();

        let mut mqtt_config = Self {
            // Default device structure
            mqtt_config_cache: MqttConfigCache {
                id: String::new(),
                broker_ip: "127.0.0.1".to_string(),
                broker_port: 1883,
            },

            mqtt_config_path,
        };

        // Load existing configuration into runtime cache.
        mqtt_config.load()?;

        // Create default mqtt config if not exists.
        if !mqtt_config.mqtt_config_path.is_file() || mqtt_config.mqtt_config_cache.id != hostname {
            mqtt_config.mqtt_config_cache.id = hostname;
            mqtt_config.save_mqtt_config()?;
        }

        Ok(mqtt_config)
    }

    fn save_mqtt_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string_pretty(&self.mqtt_config_cache)?;

        save_to_disk(&json_string, &self.mqtt_config_path)
    }

    /// Loads mqtt config and stores it in runtime struct cache
    fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.mqtt_config_path.is_file() {
            let mqtt_config_json = read_from_disk(&self.mqtt_config_path)?;
            self.mqtt_config_cache = serde_json::from_str(&mqtt_config_json)?;
        }

        Ok(())
    }
}

fn setup_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let this_file_path = Path::new(file!()).canonicalize()?;

    let proj_basedir = this_file_path
        .ancestors()
        .nth(4)
        .ok_or("Could not determine base directory")?;

    let mqtt_config_path = proj_basedir.join("config").join("mqtt.json");

    Ok(mqtt_config_path)
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
