import logging
from PySide6.QtCore import QObject, Property, Signal, Slot
from src.core.set.floor_manager import FloorManager

logger = logging.getLogger(__name__)


class UiSet(QObject):
    """QML bridge exposing FloorManager properties and methods to the user interface."""

    settingsChanged = Signal()

    def __init__(self, floor_manager: FloorManager):
        super().__init__()
        self._floor_manager = floor_manager
        self._floor_manager.stateChanged.connect(self.settingsChanged.emit)

    @Property("QVariantList", notify=settingsChanged)
    def allFloors(self):
        return self._floor_manager.all_floors

    @Property("QVariantList", notify=settingsChanged)
    def pref1Model(self):
        return self._floor_manager.get_pref_model()

    @Property("QVariantList", notify=settingsChanged)
    def pref2Model(self):
        return self._floor_manager.get_pref_model()

    @Property("QVariantList", notify=settingsChanged)
    def pref3Model(self):
        return self._floor_manager.get_pref_model()

    @Property(int, notify=settingsChanged)
    def floorIdx(self):
        return self._floor_manager.floor_idx

    @floorIdx.setter
    def floorIdx(self, val: int):
        self._floor_manager.set_main_floor(int(val))

    @Property(int, notify=settingsChanged)
    def fastCallIdx1(self):
        return self._floor_manager.get_pref(1)

    @Property(int, notify=settingsChanged)
    def fastCallIdx2(self):
        return self._floor_manager.get_pref(2)

    @Property(int, notify=settingsChanged)
    def fastCallIdx3(self):
        return self._floor_manager.get_pref(3)

    @Slot(int)
    def setMainFloor(self, floor_id: int) -> None:
        self._floor_manager.set_main_floor(floor_id)

    @Slot(int, int)
    def setPrefFloor(self, pref_num: int, floor_id: int) -> None:
        self._floor_manager.set_pref_floor(pref_num, floor_id)