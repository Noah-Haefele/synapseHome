import logging
import sys
from pathlib import Path

from PySide6.QtGui import QGuiApplication
from PySide6.QtQml import QQmlApplicationEngine

from src.core.net.utils import NetworkInterface
from src.core.set.floor_manager import FloorManager
from src.core.mqtt.mq_conf import MQTTConfig
from src.core.mqtt.mq_hdl import MQTTHandler
from src.iface.ui.bridge.ui_call import UICall
from src.iface.ui.bridge.ui_net import UINetwork
from src.iface.ui.bridge.ui_set import UiSet

logging.basicConfig(level=logging.INFO, format="%(asctime)s [%(levelname)s] %(name)s: %(message)s")
logger = logging.getLogger("Main")

def main():
    app = QGuiApplication(sys.argv)

    core_network = NetworkInterface()
    floor_manager = FloorManager()
    mqtt_config = MQTTConfig()
    mqtt_handler = MQTTHandler(mqtt_config=mqtt_config)

    # UI Bridges
    ui_handler = UiSet(floor_manager=floor_manager)
    ui_network_bridge = UINetwork(net_interface=core_network)
    call_handler = UICall(floor_manager=floor_manager)


    # QML Engine
    engine = QQmlApplicationEngine()
    engine.rootContext().setContextProperty("uiHandler", ui_handler)
    engine.rootContext().setContextProperty("networkHandler", ui_network_bridge)
    engine.rootContext().setContextProperty("callHandler", call_handler)

    BASE_DIR = Path(__file__).resolve().parent
    qml_file = BASE_DIR / "src" / "iface" / "ui" / "Main.qml"
    engine.load(str(qml_file))

    if not engine.rootObjects():
        logger.error("Failed to load QML interface: %s", qml_file)
        sys.exit(-1)
    
    sys.exit(app.exec())

if __name__ == "__main__":
    main()