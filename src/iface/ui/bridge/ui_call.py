from PySide6.QtCore import QObject, Property, Signal, Slot

from src.core.set.floor_manager import FloorManager
from src.core.net.node import CallNode


class UICall(QObject):
    callStateChanged = Signal(str)
    targetFloorChanged = Signal(int)
    destinationLabelChanged = Signal()

    def __init__(self, floor_manager: FloorManager, call_node: CallNode):
        super().__init__()

        self._floor_manager = floor_manager
        self._call_node = call_node
        self._call_state = "IDLE"
        self._target_floor_id = -1

        # Register the callback for incoming calls
        self._call_node.on_incoming_call = self._on_incoming_call
        # Register the callback for accepted calls
        self._call_node.on_call_accepted = self._on_call_accepted
        # Register the callback for stopped calls
        self._call_node.on_call_stopped = self._on_call_stopped

    @Property(str, notify=callStateChanged)
    def callState(self) -> str:
        return self._call_state

    @Property(int, notify=targetFloorChanged)
    def targetFloor(self) -> int:
        return self._target_floor_id if self._target_floor_id != -1 else -1

    @Property(str, notify=destinationLabelChanged)
    def destinationLabel(self) -> str:
        """Returns label to indicate which floor is being called"""
        if self._target_floor_id == -1:
            return "Unknown…"

        label = self._floor_manager.get_floor_name(self._target_floor_id)
        # show ellipsis while waiting for call acceptance
        if self._call_state == "CALLING":
            label += " ..."
        elif self._call_state == "RINGING":
            label += " is calling..."

        return label
    
    @Slot(int)
    def initiateCall(self, floorId: int):
        """QML triggers call"""
        self._target_floor_id = floorId
        self._call_state = "CALLING"

        self._call_node.dial(self._target_floor_id)

        self.targetFloorChanged.emit(self._target_floor_id)
        self.destinationLabelChanged.emit()
        self.callStateChanged.emit(self._call_state)

    @Slot()
    def acceptCall(self):
        """Accepts incoming call"""
        self._call_state = "CONNECTED"

        self._call_node.accept_call()

        self.callStateChanged.emit(self._call_state)

    @Slot()
    def endCall(self):
        """Ends call / rejects incoming call"""
        self._call_state = "IDLE"
        self._target_floor_id = -1

        self._call_node.stop_call()

        self.targetFloorChanged.emit(self._target_floor_id)
        self.destinationLabelChanged.emit()
        self.callStateChanged.emit(self._call_state)

    def _on_incoming_call(self, call_floor_id: int) -> None:
        """Callback handler invoked by CallNode when an inbound call arrives."""
        # Just take up the call if line is idle
        if self._call_state != "IDLE":
            return

        self._target_floor_id = call_floor_id
        self._call_state = "RINGING"

        self.targetFloorChanged.emit(self._target_floor_id)
        self.destinationLabelChanged.emit()
        self.callStateChanged.emit(self._call_state)

    def _on_call_accepted(self) -> None:
        """Callback handler invoked by CallNode when the call is accepted by the other device."""
        self._call_state = "CONNECTED"

        self.destinationLabelChanged.emit()
        self.callStateChanged.emit(self._call_state)

    def _on_call_stopped(self) -> None:
        """Callback handler invoked by CallNode when the call is stopped by the other device by either rejecting or ending the call."""
        self._call_state = "IDLE"
        self._target_floor_id = -1

        self.targetFloorChanged.emit(self._target_floor_id)
        self.destinationLabelChanged.emit()
        self.callStateChanged.emit(self._call_state)