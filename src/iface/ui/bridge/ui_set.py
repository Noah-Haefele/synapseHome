import os
import json
import logging
from PySide6.QtCore import QObject, Signal, Property, QTimer
from pathlib import Path

logger = logging.getLogger(__name__)

class SettingsManager(QObject):
    settingsChanged = Signal()

    def __init__(self):
        super().__init__()

        BASE_DIR = Path(__file__).resolve().parents[4]
        self._config_file = os.path.join(BASE_DIR, "data", "settings.json")

        # defaults
        self._data = {
            "floorIdx": 1
        }

        self._save_timer = QTimer()
        self._save_timer.setSingleShot(True)
        self._save_timer.setInterval(1000)
        self._save_timer.timeout.connect(self._do_save_to_disk)

        self._options = []
        self.load_config()

    def load_config(self):
        if os.path.exists(self._config_file):
            try:
                with open(self._config_file, "r") as f:
                    new_data = json.load(f)
                self._data.update({k: v for k, v in new_data.items() if k in self._data})
                self.settingsChanged.emit()
            except Exception as e:
                logger.error(f"Settings could not be loaded: {e}")
        else:
            # file doesnt exist, save defaults
            self._do_save_to_disk()     

    def _update (self, key, val):
        if self._data.get(key) != val:
            self._data[key] = val
            self.settingsChanged.emit()
            self._save_timer.start()

    def _do_save_to_disk(self):
        # atomic save: first write in tmp file, then replace original
        temp_file = self._config_file + ".tmp"
        try:
            with open(temp_file, "w") as f:
                json.dump(self._data, f, indent= 4)
                f.flush()
                os.fsync(f.fileno())

            os.replace(temp_file, self._config_file)
        except Exception as e:
            logger.error(f"Could not save settings: {e}")

    
    @Property(int, notify=settingsChanged)
    def floorIdx(self): return self._data.get("floorIdx", 0)
    @floorIdx.setter
    def floorIdx(self, val): self._update("floorIdx", int(val))