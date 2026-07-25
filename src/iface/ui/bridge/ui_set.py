import os
import json
import logging
from PySide6.QtCore import QObject, Signal, Property, QTimer, Slot
from pathlib import Path

logger = logging.getLogger(__name__)

class SettingsManager(QObject):
    """
    Manages persistent application settings and floor configurations
    for QML UI components.
    """

    settingsChanged = Signal()

    def __init__(self):
        super().__init__()

        BASE_DIR = Path(__file__).resolve().parents[4]
        self._settings_file = BASE_DIR / "data" / "settings.json"
        self._config_file = BASE_DIR / "config" / "floors.json"

        # Default user settings
        self._settings = {
            "floorIdx": 1,          # In which floor is the device
            "fastCallIdx1": 2,      # Floor call shortcut 1 in control grid
            "fastCallIdx2": 3,      # Floor call shortcut 2 in control grid
            "fastCallIdx3": -1       # Floor call shortcut 3 in control grid
        }

        # Default floor configuration
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

        # Debounce timer for non-blocking disk persistence (1s delay)
        self._save_timer = QTimer()
        self._save_timer.setSingleShot(True)
        self._save_timer.setInterval(1000)
        self._save_timer.timeout.connect(self._save_settings)

        self.load_settings()
        self.load_config()

    def load_settings(self):
        """Loads user settings from disk or creates default settings if missing."""
        if Path.exists(self._settings_file):
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

    def load_config(self):
        """Loads floor configuration from disk or creates default config if missing."""
        if Path.exists(self._config_file):
            try:
                with open(self._config_file, "r") as f:
                    self._config = json.load(f)

                self.settingsChanged.emit()

            except Exception as e:
                logger.error(f"Configs could not be loaded: {e}")
        else:
            # file doesnt exist, save defaults
            self._save_config()

    def _save_settings(self):
        self._do_save_to_disk(self._settings, self._settings_file)

    def _save_config(self):
        self._do_save_to_disk(self._config, self._config_file)

    def _do_save_to_disk(self, data, data_file):
        """Atomically saves JSON data using a temporary file to prevent corruption on interrupt."""
        temp_file = data_file.with_suffix(".tmp")
        try:
            with open(temp_file, "w") as f:
                json.dump(data, f, indent= 4)
                f.flush()
                os.fsync(f.fileno())

            os.replace(temp_file, data_file)
        except Exception as e:
            logger.error(f"Could not save settings: {e}")

    # --- QML Read-Only Properties ---
    
    @Property("QVariantList", notify=settingsChanged)
    def allFloors(self):
        """Returns all available floor definitions."""
        return self._config

    def _get_pref_model(self, active_pref_key):
        """
        Generates dynamic dropdown option lists for quick-call preferences.
        Excludes the active main floor location to prevent redundant shortcut choices.
        """
        model = [{"name": "Unassigned", "floorId": -1, "iconPath": "", "type": "empty"}]
        
        main_floor = self._settings.get("floorIdx", -1)

        for floor in self._config:
            # exclude floor in which the device is
            if floor["floorId"] == main_floor:
                continue
            model.append(floor)

        return model

    @Property("QVariantList", notify=settingsChanged)
    def pref1Model(self): return self._get_pref_model("fastCallIdx1")

    @Property("QVariantList", notify=settingsChanged)
    def pref2Model(self): return self._get_pref_model("fastCallIdx2")

    @Property("QVariantList", notify=settingsChanged)
    def pref3Model(self): return self._get_pref_model("fastCallIdx3")

    # --- QML Active Selection Properties ---

    @Property(int, notify=settingsChanged)
    def floorIdx(self): return self._settings.get("floorIdx", -1)

    @Property(int, notify=settingsChanged)
    def fastCallIdx1(self): return self._settings.get("fastCallIdx1", -1)

    @Property(int, notify=settingsChanged)
    def fastCallIdx2(self): return self._settings.get("fastCallIdx2", -1)

    @Property(int, notify=settingsChanged)
    def fastCallIdx3(self): return self._settings.get("fastCallIdx3", -1)

    # --- QML Interaction Slots ---

    @Slot(int)
    def setMainFloor(self, floor_id):
        """
        Sets the primary device floor location.
        Clears any shortcut preference that matches the newly selected location floor.
        """
        if self._settings.get("floorIdx") != floor_id:
            self._settings["floorIdx"] = floor_id

            # Clear shortcut slot if it matches the new main floor location
            for key in ["fastCallIdx1", "fastCallIdx2", "fastCallIdx3"]:
                if self._settings.get(key) == floor_id:
                    self._settings[key] = -1

            self.settingsChanged.emit()
            self._save_timer.start()

    @Slot(int, int)
    def setPrefFloor(self, pref_num, floor_id):
        """
        Assigns a shortcut floor preference (1, 2, or 3).
        Enforces unique selection by clearing ('stealing') the ID from other slots if previously assigned.
        """
        target_key = f"fastCallIdx{pref_num}"
        
        if self._settings.get(target_key) == floor_id:
            return

        if floor_id != -1:
            # Clear target ID from other preference slots if previously assigned elsewhere
            for num in [1, 2, 3]:
                if num != pref_num:
                    key = f"fastCallIdx{num}"
                    if self._settings.get(key) == floor_id:
                        self._settings[key] = -1

        self._settings[target_key] = floor_id
        self.settingsChanged.emit()
        self._save_timer.start()