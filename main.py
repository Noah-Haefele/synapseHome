import logging
import sys
from pathlib import Path

from PySide6.QtGui import QGuiApplication
from PySide6.QtQml import QQmlApplicationEngine

from src.core.net.utils import NetworkInterface
from src.core.net.node import CallNode
from src.core.net.audio.receive import AudioReceiver
from src.core.net.audio.send import AudioSender
from src.core.set.floor_manager import FloorManager
from src.core.set.general import SettingsManager
from src.core.mqtt.mq_conf import MQTTConfig
from src.core.mqtt.mq_hdl import MQTTHandler
from src.core.display.activity import ActivityFilter
from src.core.act.audio.discovery import AudioDeviceDiscovery
from src.core.act.audio.playback import AudioHandler
from src.core.act.audio.record import RecordHandler
from src.hardware.display import DisplaySetter
from src.iface.ui.bridge.ui_call import UICall
from src.iface.ui.bridge.ui_net import UINetwork
from src.iface.ui.bridge.ui_set import UiSet

logging.basicConfig(level=logging.INFO, format="%(asctime)s [%(levelname)s] %(name)s: %(message)s")
logger = logging.getLogger("Main")

def main():
    app = QGuiApplication(sys.argv)

    core_network = NetworkInterface()
    floor_manager = FloorManager()
    general_settings_manager = SettingsManager()
    mqtt_config = MQTTConfig()
    mqtt_handler = MQTTHandler(mqtt_config=mqtt_config)
    call_node = CallNode(net_interface=core_network, floor_manager=floor_manager, mqtt_handler=mqtt_handler)
    audio_device_disc = AudioDeviceDiscovery()

    # Apply saved audio device selections to PulseAudio on startup
    saved_output = general_settings_manager.get_setting("audioO")
    if saved_output:
        audio_device_disc.set_default_sink(saved_output)
    saved_input = general_settings_manager.get_setting("audioI")
    if saved_input:
        audio_device_disc.set_default_source(saved_input)

    # Get audio
    audio_handler = AudioHandler()
    audio_receiver = AudioReceiver(audio_handler=audio_handler)
    # Stream audio
    record_handler = RecordHandler()
    audio_sender = AudioSender(record_handler=record_handler)

    # UI Bridges
    ui_handler = UiSet(floor_manager=floor_manager, general_settings_manager=general_settings_manager, audio_device_discovery=audio_device_disc)
    ui_network_bridge = UINetwork(net_interface=core_network)
    call_handler = UICall(floor_manager=floor_manager, call_node=call_node)

    display_hardware = DisplaySetter()
    display_manager = ActivityFilter(display_hardware=display_hardware, general_settings_manager=general_settings_manager)
    app.installEventFilter(display_manager)


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