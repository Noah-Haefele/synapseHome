from PySide6.QtCore import QObject, Property, Signal, Slot

class UICall(QObject):
    callStateChanged = Signal(str)
    targetFloorChanged = Signal(int)

    def __init__(self):
        super().__init__()
        
        self._call_state = "IDLE"
        self._target_floor_id = None

    @Property(str, notify=callStateChanged)
    def callState(self) -> str:
        return self._call_state

    @Property(int, notify=targetFloorChanged)
    def targetFloor(self) -> int:
        return self._target_floor_id

    @Slot(int)
    def startCall(self, floorId: int):
        """QML triggers call"""
        self._target_floor_id = floorId
        self._call_state = "CALLING"

        self.targetFloorChanged.emit(self._target_floor_id)
        self.callStateChanged.emit(self._call_state)

    @Slot()
    def endCall(self):
        """Ends call / hand up"""
        self._call_state = "IDLE"
        self._target_floor_id = None

        self.targetFloorChanged.emit(self._target_floor_id)
        self.callStateChanged.emit(self._call_state)