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
        self._settings_file = os.path.join(BASE_DIR, "data", "settings.json")

        # defaults
        self._data = {
            "floorIdx": 0,          # In which floor is the device
            "fastCallIdx1": 1,      # Floor call shortcut 1 in control grid
            "fastCallIdx2": 2,      # Floor call shortcut 2 in control grid
            "fastCallIdx3": 3       # Floor call shortcut 3 in control grid
        }

        self._save_timer = QTimer()
        self._save_timer.setSingleShot(True)
        self._save_timer.setInterval(1000)
        self._save_timer.timeout.connect(self._do_save_to_disk)

        self._options = []
        self.load_settings()

    def load_settings(self):
        if os.path.exists(self._settings_file):
            try:
                with open(self._settings_file, "r") as f:
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
        temp_file = self._settings_file + ".tmp"
        try:
            with open(temp_file, "w") as f:
                json.dump(self._data, f, indent= 4)
                f.flush()
                os.fsync(f.fileno())

            os.replace(temp_file, self._settings_file)
        except Exception as e:
            logger.error(f"Could not save settings: {e}")

    # In which floor this device is
    @Property(int, notify=settingsChanged)
    def floorIdx(self): return self._data.get("floorIdx", 0)
    @floorIdx.setter
    def floorIdx(self, val): self._update("floorIdx", int(val))
    # Floor-Preference 1
    @Property(int, notify=settingsChanged)
    def fastCallIdx1(self): return self._data.get("fastCallIdx1", 0)
    @fastCallIdx1.setter
    def fastCallIdx1(self, val): self._update("fastCallIdx1", int(val))
    # Floor-Preference 2
    @Property(int, notify=settingsChanged)
    def fastCallIdx2(self): return self._data.get("fastCallIdx2", 0)
    @fastCallIdx2.setter
    def fastCallIdx2(self, val): self._update("fastCallIdx2", int(val))
    # Floor-Preference 3
    @Property(int, notify=settingsChanged)
    def fastCallIdx3(self): return self._data.get("fastCallIdx3", 0)
    @fastCallIdx3.setter
    def fastCallIdx3(self, val): self._update("fastCallIdx3", int(val))