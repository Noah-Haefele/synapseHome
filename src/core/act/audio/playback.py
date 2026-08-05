import logging
import numpy as np
import sounddevice as sd
from queue import Queue

from src.core.set.general import SettingsManager

logging.basicConfig(level=logging.INFO)

logger = logging.getLogger(__name__)


class AudioHandler:
    """
    Audio is mainly managed by the discovery.py file which uses pulseAudio to set default
    source and sink to the operating system...
    portAudio which is in use of sounddevice then takes the device id of pulse
    to let the pulseAudio mentioned in the discovery.py decide what sink to use
    """
    
    # Audio Parameters
    CHANNELS = 2
    SAMPLERATE = 48000
    BLOCKSIZE = 256
    DTYPE = np.int32

    def __init__(self):
        self.audio_queue = Queue(maxsize=50)

        self.output_device = self._find_portaudio_device()

    def _find_portaudio_device(self, name: str = "pulse") -> int | None:
        logger.info(f"Searching for PortAudio device: {name}")

        for i, dev in enumerate(sd.query_devices()):
            if name == dev["name"]:
                logger.info(
                    f"Found PortAudio device '{dev['name']}' with ID {i}"
                )
                return i

        logger.warning(
            f"PortAudio device '{name}' not found, using default device"
        )
        return None

if __name__ == "__main__":
    ah = AudioHandler(general_settings_manager=SettingsManager())