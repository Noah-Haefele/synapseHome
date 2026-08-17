import socket

class NetworkInterface():
    def __init__(self):
        self._loopback_fallback = "127.0.0.1"
        self._cached_ip = None

    def fetch_local_ip(self) -> str:
        try:
            s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
            s.connect(("8.8.8.8", 80))
            current_ip = s.getsockname()[0]
            s.close()
            self._cached_ip = current_ip
            return current_ip
        except Exception:
            return self._loopback_fallback
        
    @property
    def ip_address(self) -> str:
        if not self._cached_ip:
            return self.fetch_local_ip()
        return self._cached_ip
