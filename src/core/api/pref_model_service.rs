use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

use crate::core::api::proto;
use crate::core::state::devices::DeviceManager;

use proto::synapsed::api::pref::pref_models_server::PrefModels;

// --- Helper (data formats) ---
use proto::synapsed::api::helper::DeviceData as ProtoDeviceData;

// --- Replies ---
use proto::synapsed::api::pref::GetPrefModelReply;

pub struct PrefModelService {
    device_manager: Arc<Mutex<DeviceManager>>,
}

impl PrefModelService {
    pub fn new(device_manager: Arc<Mutex<DeviceManager>>) -> Self {
        Self { device_manager }
    }
}

#[tonic::async_trait]
impl PrefModels for PrefModelService {
    async fn get_pref_model(&self, _: Request<()>) -> Result<Response<GetPrefModelReply>, Status> {
        let manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        let devices = manager
            .get_pref_model()
            .into_iter()
            .map(|device| ProtoDeviceData {
                device_name: device.device_name,
                device_short_name: device.device_short_name,
                device_id: device.device_id,
            })
            .collect();

        Ok(Response::new(GetPrefModelReply { devices }))
    }
}
