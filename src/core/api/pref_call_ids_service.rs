use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

use crate::core::api::proto;
use crate::core::state::devices::DeviceManager;

use proto::synapsed::api::pref::pref_call_ids_server::PrefCallIds;

// --- Requests ---
use proto::synapsed::api::pref::GetPrefCallIdRequest;
use proto::synapsed::api::pref::SetPrefCallIdRequest;

// --- Replies ---
use proto::synapsed::api::pref::GetPrefCallIdReply;

pub struct PrefCallIdsService {
    device_manager: Arc<Mutex<DeviceManager>>,
}

impl PrefCallIdsService {
    pub fn new(device_manager: Arc<Mutex<DeviceManager>>) -> Self {
        Self { device_manager }
    }
}

#[tonic::async_trait]
impl PrefCallIds for PrefCallIdsService {
    async fn get_pref_call_id(
        &self,
        request: Request<GetPrefCallIdRequest>,
    ) -> Result<Response<GetPrefCallIdReply>, Status> {
        let req = request.into_inner();

        let manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        let device_id = manager.get_pref_call_id(req.num);

        Ok(Response::new(GetPrefCallIdReply { device_id }))
    }

    async fn set_pref_call_id(
        &self,
        request: Request<SetPrefCallIdRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let mut manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        manager
            .set_pref_call_id(req.num, req.device_id)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(()))
    }
}
