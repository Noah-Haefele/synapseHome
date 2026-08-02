import logging
from typing import Callable
from threading import Lock
import paho.mqtt.client as mqtt
from src.core.mqtt.mq_conf import MQTTConfig

logger = logging.getLogger(__name__)

class MQTTHandler:
    def __init__(self, mqtt_config: MQTTConfig):
        self._mqtt_config = mqtt_config

        self._subscriptions: dict[str, Callable[[str], None]] = {}
        self._subscription_lock = Lock()

        self._mqttc = mqtt.Client(
            client_id=self._mqtt_config.client_id
        )

        self._mqttc.on_connect = self._on_connect
        self._mqttc.on_disconnect = self._on_disconnect
        self._mqttc.on_message = self._on_message

        self._mqttc.reconnect_delay_set(
            min_delay=1,
            max_delay=60
        )

        self._running = False

    def start(self):
        """Starts the MQTT client, connects to the broker, and starts the network loop."""
        if self._running:
            return  # Already running
        
        try:
            self._mqttc.connect(
                self._mqtt_config.host, 
                self._mqtt_config.port, 
                self._mqtt_config.keepalive
            )
            self._mqttc.loop_start()

            self._running = True
        except Exception:
            self._running = False
            logger.exception("MQTT connection failed")

    def stop(self):
        """Stops the MQTT network loop and disconnects from the broker."""
        if not self._running:
            return  # Not running

        if self._mqttc.is_connected():
            self._mqttc.disconnect()
        self._mqttc.loop_stop()

        self._running = False

    def subscribe(self, subtopic: str, callback: Callable[[str], None]):
        """
        Subscribes to a subtopic and registers a callback for incoming messages.
        The configured topic prefix is automatically added to the given subtopic.
        """
        full_topic = f"{self._mqtt_config.topic_prefix}/{subtopic}"

        with self._subscription_lock:
            self._subscriptions[full_topic] = callback

        if self._mqttc.is_connected():
            result, mid = self._mqttc.subscribe(full_topic)

            if result != mqtt.MQTT_ERR_SUCCESS:
                logger.error("Failed to subscribe to topic: %s", full_topic)
            else:
                logger.info("Subscribed to topic: %s", full_topic)

    def unsubscribe(self, subtopic: str):
        """
        Removes a subscription and unregisters its callback.
        The configured topic prefix is automatically added to the given subtopic.
        """
        full_topic = f"{self._mqtt_config.topic_prefix}/{subtopic}"

        with self._subscription_lock:
            self._subscriptions.pop(full_topic, None)

        if self._mqttc.is_connected():
            result, mid = self._mqttc.unsubscribe(full_topic)

            if result != mqtt.MQTT_ERR_SUCCESS:
                logger.error(
                    "Failed to unsubscribe from topic: %s",
                    full_topic
                )
            else:
                logger.info(f"Unsubscribed from topic: {full_topic}")

    def publish(self, subtopic: str, payload: str, qos: int = 1, retain: bool = False) -> None:
        """Publishes a payload to the broker under the configured topic prefix."""
        full_topic = f"{self._mqtt_config.topic_prefix}/{subtopic}"
        result = self._mqttc.publish(full_topic, payload, qos=qos, retain=retain)
        
        if result.rc != mqtt.MQTT_ERR_SUCCESS:
            logger.warning("Failed to queue message for topic %s (rc: %d)", full_topic, result.rc)

    # --- Callbacks --- 
    
    def _on_message(self, client, userdata, msg) -> None:
        topic = msg.topic
        payload = msg.payload.decode("utf-8", errors="replace")

        with self._subscription_lock:
            callback = self._subscriptions.get(topic)

        logger.debug(
            "Received message on topic %s: %s",
            topic,
            payload
        )

        if callback:
            try:
                callback(payload)
            except Exception:
                logger.exception(
                    "Error handling MQTT message on %s",
                    topic
                )

    def _on_connect(self, client, userdata, flags, rc) -> None:
        if rc == 0:
            logger.info("Connected to MQTT broker as '%s'", self._mqtt_config.client_id)

            with self._subscription_lock:
                topics = list(self._subscriptions.keys())

            for topic in topics:
                self._mqttc.subscribe(topic)
        else:
            logger.error(f"Failed to connect to MQTT broker, return code {rc}")

    def _on_disconnect(self, client, userdata, rc) -> None:
        if rc != 0:
            logger.warning("Unexpected MQTT disconnect (rc: %d). Auto-reconnecting...", rc)
