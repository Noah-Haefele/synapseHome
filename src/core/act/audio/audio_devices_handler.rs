use std::collections::HashMap;
use std::process::Command;

/// Handles audio devices and stores information about them (e.g default sink/source)
pub struct AudioDevicesHandler {
    sinks: HashMap<i32, String>,    // ID Index -> Name
    sources: HashMap<i32, String>,  // ID Index -> Name
    default_sink_id: Option<i32>,   // Sink currently the default
    default_source_id: Option<i32>, // Source currently the default
}

impl AudioDevicesHandler {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut audio_devices_handler = Self {
            sinks: HashMap::new(),
            sources: HashMap::new(),
            default_sink_id: None,
            default_source_id: None,
        };
        audio_devices_handler.fetch_audio_devices()?;

        Ok(audio_devices_handler)
    }

    /// Fetches all available audio sinks and sources, stores them in their
    /// respective hashmaps, and determines the default sink and source IDs.
    fn fetch_audio_devices(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::new("wpctl").arg("status").output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Remove all data from the previous function call.
        self.sinks.clear();
        self.sources.clear();
        self.default_sink_id = None;
        self.default_source_id = None;

        let mut current_section = "";
        let mut current_subsection = "";

        for line in stdout.lines() {
            // Detect main sections
            if line.contains("Audio") {
                current_section = "audio";
            } else if line.contains("Video") || line.contains("Settings") {
                current_section = "";
            }

            // Detect subsections
            if line.contains("Sinks:") {
                current_subsection = "sinks";
                continue;
            } else if line.contains("Sources:") {
                current_subsection = "sources";
                continue;
            } else if line.contains("Filters:")
                || line.contains("Video:")
                || line.contains("Settings:")
            {
                current_subsection = "";
                continue;
            }

            if !current_subsection.is_empty() {
                let trimmed = line.trim();

                // Skip empty lines
                if trimmed.is_empty()
                    || trimmed.starts_with('├')
                    || trimmed.starts_with('└')
                    || trimmed.starts_with('─')
                {
                    continue;
                }

                // Remove "|" and "*"
                let sanitized = trimmed.replace('│', "").replace('*', "");
                let sanitized = sanitized.trim();

                // Split at the dot separating the ID from the rest.
                if let Some(dot_idx) = sanitized.find('.') {
                    let potential_id = &sanitized[..dot_idx];

                    // Check if id only consists of numbers
                    if potential_id.chars().all(|c| c.is_ascii_digit()) && !potential_id.is_empty()
                    {
                        let id = potential_id.parse::<i32>()?;
                        let rest = sanitized[dot_idx + 1..].trim();

                        // Remove volume
                        let name = if let Some(bracket_idx) = rest.find('[') {
                            rest[..bracket_idx].trim().to_string()
                        } else {
                            rest.to_string()
                        };

                        // Attach to hashmap
                        // Note: wpctl highlights the default sink/source by a "*" infront of the ID Index
                        if current_section == "audio" {
                            match current_subsection {
                                "sinks" => {
                                    self.sinks.insert(id, name);
                                    if line.contains("*") {
                                        self.default_sink_id = Some(id);
                                    }
                                }
                                "sources" => {
                                    self.sources.insert(id, name);
                                    if line.contains("*") {
                                        self.default_source_id = Some(id);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Retrieves all available audio sinks
    pub fn get_sinks(&mut self) -> Result<&HashMap<i32, String>, Box<dyn std::error::Error>> {
        self.fetch_audio_devices()?;

        Ok(&self.sinks)
    }

    /// Retrieves all available audio sources
    pub fn get_sources(&mut self) -> Result<&HashMap<i32, String>, Box<dyn std::error::Error>> {
        self.fetch_audio_devices()?;

        Ok(&self.sources)
    }

    /// Retrieves default sink ID index
    pub fn get_default_sink_id(&mut self) -> Result<Option<i32>, Box<dyn std::error::Error>> {
        self.fetch_audio_devices()?;

        Ok(self.default_sink_id)
    }

    /// Retrieves default source ID index
    pub fn get_default_source_id(&mut self) -> Result<Option<i32>, Box<dyn std::error::Error>> {
        self.fetch_audio_devices()?;

        Ok(self.default_source_id)
    }

    /// Sets default sink/source based on ID index
    pub fn set_default(&self, node_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let status = Command::new("wpctl")
            .arg("set-default")
            .arg(node_id.to_string())
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(format!("Failed to set default sink/source with id: {}", node_id).into())
        }
    }
}
