use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AudioConfig {
    pub ringtone_path: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RingtoneItem {
    pub ringtone_id: i32,
    pub ringtone_name: String,
    pub ringtone_path: PathBuf,
}

#[derive(Debug)]
pub struct RingtoneManager {
    internal_audio_config: AudioConfig,
    internal_audio_config_path: PathBuf,

    ringtones: Vec<RingtoneItem>,
    ringtone_dir: PathBuf,
}

impl RingtoneManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (internal_audio_config_path, ringtone_dir) = setup_path();

        let mut manager = Self {
            internal_audio_config: AudioConfig::default(),
            internal_audio_config_path,

            ringtones: Vec::new(),
            ringtone_dir,
        };

        // Load config from json
        manager.load()?;

        // Scan ringtone directory and validate configured path
        manager.fetch_ringtone_files()?;
        if manager.validate_internal_audio_config() {
            manager.save_internal_audio_config()?;
        }

        if !manager.internal_audio_config_path.is_file() {
            manager.save_internal_audio_config()?;
        }

        Ok(manager)
    }

    fn save_internal_audio_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string_pretty(&self.internal_audio_config)?;
        save_to_disk(&json_string, &self.internal_audio_config_path)
    }

    fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.internal_audio_config_path.is_file() {
            let json_data = read_from_disk(&self.internal_audio_config_path)?;
            self.internal_audio_config = serde_json::from_str(&json_data).unwrap_or_default();
        }

        Ok(())
    }

    pub fn fetch_ringtone_files(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.ringtones.clear();

        if self.ringtone_dir.is_dir() {
            let mut entries: Vec<_> = fs::read_dir(&self.ringtone_dir)?
                .filter_map(|e| e.ok())
                .collect();
            entries.sort_by_key(|e| e.path());

            for entry in entries {
                let path = entry.path();
                if path
                    .extension()
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("mp3"))
                {
                    if let Some(file_stem) = path.file_stem() {
                        let raw_name = file_stem.to_string_lossy();
                        let mut chars = raw_name.chars();
                        let formatted_name = match chars.next() {
                            None => String::new(),
                            Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                        };

                        let ringtone_id = calculate_stable_id(&path);
                        self.ringtones.push(RingtoneItem {
                            ringtone_id,
                            ringtone_name: formatted_name,
                            ringtone_path: path,
                        });
                    }
                }
            }
        } else {
            return Err(format!("Directory {:?} not found!", self.ringtone_dir).into());
        }

        Ok(())
    }

    fn validate_internal_audio_config(&mut self) -> bool {
        if let Some(path) = &self.internal_audio_config.ringtone_path {
            if !self.ringtones.iter().any(|r| &r.ringtone_path == path) {
                self.internal_audio_config.ringtone_path = None;
                return true;
            }
        }
        false
    }
}

/// External functions called e.g. by gRPC server
impl RingtoneManager {
    pub fn get_ringtone_model(&self) -> Vec<RingtoneItem> {
        let mut model = vec![RingtoneItem {
            ringtone_id: -1,
            ringtone_name: "Unassigned".to_string(),
            ringtone_path: PathBuf::new(),
        }];

        model.extend(self.ringtones.iter().cloned());
        model
    }

    pub fn get_ringtone_id(&self) -> i32 {
        match &self.internal_audio_config.ringtone_path {
            Some(path) => self
                .ringtones
                .iter()
                .find(|r| &r.ringtone_path == path)
                .map(|r| r.ringtone_id)
                .unwrap_or(-1),
            None => -1,
        }
    }

    pub fn get_ringtone_path(&self) -> Option<&Path> {
        self.internal_audio_config.ringtone_path.as_deref()
    }

    pub fn set_ringtone_id(&mut self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.fetch_ringtone_files()?;

        if id == -1 {
            self.internal_audio_config.ringtone_path = None;
        } else {
            let item = self
                .ringtones
                .iter()
                .find(|r| r.ringtone_id == id)
                .ok_or("Ringtone ID does not exist")?;
            self.internal_audio_config.ringtone_path = Some(item.ringtone_path.clone());
        }

        self.validate_internal_audio_config();
        self.save_internal_audio_config()?;

        Ok(())
    }

    pub fn refresh_ringtones(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.fetch_ringtone_files()?;

        if self.validate_internal_audio_config() {
            self.save_internal_audio_config()?;
        }

        Ok(())
    }
}

fn setup_path() -> (PathBuf, PathBuf) {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let internal_audio_config_path = base_dir.join("internal").join("ringtone.json");
    let ringtone_dir = base_dir.join("assets").join("audio").join("ringtones");
    (internal_audio_config_path, ringtone_dir)
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

fn calculate_stable_id(path: &Path) -> i32 {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    // map to i32 positive
    (hasher.finish() & 0x7FFFFFFF) as i32
}
