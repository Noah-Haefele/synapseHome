import json
import logging
import socket
from dataclasses import asdict, dataclass, field
from pathlib import Path

# Logger initialisieren
logger = logging.getLogger(__name__)


@dataclass
class MQTTConfig:
    host: str = "127.0.0.1"
    port: int = 1883
    username: str = ""
    password: str = ""
    # on every new instance
    client_id: str = field(default_factory=lambda: f"synapsed_{socket.gethostname()}")
    topic_prefix: str = "synapsed"

    @classmethod
    def load_from_file(cls, filepath: str | Path) -> "MQTTConfig":
        """
        Loads the MQTT configuration from a JSON file.
        If the file does not exist or is invalid, a default configuration 
        is created and saved to disk.
        """
        path = Path(filepath)

        try:
            with path.open("r") as f:
                data = json.load(f)
                return cls(**data.get("mqtt", {}))
        except (FileNotFoundError, json.JSONDecodeError) as e:
            logger.warning(
                "Configuration file '%s' not found or invalid (%s). Generating default configuration.",
                path,
                e,
            )
            
            default_config = cls()
            default_config.save_to_file(path)
            return default_config

    def save_to_file(self, filepath: str | Path) -> None:
        """
        Saves the current MQTT configuration to a JSON file.
        Creates parent directories automatically if they do not exist.
        """
        path = Path(filepath)

        try:
            path.parent.mkdir(parents=True, exist_ok=True)

            with path.open("w") as f:
                json.dump({"mqtt": asdict(self)}, f, indent=4)
                
            logger.info("MQTT configuration successfully saved to '%s'.", path)
        except OSError as e:
            logger.error("Failed to save MQTT configuration to '%s': %s", path, e)