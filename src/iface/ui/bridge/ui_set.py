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
        self._config_file = os.path.join(BASE_DIR, "config", "floors.json")

        # defaults
        # settings
        self._settings = {
            "floorIdx": 0,          # In which floor is the device
            "fastCallIdx1": 1,      # Floor call shortcut 1 in control grid
            "fastCallIdx2": 2,      # Floor call shortcut 2 in control grid
            "fastCallIdx3": 3       # Floor call shortcut 3 in control grid
        }

        # config
        self._config = [
            {
                "name": "1st Floor",
                "floorId": 1,
                "iconPath": "../../../../assets/icons/button/callE.svg",
                "type": "floor"
            },
            {
                "name": "2nd Floor",
                "floorId": 2,
                "iconPath": "../../../../assets/icons/button/call1.svg",
                "type": "floor"
            },
            {
                "name": "3rd Floor",
                "floorId": 3,
                "iconPath": "../../../../assets/icons/button/callD.svg",
                "type": "floor"
            }
        ]


        self._save_timer = QTimer()
        self._save_timer.setSingleShot(True)
        self._save_timer.setInterval(1000)
        self._save_timer.timeout.connect(self._save_settings)

        self._options = []
        self.load_settings()
        self.load_config()

    # loads settings which user can change
    def load_settings(self):
        if os.path.exists(self._settings_file):
            try:
                with open(self._settings_file, "r") as f:
                    new_settings = json.load(f)
                self._settings.update({k: v for k, v in new_settings.items() if k in self._settings})
                self.settingsChanged.emit()
            except Exception as e:
                logger.error(f"Settings could not be loaded: {e}")
        else:
            # file doesnt exist, save defaults
            self._save_settings() 

    # load manditory config which are relevant for overall application behavior
    def load_config(self):
        if os.path.exists(self._config_file):
            try:
                with open(self._config_file, "r") as f:
                    self._config = json.load(f)

                self.settingsChanged.emit()

            except Exception as e:
                logger.error(f"Configs could not be loaded: {e}")
        else:
            self._save_config()

    def _update (self,data, key, val):
        if data.get(key) != val:
            data[key] = val
            self.settingsChanged.emit()
            self._save_timer.start()

    def _save_settings(self):
        self._do_save_to_disk(self._settings, self._settings_file)

    def _save_config(self):
        self._do_save_to_disk(self._config, self._config_file)

    def _do_save_to_disk(self, data, data_file):
        # atomic save: first write in tmp file, then replace original
        temp_file = data_file + ".tmp"
        try:
            with open(temp_file, "w") as f:
                json.dump(data, f, indent= 4)
                f.flush()
                os.fsync(f.fileno())

            os.replace(temp_file, data_file)
        except Exception as e:
            logger.error(f"Could not save settings: {e}")

    # In which floor this device is
    @Property(int, notify=settingsChanged)
    def floorIdx(self): return self._settings.get("floorIdx", 0)
    @floorIdx.setter
    def floorIdx(self, val): self._update(self._settings ,"floorIdx", int(val))
    # Floor-Preference 1
    @Property(int, notify=settingsChanged)
    def fastCallIdx1(self): return self._settings.get("fastCallIdx1", 0)
    @fastCallIdx1.setter
    def fastCallIdx1(self, val): self._update(self._settings ,"fastCallIdx1", int(val))
    # Floor-Preference 2
    @Property(int, notify=settingsChanged)
    def fastCallIdx2(self): return self._settings.get("fastCallIdx2", 0)
    @fastCallIdx2.setter
    def fastCallIdx2(self, val): self._update(self._settings ,"fastCallIdx2", int(val))
    # Floor-Preference 3
    @Property(int, notify=settingsChanged)
    def fastCallIdx3(self): return self._settings.get("fastCallIdx3", 0)
    @fastCallIdx3.setter
    def fastCallIdx3(self, val): self._update(self._settings ,"fastCallIdx3", int(val))