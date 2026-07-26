import json
import logging
import os
from pathlib import Path
from PySide6.QtCore import QObject, Signal, QTimer

logger = logging.getLogger(__name__)


class FloorManager(QObject):
    """Manages persistent floor configurations, device locations, and shortcut preferences."""

    stateChanged = Signal()

    def __init__(self):
        super().__init__()

        base_dir = Path(__file__).resolve().parents[3]
        self._settings_file = base_dir / "data" / "settings.json"
        self._config_file = base_dir / "config" / "floors.json"

        # Default user settings
        self._settings = {
            "floorIdx": 1,
            "fastCallIdx1": 2,
            "fastCallIdx2": 3,
            "fastCallIdx3": -1
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

        self._floors_by_id = {}

        # Debounce timer for non-blocking disk persistence (1s delay)
        self._save_timer = QTimer(self)
        self._save_timer.setSingleShot(True)
        self._save_timer.setInterval(1000)
        self._save_timer.timeout.connect(self._save_settings)

        self.load_settings()
        self.load_config()

    def load_settings(self) -> None:
        """Loads user settings from disk or creates default settings if missing."""
        if self._settings_file.exists():
            try:
                with open(self._settings_file, "r", encoding="utf-8") as f:
                    new_settings = json.load(f)
                self._settings.update({k: v for k, v in new_settings.items() if k in self._settings})
                self.stateChanged.emit()
            except Exception as e:
                logger.error(f"Settings could not be loaded: {e}")
        else:
            self._save_settings()

    def load_config(self) -> None:
        """Loads floor configuration from disk or creates default config if missing."""
        if self._config_file.exists():
            try:
                with open(self._config_file, "r", encoding="utf-8") as f:
                    self._config = json.load(f)        
            except Exception as e:
                logger.error(f"Configs could not be loaded: {e}")
        else:
            self._save_config()

        self._rebuild_floor_map()
        self.stateChanged.emit()

    def _rebuild_floor_map(self):
        self._floors_by_id = {
            floor["floorId"]: floor
            for floor in self._config
        }

    def _save_settings(self) -> None:
        self._do_save_to_disk(self._settings, self._settings_file)

    def _save_config(self) -> None:
        self._do_save_to_disk(self._config, self._config_file)

    def _do_save_to_disk(self, data, data_file: Path) -> None:
        """Atomically saves JSON data using a temporary file to prevent corruption on interrupt."""
        temp_file = data_file.with_suffix(".tmp")
        try:
            data_file.parent.mkdir(parents=True, exist_ok=True)
            with open(temp_file, "w", encoding="utf-8") as f:
                json.dump(data, f, indent=4)
                f.flush()
                os.fsync(f.fileno())
            temp_file.replace(data_file)
        except Exception as e:
            logger.error(f"Could not save to {data_file}: {e}")

    @property
    def all_floors(self):
        return self._config

    @property
    def floor_idx(self):
        return self._settings.get("floorIdx", -1)

    def get_pref(self, num: int) -> int:
        return self._settings.get(f"fastCallIdx{num}", -1)

    def get_pref_model(self) -> list:
        model = [{"name": "Unassigned", "floorId": -1, "iconPath": "", "type": "empty"}]
        main_floor = self.floor_idx
        for floor in self._config:
            if floor.get("floorId") == main_floor:
                continue
            model.append(floor)
        return model

    def get_pref_icon_path(self, pref_num: int) -> str:
        floor_id = self.get_pref(pref_num)

        if floor_id == -1:
            return ""
        
        floor = self._floors_by_id.get(floor_id)

        if floor is None:
            return ""

        return floor["iconPath"]

    def set_main_floor(self, floor_id: int) -> None:
        if self._settings.get("floorIdx") != floor_id:
            self._settings["floorIdx"] = floor_id
            for key in ["fastCallIdx1", "fastCallIdx2", "fastCallIdx3"]:
                if self._settings.get(key) == floor_id:
                    self._settings[key] = -1
            self.stateChanged.emit()
            self._save_timer.start()

    def set_pref_floor(self, pref_num: int, floor_id: int) -> None:
        target_key = f"fastCallIdx{pref_num}"
        if self._settings.get(target_key) == floor_id:
            return

        if floor_id != -1:
            for num in (1, 2, 3):
                if num != pref_num:
                    key = f"fastCallIdx{num}"
                    if self._settings.get(key) == floor_id:
                        self._settings[key] = -1

        self._settings[target_key] = floor_id
        self.stateChanged.emit()
        self._save_timer.start()