import logging
from PySide6.QtCore import QObject, Property, Signal, Slot
from src.core.set.floor_manager import FloorManager
from src.core.set.general import SettingsManager

logger = logging.getLogger(__name__)


class UiSet(QObject):
    """QML bridge exposing FloorManager properties and methods to the user interface."""

    settingsChanged = Signal()

    def __init__(self, floor_manager: FloorManager, general_settings_manager=SettingsManager):
        super().__init__()
        self._floor_manager = floor_manager
        self._settings_manager = general_settings_manager

        self._floor_manager.stateChanged.connect(self.settingsChanged.emit)
        self._settings_manager.settingsChanged.connect(self.settingsChanged.emit)

    @Property("QVariantList", notify=settingsChanged)
    def allFloors(self):
        return self._floor_manager.all_floors

    # all posible options for pref dropdown
    @Property("QVariantList", notify=settingsChanged)
    def prefModel(self):
        return self._floor_manager.get_pref_model()


    @Property(int, notify=settingsChanged)
    def floorIdx(self):
        return self._floor_manager.floor_idx

    @floorIdx.setter
    def floorIdx(self, val: int):
        self._floor_manager.set_main_floor(int(val))

    # actual value of each dropdown
    @Property(int, notify=settingsChanged)
    def fastCallIdx1(self):
        return self._floor_manager.get_pref(1)

    @Property(int, notify=settingsChanged)
    def fastCallIdx2(self):
        return self._floor_manager.get_pref(2)

    @Property(int, notify=settingsChanged)
    def fastCallIdx3(self):
        return self._floor_manager.get_pref(3)

    # icon path for each icon in ControlGrid based on setting of dropdown
    @Property(str, notify=settingsChanged)
    def pref1IconPath(self):
        return self._floor_manager.get_pref_icon_path(1)

    @Property(str, notify=settingsChanged)
    def pref2IconPath(self):
        return self._floor_manager.get_pref_icon_path(2)

    @Property(str, notify=settingsChanged)
    def pref3IconPath(self):
        return self._floor_manager.get_pref_icon_path(3)
    
    # short name for each call icon in ControlGrid
    @Property(str, notify=settingsChanged)
    def pref1ShortName(self):
        return self._floor_manager.get_floor_short_name(
            self._floor_manager.get_pref(1)
        )

    @Property(str, notify=settingsChanged)
    def pref2ShortName(self):
        return self._floor_manager.get_floor_short_name(
            self._floor_manager.get_pref(2)
        )

    @Property(str, notify=settingsChanged)
    def pref3ShortName(self):
        return self._floor_manager.get_floor_short_name(
            self._floor_manager.get_pref(3)
        )

    @Slot(int)
    def setMainFloor(self, floor_id: int) -> None:
        self._floor_manager.set_main_floor(floor_id)

    @Slot(int, int)
    def setPrefFloor(self, pref_num: int, floor_id: int) -> None:
        self._floor_manager.set_pref_floor(pref_num, floor_id)

    @Slot(int, result=int)
    def getPrefFloor(self, pref_num: int):
        return self._floor_manager.get_pref(pref_num)

    # --- Brightness and display standby time settings --- 

    @Property(int, notify=settingsChanged)
    def brightness(self): return self._settings_manager.get_brightness()
    @brightness.setter
    def brightness(self, val): self._settings_manager.set_setting("brightness", int(val))
    
    @Property(int, notify=settingsChanged)
    def displayTime(self): return self._settings_manager.get_display_time()
    @displayTime.setter
    def displayTime(self, val): self._settings_manager.set_setting("displayTime", int(val))