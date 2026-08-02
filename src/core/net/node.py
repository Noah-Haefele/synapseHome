from src.core.mqtt.mq_hdl import MQTTHandler
from src.core.set.floor_manager import FloorManager
from src.core.net.utils import NetworkInterface

class CallNode():
    def __init__(self, net_interface: NetworkInterface, floor_manager: FloorManager, mqtt_handler: MQTTHandler):
        self._net_interface = net_interface
        self._floor_manager = floor_manager
        self._mqtt_handler = mqtt_handler
        self._mqtt_handler.start()  # Start the MQTT handler when the CallNode is initialized

    def dial(self, target_floor_id: int):
        topic = f"call/{target_floor_id}"
        message = f"{self._floor_manager.floor_idx}:{self._net_interface.ip_address}"
        self._mqtt_handler._mqttc.publish(topic, message)