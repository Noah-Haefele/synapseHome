use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

use crate::core::api::proto;
use crate::core::state::devices::DeviceManager;

use proto::synapsed::api::pref::pref_icon_paths_server::PrefIconPaths;

// --- Replies ---
use proto::synapsed::api::pref::GetPref1IconPathReply;
use proto::synapsed::api::pref::GetPref2IconPathReply;
use proto::synapsed::api::pref::GetPref3IconPathReply;

pub struct PrefIconPathsService {
    device_manager: Arc<Mutex<DeviceManager>>,
}

impl PrefIconPathsService {
    pub fn new(device_manager: Arc<Mutex<DeviceManager>>) -> Self {
        Self { device_manager }
    }
}

#[tonic::async_trait]
impl PrefIconPaths for PrefIconPathsService {
    async fn get_pref1_icon_path(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetPref1IconPathReply>, Status> {
        let manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        let path = manager.get_pref_icon_path(1);

        Ok(Response::new(GetPref1IconPathReply { path }))
    }

    async fn get_pref2_icon_path(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetPref2IconPathReply>, Status> {
        let manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        let path = manager.get_pref_icon_path(2);

        Ok(Response::new(GetPref2IconPathReply { path }))
    }

    async fn get_pref3_icon_path(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetPref3IconPathReply>, Status> {
        let manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        let path = manager.get_pref_icon_path(3);

        Ok(Response::new(GetPref3IconPathReply { path }))
    }
}
