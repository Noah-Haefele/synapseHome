from PySide6.QtCore import QObject, Property, Signal, Slot

from src.core.set.floor_manager import FloorManager


class UICall(QObject):
    callStateChanged = Signal(str)
    targetFloorChanged = Signal(int)
    destinationLabelChanged = Signal()

    def __init__(self, floor_manager: FloorManager):
        super().__init__()

        self._floor_manager = floor_manager
        self._call_state = "IDLE"
        self._target_floor_id = -1

    @Property(str, notify=callStateChanged)
    def callState(self) -> str:
        return self._call_state

    @Property(int, notify=targetFloorChanged)
    def targetFloor(self) -> int:
        return self._target_floor_id if self._target_floor_id != -1 else -1

    @Property(str, notify=destinationLabelChanged)
    def destinationLabel(self) -> str:
        if self._target_floor_id == -1:
            return "Unknown..."

        return self._floor_manager.get_floor_name(self._target_floor_id)

    @Slot(int)
    def startCall(self, floorId: int):
        """QML triggers call"""
        self._target_floor_id = floorId
        self._call_state = "CALLING"

        self.targetFloorChanged.emit(self._target_floor_id)
        self.destinationLabelChanged.emit()
        self.callStateChanged.emit(self._call_state)

    @Slot()
    def endCall(self):
        """Ends call / hand up"""
        self._call_state = "IDLE"
        self._target_floor_id = -1

        self.targetFloorChanged.emit(self._target_floor_id)
        self.destinationLabelChanged.emit()
        self.callStateChanged.emit(self._call_state)