use std::collections::HashMap;
use std::process::Command;

pub struct AudioDevicesHandler {
    pub sinks: HashMap<String, String>, // ID -> Name (z. B. "58" -> "Raptor Lake-P/U/H cAVS Speaker")
    pub sources: HashMap<String, String>, // ID -> Name (z. B. "60" -> "Raptor Lake-P/U/H cAVS Digital Microphone")
}

impl AudioDevicesHandler {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut audio_devices_handler = Self {
            sinks: HashMap::new(),
            sources: HashMap::new(),
        };
        audio_devices_handler.fetch_audio_devices()?;

        Ok(audio_devices_handler)
    }

    fn fetch_audio_devices(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::new("wpctl").arg("status").output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);

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

                // Scip empty lines
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

                // Cut after the dot seperating id from rest
                if let Some(dot_idx) = sanitized.find('.') {
                    let potential_id = &sanitized[..dot_idx];

                    // Check if id only consists of numbers
                    if potential_id.chars().all(|c| c.is_ascii_digit()) && !potential_id.is_empty()
                    {
                        let id = potential_id.to_string();
                        let rest = sanitized[dot_idx + 1..].trim();

                        // Remove volume
                        let name = if let Some(bracket_idx) = rest.find('[') {
                            rest[..bracket_idx].trim().to_string()
                        } else {
                            rest.to_string()
                        };

                        // Attach to hashmap
                        if current_section == "audio" {
                            match current_subsection {
                                "sinks" => {
                                    self.sinks.insert(id, name);
                                }
                                "sources" => {
                                    self.sources.insert(id, name);
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

    pub fn get_sinks(&self) -> &HashMap<String, String> {
        &self.sinks
    }

    pub fn get_sources(&self) -> &HashMap<String, String> {
        &self.sources
    }

    pub fn set_default(&self, node_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let status = Command::new("wpctl")
            .arg("set-default")
            .arg(node_id)
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(format!("Failed to set default sink/source with id: {}", node_id).into())
        }
    }
}
