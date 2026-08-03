import os
import json
import logging
from pathlib import Path
from typing import Callable, Optional, Any
from PySide6.QtCore import QObject, QTimer, Signal

logger = logging.getLogger(__name__)

class SettingsManager(QObject):
    settingsChanged = Signal()
    
    def __init__(self):
        super().__init__()
        
        BASE_DIR = Path(__file__).resolve().parents[3]
        self._settings_file = BASE_DIR / "internal" / "general.json"

        # defaults
        self._data = {
            "brightness": 50,
            "displayTime": 30
        }
        
        self._save_timer = QTimer()
        self._save_timer.setSingleShot(True)
        self._save_timer.setInterval(1000)
        self._save_timer.timeout.connect(self._do_save_to_disk)
        
        self._load_settings()

    def get_setting(self, key, default: Any=None) -> Any: 
        return self._data.get(key, default)

    def get_brightness(self) -> int:
        return self.get_setting("brightness", 50)

    def get_display_time(self) -> int:
        return self.get_setting("displayTime", 30)

    def set_setting(self, key: str, val: Any) -> None:
        if self._data.get(key) != val:
            self._data[key] = val
            
            self.settingsChanged.emit()

            self._save_timer.start()

    def _do_save_to_disk(self) -> None:
        """Atomic write using a temporary file to prevent corruption."""
        try:
            self._settings_file.parent.mkdir(parents=True, exist_ok=True)
            temp_file = self._settings_file.with_suffix(".tmp")

            with open(temp_file, "w", encoding="utf-8") as f:
                json.dump(self._data, f, indent=4)
                f.flush()
                os.fsync(f.fileno())

            os.replace(temp_file, self._settings_file)
            logger.info("Settings successfully saved to disk: %s", self._settings_file)

        except (OSError, PermissionError) as e:
            logger.error("I/O error while saving settings: %s", e)
        except Exception as e:
            logger.error("Unexpected error while saving settings: %s", e, exc_info=True)

    def _load_settings(self) -> None:
        if not self._settings_file.exists():
            logger.info("No settings file found. Creating default settings at %s", self._settings_file)
            self._do_save_to_disk()
            return

        try:
            with open(self._settings_file, "r", encoding="utf-8") as f:
                new_data = json.load(f)
                if isinstance(new_data, dict):
                    self._data.update(new_data)
                    logger.info("Settings successfully loaded.")
                else:
                    logger.warning("Settings file did not contain a valid JSON object. Using defaults.")
        except json.JSONDecodeError as e:
            logger.error("Corrupted settings file / invalid JSON: %s. Reverting to defaults.", e)
        except (OSError, PermissionError) as e:
            logger.error("I/O error while loading settings: %s", e)
        except Exception as e:
            logger.error("Unexpected error while loading settings: %s", e, exc_info=True)