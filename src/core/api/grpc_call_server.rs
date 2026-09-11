use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

use crate::core::api::proto;

use crate::core::act::call::call_handler::CallHandler;
use crate::core::state::devices::DeviceManager;

use proto::synapsed::api::call::call_helpers_server::CallHelpers;

use proto::synapsed::api::call::GetCallLabelReply;

#[derive(Clone)]
pub struct CallApi {
    call_handler: Arc<Mutex<CallHandler>>,
    device_manager: Arc<Mutex<DeviceManager>>,
}

impl CallApi {
    pub fn new(
        call_handler: Arc<Mutex<CallHandler>>,
        device_manager: Arc<Mutex<DeviceManager>>,
    ) -> Self {
        Self {
            call_handler,
            device_manager,
        }
    }
}

#[tonic::async_trait]
impl CallHelpers for CallApi {
    async fn get_call_label(&self, _: Request<()>) -> Result<Response<GetCallLabelReply>, Status> {
        let call_handler = self
            .call_handler
            .lock()
            .map_err(|_| Status::internal("Failed to lock CallHandler"))?;
        let device_manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Failed to lock DeviceManager"))?;

        let call_device_id = call_handler.get_call_device_id();
        let device_name = device_manager.get_device_name(call_device_id);
        let call_label = call_handler.get_call_label(&device_name);

        Ok(Response::new(GetCallLabelReply { call_label }))
    }
}
