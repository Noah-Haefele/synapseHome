import logging
import socket
import threading
import numpy as np
import time

from src.core.act.audio.playback import PlaybackHandler

logger = logging.getLogger(__name__)


class AudioReceiver:
    def __init__(
            self, 
            playback_handler: PlaybackHandler,
            host="0.0.0.0", 
            port=5005
        ):
        self._playback_handler = playback_handler

        self.host = host
        self.port = port

        self.is_running = False

        self.DTYPE = self._playback_handler.DTYPE
        self.CHANNELS = self._playback_handler.CHANNELS

        # Initialize socket
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.socket.setsockopt(socket.SOL_SOCKET, socket.SO_RCVBUF, 65536)
        self.socket.bind((self.host, self.port))
        self.socket.setblocking(False)

    def start(self):
        self.is_running = True

        self._playback_handler.start()

        playback_thread = threading.Thread(target=self._playback_handler.playback_audio, daemon=True)
        receive_thread = threading.Thread(target=self._receive_audio, daemon=True)

        playback_thread.start()
        receive_thread.start()

    def _receive_audio(self):
        """Receives audio packets via UDP"""
        while self.is_running:
            try:
                data, _ = self.socket.recvfrom(65536)
                
                if data:
                    # Convert data to numpy array
                    audio_data = np.frombuffer(data, dtype=self.DTYPE)
                    audio_data = audio_data.reshape(-1, self.CHANNELS)
                    
                    # Attach to queue
                    try:
                        self._playback_handler.attachToAudioQueue(audio_data=audio_data)
                    except:
                        pass # Queue full
            
            except BlockingIOError:
                time.sleep(0.001)
            except Exception as e:
                if self.is_running:
                    logger.error(f"Error while receiving {e}")
                break

    def stop(self):
        self.is_running = False
        self._playback_handler.stop()
        logger.info("Stopped audio receiver")

    def shutdown(self):
        self.stop()

        try:
            self.socket.close()
        except OSError:
            pass

        logger.info("Audio receiver shutdown")