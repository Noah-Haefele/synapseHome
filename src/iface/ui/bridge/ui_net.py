from PySide6.QtCore import QObject, Property, Signal
from src.core.net.utils import NetworkInterface

class UINetwork(QObject):
    ip_changed = Signal(str)

    def __init__(self, net_interface: NetworkInterface):
        super().__init__()
        self._net_interface = net_interface

    @Property(str, notify=ip_changed)
    def ipAddress(self) -> str:
        return self._net_interface.ip_address