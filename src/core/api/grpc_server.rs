use std::sync::Arc;
use std::sync::Mutex;
use tonic::{Request, Response, Status};

use crate::core::state::devices::DeviceManager;

// --- Preference Api ---
use crate::core::api::proto::synapsed::api::pref::pref_short_names_server::PrefShortNames;

// --- Reply Messages ---
// Pref short Labels
use crate::core::api::proto::synapsed::api::pref::GetPref1ShortNameReply;
use crate::core::api::proto::synapsed::api::pref::GetPref2ShortNameReply;
use crate::core::api::proto::synapsed::api::pref::GetPref3ShortNameReply;

#[derive(Clone)]
pub struct CallIcons {
    device_manager: Arc<Mutex<DeviceManager>>,
}

impl CallIcons {
    pub fn new(device_manager: Arc<Mutex<DeviceManager>>) -> Self {
        Self { device_manager }
    }
}

#[tonic::async_trait]
impl PrefShortNames for CallIcons {
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
