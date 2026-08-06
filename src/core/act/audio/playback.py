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
    CHANNELS = 1
    SAMPLERATE = 48000
    BLOCKSIZE = 128
    DTYPE = np.int32

    def __init__(self, gain: float = 1.0):
        self.audio_queue = Queue(maxsize=50)

        self.gain = gain

        self.is_running = False
        self.output_stream = None

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
    
        self.output_device = self._find_portaudio_device()

        try:
            self.output_stream = sd.OutputStream(
                device=self.output_device,
                channels=self.CHANNELS,
                samplerate=self.SAMPLERATE,
                blocksize=self.BLOCKSIZE,
                dtype='float32',
                latency='low',
                callback=self._audio_callback
            )
            self.output_stream.start()
            logger.info("Audio stream started")
        except Exception as e:
            logger.error(f"Error during stream start: {e}")
            self.is_running = False


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

    def attach_to_audio_queue(self, audio_data: np.ndarray) -> bool:
        try:
            # If queue is full, remove oldest packets
            if self.audio_queue.full():
                try:
                    self.audio_queue.get_nowait()
                except Empty:
                    pass

            self.audio_queue.put_nowait(audio_data)
            return True
        except Full:
            logger.error("Audio queue is full")
            return False
        except Exception as e:
            logger.error(f"Failed to attach audio data to queue: {e}")
            return False

    def _audio_callback(self, outdata, frames, time_info, status):
        """Is called by the soundcard to get the audio to play"""
        if status:
            logger.debug(f"Audio output status: {status}")

        try:
            audio_data = self.audio_queue.get_nowait()

            audio_normalized = audio_data.astype(np.float32) / 2147483648.0
            if self.gain != 1.0:
                audio_normalized *= self.gain

            audio_normalized = np.clip(audio_normalized, -1.0, 0.999999)

            outdata[:] = audio_normalized
        except Empty:
            # No data available -> send 0
            outdata.fill(0.0)
        except Exception as e:
            outdata.fill(0.0)
            logger.error(f"Callback error: {e}")

    def stop(self):
        self.is_running = False
        if self.output_stream:
            self.output_stream.stop()
            self.output_stream.close()
            logger.info("Audio stream closed")