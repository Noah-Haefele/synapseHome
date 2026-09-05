use std::net::UdpSocket;

pub struct AudioSender {
    socket: UdpSocket,
}

impl AudioSender {
    pub fn new(ip_address: &str, port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind(format!("{}:{}", ip_address, port))
            .map_err(|_| "Could not bind to socket address")?;

        Ok(Self { socket })
    }

    pub fn send_audio(
        &self,
        audio_bytes: &[u8],
        ip_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.socket.send_to(audio_bytes, ip_address)?;
        Ok(())
    }
}
