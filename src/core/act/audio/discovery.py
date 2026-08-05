import logging
from typing import List, Dict, Any
import pulsectl

logger = logging.getLogger(__name__)


class AudioDeviceDiscovery:
    """
    Manages discovery and system-wide routing of audio hardware using PulseAudio.
    
    This class handles:
    1. Identifying physical inputs (filtering out virtual monitor loopbacks).
    2. Expanding output options to include distinct ports (e.g., Speakers vs 
       Headphones) and inactive but available card profiles (e.g., secondary HDMI ports).
    3. Dynamically routing system audio by modifying PulseAudio default sinks/sources, 
       active sink ports, or card profiles.
    """

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
                {
                    "id": src.name, 
                    "name": src.description, 
                    "channels": src.channel_count
                }
                for src in self.pulse.source_list()
                if "monitor" not in src.name.lower()
            ]
            logger.info(f"Found {len(inputs)} input device(s)")
            return inputs
        except pulsectl.PulseError as e:
            logger.error(f"Error querying input devices: {e}")
            return []

    def get_output_devices(self) -> List[Dict[str, Any]]:
        """Returns all available output devices, including ports and card profiles."""
        try:
            outputs = []
            
            # 1. Active sinks and their ports
            for sink in self.pulse.sink_list():
                if sink.port_list:
                    for port in sink.port_list:
                        if port.available == "no":
                            continue
                        outputs.append({
                            "id": f"port:{sink.name}:{port.name}",
                            "name": f"{sink.description} - {port.description}",
                            "channels": sink.channel_count
                        })
                else:
                    outputs.append({
                        "id": f"sink:{sink.name}",
                        "name": sink.description,
                        "channels": sink.channel_count
                    })
            
            # 2. Available but inactive output profiles (e.g. secondary HDMI ports)
            for card in self.pulse.card_list():
                for profile in card.profile_list:
                    if not profile.available or not profile.name.startswith("output:hdmi-"):
                        continue
                    if card.profile_active and card.profile_active.name == profile.name:
                        continue
                    
                    profile_suffix = profile.name.replace("output:", "")
                    expected_suffix = profile_suffix.split("+")[0]
                    is_active = any(sink.name.endswith(expected_suffix) for sink in self.pulse.sink_list())
                    if not is_active:
                        card_desc = card.proplist.get("device.description") or card.name
                        outputs.append({
                            "id": f"profile:{card.name}:{profile.name}",
                            "name": f"{card_desc} - {profile.description}",
                            "channels": 2
                        })
            
            logger.info(f"Found {len(outputs)} output device(s)")
            return outputs
        except pulsectl.PulseError as e:
            logger.error(f"Error querying output devices: {e}")
            return []

    def set_default_sink(self, sink_id_or_name: str) -> bool:
        """Sets the default PulseAudio sink by name, port-specific ID, or card profile ID."""
        try:
            # Case 1: port-specific ID e.g. "port:sink_name:port_name"
            if sink_id_or_name.startswith("port:"):
                parts = sink_id_or_name.split(":", 2)
                if len(parts) == 3:
                    _, sink_name, port_name = parts
                    sink = self.pulse.get_sink_by_name(sink_name)
                    self.pulse.default_set(sink)
                    self.pulse.sink_port_set(sink.index, port_name)
                    logger.info(f"Successfully set default PulseAudio sink to: {sink_name} (port: {port_name})")
                    return True

            # Case 2: profile-specific ID e.g. "profile:card_name:profile_name"
            elif sink_id_or_name.startswith("profile:"):
                parts = sink_id_or_name.split(":", 2)
                if len(parts) == 3:
                    _, card_name, profile_name = parts
                    card = self.pulse.get_card_by_name(card_name)
                    self.pulse.card_profile_set(card, profile_name)
                    logger.info(f"Set card {card_name} profile to: {profile_name}")
                    
                    profile_suffix = profile_name.replace("output:", "")
                    expected_suffix = profile_suffix.split("+")[0]
                    
                    import time
                    for _ in range(5):
                        time.sleep(0.1)
                        for sink in self.pulse.sink_list():
                            if sink.name.endswith(expected_suffix) or card_name.replace("alsa_card.", "") in sink.name:
                                self.pulse.default_set(sink)
                                logger.info(f"Successfully set default PulseAudio sink to newly created: {sink.name}")
                                return True
                    logger.warning(f"Profile switched, but could not locate new sink for card: {card_name}")
                    return True

            # Case 3: sink-specific ID e.g. "sink:sink_name"
            elif sink_id_or_name.startswith("sink:"):
                sink_name = sink_id_or_name.replace("sink:", "", 1)
                sink = self.pulse.get_sink_by_name(sink_name)
                self.pulse.default_set(sink)
                logger.info(f"Successfully set default PulseAudio sink to: {sink_name}")
                return True

            # Case 4: direct sink name or fallback matching
            else:
                try:
                    sink = self.pulse.get_sink_by_name(sink_id_or_name)
                    self.pulse.default_set(sink)
                    logger.info(f"Successfully set default PulseAudio sink to: {sink_id_or_name}")
                    return True
                except Exception:
                    for sink in self.pulse.sink_list():
                        if sink.description == sink_id_or_name or sink.name == sink_id_or_name:
                            self.pulse.default_set(sink)
                            logger.info(f"Successfully set default PulseAudio sink to: {sink.name} (via match)")
                            return True
            logger.warning(f"Could not find PulseAudio sink: {sink_id_or_name}")
            return False
        except pulsectl.PulseError as e:
            logger.error(f"Error setting default PulseAudio sink: {e}")
            return False

    def set_default_source(self, source_id_or_name: str) -> bool:
        """Sets the default PulseAudio source by name or description."""
        try:
            try:
                source = self.pulse.get_source_by_name(source_id_or_name)
                self.pulse.default_set(source)
                logger.info(f"Successfully set default PulseAudio source to: {source_id_or_name}")
                return True
            except Exception:
                for source in self.pulse.source_list():
                    if source.description == source_id_or_name or source.name == source_id_or_name:
                        self.pulse.default_set(source)
                        logger.info(f"Successfully set default PulseAudio source to: {source.name} (via match)")
                        return True
            logger.warning(f"Could not find PulseAudio source: {source_id_or_name}")
            return False
        except pulsectl.PulseError as e:
            logger.error(f"Error setting default PulseAudio source: {e}")
            return False

    def close(self):
        """Clean up connection."""
        self.pulse.close()