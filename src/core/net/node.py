import logging
from typing import Callable, Optional

from src.core.mqtt.mq_hdl import MQTTHandler
from src.core.set.floor_manager import FloorManager
from src.core.net.utils import NetworkInterface

logger = logging.getLogger(__name__)

class CallNode():
    def __init__(self, net_interface: NetworkInterface, floor_manager: FloorManager, mqtt_handler: MQTTHandler):
        self._net_interface = net_interface
        self._floor_manager = floor_manager
        self._mqtt_handler = mqtt_handler

        # Incoming call callback, to notify the UI bridge
        self.on_incoming_call: Optional[Callable[[int], None]] = None
        # Accepted call callback, to notify the UI bridge
        self.on_call_accepted: Optional[Callable[[int], None]] = None

        self._mqtt_handler.start()  # Start the MQTT handler when the CallNode is initialized
        self._subscribe_to_call_message()  # Subscribe to the call topic for this floor

    def _subscribe_to_call_message(self):
        """Subscribes to the call topic for this floor to listen for incoming calls."""
        subtopic = f"call/{self._floor_manager.floor_idx}"

        self._mqtt_handler.subscribe(subtopic, self._handle_incoming_call_message)

    def dial(self, target_floor_id: int):
        """Initiates a call to the specified target floor by publishing a message to the corresponding MQTT topic."""
        subtopic = f"call/{target_floor_id}"
        payload = f"CALLING:{self._floor_manager.floor_idx}:{self._net_interface.ip_address}"

        self._mqtt_handler.publish(subtopic, payload)

        logger.info(
            "Dialing floor %s from floor %s with IP: %s",
            target_floor_id,
            self._floor_manager.floor_idx,
            self._net_interface.ip_address
        )

    def accept_call(self):
        """Accepts an incoming call by publishing a message to the corresponding MQTT topic."""
        subtopic = f"call/{self._caller_floor_id}"
        payload = f"ACCEPTED:{self._floor_manager.floor_idx}:{self._net_interface.ip_address}"

        self._mqtt_handler.publish(subtopic, payload)

        logger.info(
            "Accepted call from floor %s as floor %s with IP: %s",
            self._caller_floor_id,
            self._floor_manager.floor_idx,
            self._net_interface.ip_address
        )

    def _handle_incoming_call_message(self, payload: str) -> None:
        """Internal handler for incoming call messages."""
        try:
            # Format: "floor_id:ip_address"
            state, caller_floor_str, caller_ip = payload.split(":", 2)
            self._caller_floor_id = int(caller_floor_str)

            if (state == "CALLING"):
                # Notify the UI bridge about incoming call if the callback is set
                if self.on_incoming_call:
                    self.on_incoming_call(self._caller_floor_id)

                logger.info(
                    "Incoming call from floor %s with IP: %s",
                    self._caller_floor_id,
                    caller_ip
                )
            elif (state == "ACCEPTED"):
                # Notify the UI bridge about call acceptance if the callback is set
                if self.on_call_accepted:
                    self.on_call_accepted(self._caller_floor_id)

        except ValueError:
            logger.error("Failed to parse incoming call payload: %s", payload)