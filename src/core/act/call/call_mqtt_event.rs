pub enum CallEvent {
    Calling {
        source_device_id: i32,
        source_ip_address: String,
    },
    Accepted {
        source_device_id: i32,
        source_ip_address: String,
    },
    End {
        source_device_id: i32,
        source_ip_address: String,
    },
}
