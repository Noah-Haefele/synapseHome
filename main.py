import sys
from PySide6.QtGui import QGuiApplication
from PySide6.QtQml import QQmlApplicationEngine
from pathlib import Path

from src.iface.ui.bridge.ui_set import UiSet
from src.iface.ui.bridge.ui_call import UICall
from src.core.set.floor_manager import FloorManager
# Networking
from src.core.net.utils import NetworkInterface
from src.iface.ui.bridge.ui_net import UINetwork

def main():
    app = QGuiApplication(sys.argv)

    floor_manager = FloorManager()
    ui_handler = UiSet(floor_manager=floor_manager)
    call_handler = UICall(floor_manager=floor_manager)
    # Networking
    core_network = NetworkInterface()
    ui_network_bridge = UINetwork(net_interface=core_network)


    engine = QQmlApplicationEngine()
    engine.rootContext().setContextProperty("uiHandler", ui_handler)
    engine.rootContext().setContextProperty("networkHandler", ui_network_bridge)
    engine.rootContext().setContextProperty("callHandler", call_handler)

    BASE_DIR = Path(__file__).resolve().parent
    qml_file = BASE_DIR / "src" / "iface" / "ui" / "Main.qml"

    engine.load(str(qml_file))

    if not engine.rootObjects():
        print(f"Error loading QML file: {qml_file}")
        sys.exit(-1)
    
    exit_code = app.exec()

    sys.exit(exit_code)

if __name__ == "__main__":
    main()