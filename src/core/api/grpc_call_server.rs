use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

use crate::core::api::proto;

use crate::core::act::call::call_handler::CallHandler;
use crate::core::state::devices::DeviceManager;
use crate::networking::net_iface::NetIface;

use proto::synapsed::api::call::call_actions_server::CallActions;
use proto::synapsed::api::call::call_helpers_server::CallHelpers;

use proto::synapsed::api::call::InitiateRequest;

use proto::synapsed::api::call::GetCallLabelReply;

#[derive(Clone)]
pub struct CallApi {
    call_handler: Arc<Mutex<CallHandler>>,
    device_manager: Arc<Mutex<DeviceManager>>,
    net_iface: Arc<Mutex<NetIface>>,
}

impl CallApi {
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
}

#[tonic::async_trait]
impl CallActions for CallApi {
    async fn initiate(&self, request: Request<InitiateRequest>) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let mut handler = self
            .call_handler
            .lock()
            .map_err(|_| Status::internal("Failed to lock CallHandler"))?;
        let device_manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Failed to lock DeviceManager"))?;
        let net_iface = self
            .net_iface
            .lock()
            .map_err(|_| Status::internal("Failed to lock NetIface"))?;

        let location_id = device_manager.get_location_id();
        let ip_address = net_iface
            .get_ip_address()
            .map_err(|e| Status::internal(e.to_string()))?;

        handler
            .initiate_call(req.device_id, location_id, &ip_address)
            .map_err(|e| Status::internal(e.to_string()))?;

        println!("Initiate {}", req.device_id);
        Ok(Response::new(()))
    }

    async fn accept(&self, _: Request<()>) -> Result<Response<()>, Status> {
        let mut handler = self
            .call_handler
            .lock()
            .map_err(|_| Status::internal("Failed to lock CallHandler"))?;
        let device_manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Failed to lock DeviceManager"))?;
        let net_iface = self
            .net_iface
            .lock()
            .map_err(|_| Status::internal("Failed to lock NetIface"))?;

        let location_id = device_manager.get_location_id();
        let ip_address = net_iface
            .get_ip_address()
            .map_err(|e| Status::internal(e.to_string()))?;

        handler
            .accept_call(location_id, &ip_address)
            .map_err(|e| Status::internal(e.to_string()))?;

        println!("Accept");
        Ok(Response::new(()))
    }

    async fn end(&self, _: Request<()>) -> Result<Response<()>, Status> {
        let mut handler = self
            .call_handler
            .lock()
            .map_err(|_| Status::internal("Failed to lock CallHandler"))?;
        let device_manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Failed to lock DeviceManager"))?;
        let net_iface = self
            .net_iface
            .lock()
            .map_err(|_| Status::internal("Failed to lock NetIface"))?;

        let location_id = device_manager.get_location_id();
        let ip_address = net_iface
            .get_ip_address()
            .map_err(|e| Status::internal(e.to_string()))?;

        handler
            .end_call(location_id, &ip_address)
            .map_err(|e| Status::internal(e.to_string()))?;

        println!("End");
        Ok(Response::new(()))
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
