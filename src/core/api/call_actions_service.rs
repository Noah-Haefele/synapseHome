use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

use crate::core::act::call::call_handler::CallHandler;
use crate::core::api::proto;
use crate::core::state::devices::DeviceManager;
use crate::networking::net_iface::NetIface;

use proto::synapsed::api::call::call_actions_server::CallActions;

// --- Requests ---
use proto::synapsed::api::call::InitiateRequest;

pub struct CallActionsService {
    call_handler: Arc<Mutex<CallHandler>>,
    device_manager: Arc<Mutex<DeviceManager>>,
    net_iface: Arc<Mutex<NetIface>>,
}

impl CallActionsService {
    pub fn new(
        call_handler: Arc<Mutex<CallHandler>>,
        device_manager: Arc<Mutex<DeviceManager>>,
        net_iface: Arc<Mutex<NetIface>>,
    ) -> Self {
        Self {
            call_handler,
            device_manager,
            net_iface,
        }
    }

    fn get_call_context(&self) -> Result<(i32, String), Status> {
        let location_id = {
            let device_manager = self
                .device_manager
                .lock()
                .map_err(|_| Status::internal("Failed to lock DeviceManager"))?;

            device_manager.get_location_id()
        };

        let ip_address = {
            let net_iface = self
                .net_iface
                .lock()
                .map_err(|_| Status::internal("Failed to lock NetIface"))?;

            net_iface
                .get_ip_address()
                .map_err(|e| Status::internal(e.to_string()))?
        };

        Ok((location_id, ip_address))
    }
}

#[tonic::async_trait]
impl CallActions for CallActionsService {
    async fn initiate(&self, request: Request<InitiateRequest>) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let (location_id, ip_address) = self.get_call_context()?;

        let mut handler = self
            .call_handler
            .lock()
            .map_err(|_| Status::internal("Failed to lock CallHandler"))?;

        handler
            .initiate_call(req.device_id, location_id, &ip_address)
            .map_err(|e| Status::internal(e.to_string()))?;

        println!("Initiate {}", req.device_id);
        Ok(Response::new(()))
    }

    async fn all(&self, _: Request<()>) -> Result<Response<()>, Status> {
        let (location_id, ip_address) = self.get_call_context()?;

        let mut handler = self
            .call_handler
            .lock()
            .map_err(|_| Status::internal("Failed to lock CallHandler"))?;

        handler
            .initiate_call_all(location_id, &ip_address)
            .map_err(|e| Status::internal(e.to_string()))?;

        println!("Call to everyone");
        Ok(Response::new(()))
    }

    async fn accept(&self, _: Request<()>) -> Result<Response<()>, Status> {
        let (location_id, ip_address) = self.get_call_context()?;

        let mut handler = self
            .call_handler
            .lock()
            .map_err(|_| Status::internal("Failed to lock CallHandler"))?;

        handler
            .accept_call(location_id, &ip_address)
            .map_err(|e| Status::internal(e.to_string()))?;

        println!("Accept");
        Ok(Response::new(()))
    }

    async fn end(&self, _: Request<()>) -> Result<Response<()>, Status> {
        let (location_id, ip_address) = self.get_call_context()?;

        let mut handler = self
            .call_handler
            .lock()
            .map_err(|_| Status::internal("Failed to lock CallHandler"))?;

        handler
            .end_call(location_id, &ip_address)
            .map_err(|e| Status::internal(e.to_string()))?;

        println!("End");
        Ok(Response::new(()))
    }
}
