use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

use crate::core::api::proto;
use crate::core::state::devices::DeviceManager;

use proto::synapsed::api::pref::pref_short_names_server::PrefShortNames;

// --- Replies ---
use proto::synapsed::api::pref::GetPref1ShortNameReply;
use proto::synapsed::api::pref::GetPref2ShortNameReply;
use proto::synapsed::api::pref::GetPref3ShortNameReply;

/// Provides the short names displayed on top of each call icons.
pub struct PrefShortNamesService {
    device_manager: Arc<Mutex<DeviceManager>>,
}

impl PrefShortNamesService {
    pub fn new(device_manager: Arc<Mutex<DeviceManager>>) -> Self {
        Self { device_manager }
    }
}

#[tonic::async_trait]
impl PrefShortNames for PrefShortNamesService {
    async fn get_pref1_short_name(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetPref1ShortNameReply>, Status> {
        let manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        let name = manager.get_device_short_name(1);

        Ok(Response::new(GetPref1ShortNameReply { name }))
    }

    async fn get_pref2_short_name(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetPref2ShortNameReply>, Status> {
        let manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        let name = manager.get_device_short_name(2);

        Ok(Response::new(GetPref2ShortNameReply { name }))
    }

    async fn get_pref3_short_name(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetPref3ShortNameReply>, Status> {
        let manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        let name = manager.get_device_short_name(3);

        Ok(Response::new(GetPref3ShortNameReply { name }))
    }
}
