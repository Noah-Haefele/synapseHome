import logging
import numpy as np
import sounddevice as sd
from queue import Queue, Full, Empty

logging.basicConfig(level=logging.INFO)

logger = logging.getLogger(__name__)


class PlaybackHandler:
    """
    Audio is mainly managed by the discovery.py file which uses pulseAudio to set default
    source and sink to the operating system...
    portAudio which is in use of sounddevice then takes the device id of pulse
    to let the pulseAudio mentioned in the discovery.py decide what sink to use
    """

    # Audio Parameters
    CHANNELS = 2
    SAMPLERATE = 48000
    BLOCKSIZE = 50
    DTYPE = np.int32

    def __init__(self, gain: float = 1.0):
        self.audio_queue = Queue(maxsize=50)

        self.is_running = False

        self.gain = gain

    def start(self):
        if self.is_running:
            return

        self.is_running = True
        self.play_audio = True
    
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

    def attachToAudioQueue(self, audio_data: np.ndarray) -> bool:
        try:
            self.audio_queue.put(audio_data, timeout=0.1)
            return True
        except Full:
            logger.error("Audio queue is full")
            return False
        except Exception as e:
            logger.error(f"Failed to attach audio data to queue: {e}")
            return False

    def playback_audio(self):
        """Main Playback of audio"""
        output_stream = None
        
        try:
            if self.play_audio:
                try:
                    output_stream = sd.OutputStream(
                        device=self.output_device,
                        channels=self.CHANNELS,
                        samplerate=self.SAMPLERATE,
                        blocksize=self.BLOCKSIZE,
                        dtype='float32',
                        latency='low'
                    )
                    output_stream.start()
                    logger.info("Audio stream started")
                except Exception as e:
                    logger.error(f"Stream-Fehler: {e}")
                    logger.info(f"Versuche alternatives Device...")

                    return
            
            while self.is_running:
                try:
                    audio_data = self.audio_queue.get(timeout=1)
                    
                    # Convertion: int32 → float32
                    audio_normalized = audio_data.astype(np.float32) / (2**31)
                    
                    # apply gain
                    if self.gain != 1.0:
                        audio_normalized = audio_normalized * self.gain
                    
                    audio_normalized = np.clip(audio_normalized, -1.0, 1.0)
                    
                    if self.play_audio and output_stream:
                        output_stream.write(audio_normalized)
                
                except Empty:
                    continue
                except Exception as e:
                    logger.error("Error during audio output write: %s", e)
        
        finally:
            if output_stream:
                try:
                    output_stream.stop()
                    output_stream.close()
                    logger.info("Audio stream closed")
                except Exception as e:
                    logger.error("Error closing audio stream: %s", e)

    def stop(self):
        self.is_running = False
        self.play_audio = False