import logging
from typing import List, Dict, Any
import pulsectl

logger = logging.getLogger(__name__)

class AudioDeviceDiscovery:
    """Manages audio hardware discovery."""

    def __init__(self):
        try:
            self.pulse = pulsectl.Pulse("audio-discovery")
            logger.info("PulseAudio connection established")
        except pulsectl.PulseError as e:
            logger.error(f"Failed to connect to PulseAudio: {e}")
            raise RuntimeError("Could not initialize audio device discovery") from e

    def get_input_devices(self) -> List[Dict[str, Any]]:
        """Returns all available input devices (live detection)."""
        try:
            inputs = [
                {"id": src.index, "name": src.description, "channels": src.channel_count}
                for src in self.pulse.source_list()
                if "Monitor" not in src.name
            ]
            logger.info(f"Found {len(inputs)} input device(s)")
            return inputs
        except pulsectl.PulseError as e:
            logger.error(f"Error querying input devices: {e}")
            return []

    def get_output_devices(self) -> List[Dict[str, Any]]:
        """Returns all available output devices (live detection)."""
        try:
            outputs = [
                {"id": sink.index, "name": sink.description, "channels": sink.channel_count}
                for sink in self.pulse.sink_list()
            ]
            logger.info(f"Found {len(outputs)} output device(s)")
            return outputs
        except pulsectl.PulseError as e:
            logger.error(f"Error querying output devices: {e}")
            return []

    def close(self):
        """Clean up connection."""
        self.pulse.close()
