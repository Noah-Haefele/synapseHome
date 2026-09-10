use std::sync::Mutex;
use tonic::{Request, Response, Status};

use crate::core::api::proto;
use crate::core::display::brightness::DisplayManager;

use proto::synapsed::api::settings::display_server::Display;

// --- Requests ---
use proto::synapsed::api::settings::SetBrightnessRequest;
use proto::synapsed::api::settings::SetDisplayTimeRequest;

// --- Replies ---
use proto::synapsed::api::settings::GetBrightnessReply;
use proto::synapsed::api::settings::GetDisplayTimeReply;

pub struct DisplaySettingsServer {
    display_manager: Mutex<DisplayManager>,
}

impl DisplaySettingsServer {
    pub fn new(display_manager: Mutex<DisplayManager>) -> Self {
        Self { display_manager }
    }
}

#[tonic::async_trait]
impl Display for DisplaySettingsServer {
    async fn get_brightness(&self, _: Request<()>) -> Result<Response<GetBrightnessReply>, Status> {
        let display_manager = self
            .display_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        let val = display_manager.get_brightness();

        Ok(Response::new(GetBrightnessReply { val }))
    }

    async fn get_display_time(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetDisplayTimeReply>, Status> {
        let display_manager = self
            .display_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        let val = display_manager.get_display_time();

        Ok(Response::new(GetDisplayTimeReply { val }))
    }

    async fn set_brightness(
        &self,
        request: Request<SetBrightnessRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let mut display_manager = self
            .display_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        display_manager
            .set_brightness(req.val)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(()))
    }

    async fn set_display_time(
        &self,
        request: Request<SetDisplayTimeRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let mut display_manager = self
            .display_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        display_manager
            .set_display_time(req.val)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(()))
    }
}
