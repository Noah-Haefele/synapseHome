import logging
import numpy as np
import sounddevice as sd
import time
from queue import Queue, Empty, Full

logger = logging.getLogger(__name__)


class RecordHandler:
    CHANNELS = 1
    SAMPLERATE = 48000
    BLOCKSIZE = 128
    DTYPE = np.int32

    def __init__(self):

        self.audio_queue = Queue(maxsize=50)
        
        self.is_running = False

    def start(self):
        if self.is_running:
            return

        self.is_running = True

        self.input_device = self._find_portaudio_device()

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

    def get_audio_queue(self) -> None | np.ndarray:
            try:
                return self.audio_queue.get(timeout=1)
            except Empty:
                return None
            except Exception as e:
                logger.error(f"Failed to get audio data from queue: {e}")
                return None

    def record_audio(self):
        """ Record audio from selected input device (in portaudio always pulse because pulsaudio decides source) """
        try:
            def audio_callback(indata, frames, time_info, status):
                if status:
                    logger.info(f"Audio status {status}")

                try:
                    self.audio_queue.put_nowait(indata.copy())
                except Full:
                    logger.warning("Audio queue is full")

            # Start stream
            with sd.InputStream(
                device=self.input_device,
                channels=self.CHANNELS,
                samplerate=self.SAMPLERATE,
                blocksize=self.BLOCKSIZE,
                dtype=self.DTYPE,
                callback=audio_callback,
                latency='low'
            ):
                logger.info(f"Audio recording started")
                while self.is_running:
                    time.sleep(0.1)
        except Exception as e:
            logger.error(f"An error accured: {e}")

        finally:
            self.stop()

    def stop(self):
        self.is_running = False
        logger.info("Recording stopped")