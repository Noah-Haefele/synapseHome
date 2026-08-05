import logging
import socket
import threading
import time

from src.core.act.audio.record import RecordHandler

logger = logging.getLogger(__name__)


class AudioSender:
    def __init__(
            self,
            record_handler: RecordHandler,
            host="0.0.0.0",
            port=5005
    ):
        self._record_handler = record_handler
        
        self.host = host
        self.port = port

        self.is_running = False

        self.DTYPE = self._record_handler.DTYPE

        self.socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.socket.setsockopt(socket.SOL_SOCKET, socket.SO_SNDBUF, 65536)

    def start(self, target_ip, target_port=5005):
        if self.is_running:
            return

        self.target_ip = target_ip
        self.target_port = target_port

        self.is_running = True

        self._record_handler.start()

        record_thread = threading.Thread(target=self._record_handler.record_audio, daemon=True)
        send_thread = threading.Thread(target=self._send_audio, daemon=True)

        record_thread.start()
        send_thread.start()

    def _send_audio(self):
        """ Sends audio via UDP """
        while self.is_running:
            try:
                self.audio_data = self._record_handler.get_audio_queue()

                if self.audio_data is None:
                    continue

                self.audio_bytes = self.audio_data.astype(self.DTYPE).tobytes()
                self.socket.sendto(self.audio_bytes, (self.target_ip, self.target_port))
            except BlockingIOError:
                time.sleep(0.001)
            except Exception as e:
                if self.is_running:
                    logger.error(f"Error while receiving {e}")
                break

    def stop(self):
        self.is_running = False
        self._record_handler.stop()
        logger.info("Stopped audio sender")

    def shutdown(self):
        self.stop()

        try:
            self.socket.close()
        except OSError:
            pass

        logger.info("Audio sender shut down")