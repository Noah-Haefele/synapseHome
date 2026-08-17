import logging
import numpy as np
import sounddevice as sd
import time
from queue import Queue, Empty

logger = logging.getLogger(__name__)


class RecordHandler:
    CHANNELS = 1
    SAMPLERATE = 48000
    BLOCKSIZE = 128
    DTYPE = np.int32

    def __init__(self):

        self.audio_queue = Queue(maxsize=50)
        
        self.is_running = False
        self.input_device = self._find_portaudio_device()
        self.stream = None

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

    def start(self):
        if self.is_running:
            return

        # Clear queue
        while not self.audio_queue.empty():
            try:
                self.audio_queue.get_nowait()
            except Empty:
                break

        self.is_running = True

        def audio_callback(indata, frames, time_info, status):
            if status:
                logger.debug(f"Audio status: {status}")

            try:
                # Ring Buffer behavior
                if self.audio_queue.full():
                    try:
                        self.audio_queue.get_nowait()
                    except Empty:
                        pass
                
                self.audio_queue.put_nowait(indata.copy())
            except Exception as e:
                logger.error(f"Error in record callback: {e}")

        try:
            self.stream = sd.InputStream(
                device=self.input_device,
                channels=self.CHANNELS,
                samplerate=self.SAMPLERATE,
                blocksize=self.BLOCKSIZE,
                dtype=self.DTYPE,
                callback=audio_callback,
                latency='low'
            )
            self.stream.start()
            logger.info("Audio recording started (Callback Mode)")
        except Exception as e:
            logger.error(f"Failed to start recording stream: {e}")
            self.is_running = False

    def get_audio_queue(self) -> None | np.ndarray:
            try:
                return self.audio_queue.get(timeout=1)
            except Empty:
                return None
            except Exception as e:
                logger.error(f"Failed to get audio data from queue: {e}")
                return None

    def stop(self):
        self.is_running = False
        if self.stream:
            self.stream.stop()
            self.stream.close()
            logger.info("Recording stopped")