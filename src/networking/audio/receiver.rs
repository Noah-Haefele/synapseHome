use std::net::{SocketAddr, UdpSocket};

pub struct AudioReceiver {
    socket: UdpSocket,
}

impl AudioReceiver {
    pub fn new(ip_address: &str, port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind(format!("{}:{}", ip_address, port))?;

        Ok(Self { socket })
    }

    pub fn receive_audio(
        &self,
        mut buffer: &mut [u8],
    ) -> Result<(usize, SocketAddr), Box<dyn std::error::Error>> {
        Ok(self.socket.recv_from(&mut buffer)?)
    }
}
