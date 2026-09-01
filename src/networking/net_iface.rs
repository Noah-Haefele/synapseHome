use local_ip_address::{Error, local_ip};

pub struct NetIface;

impl NetIface {
    pub fn new() -> Self {
        Self
    }

    pub fn get_ip_address(&self) -> Result<String, Error> {
        Ok(local_ip()?.to_string())
    }
}
