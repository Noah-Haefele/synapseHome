use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

use crate::core::act::call::call_setup::CallSetup;
use crate::core::api::proto;
use crate::core::state::devices::DeviceManager;
use crate::networking::net_iface::NetIface;

use proto::synapsed::api::settings::system_server::System;

// --- Helper (data formats) ---
use proto::synapsed::api::helper::DeviceData as ProtoDeviceData;

// --- Requests ---
use proto::synapsed::api::settings::SetLocationIdRequest;

// --- Replies ---
use proto::synapsed::api::settings::GetAllDevicesReply;
use proto::synapsed::api::settings::GetIpAddressReply;
use proto::synapsed::api::settings::GetLocationIdReply;

pub struct SystemSettingsServer {
    device_manager: Arc<Mutex<DeviceManager>>,
    call_setup: Mutex<CallSetup>,
    net_iface: Arc<Mutex<NetIface>>,
}

impl SystemSettingsServer {
    pub fn new(
        device_manager: Arc<Mutex<DeviceManager>>,
        call_setup: Mutex<CallSetup>,
        net_iface: Arc<Mutex<NetIface>>,
    ) -> Self {
        Self {
            device_manager,
            call_setup,
            net_iface,
        }
    }
}

#[tonic::async_trait]
impl System for SystemSettingsServer {
    async fn get_all_devices(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetAllDevicesReply>, Status> {
        let manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Failed to lock DeviceManager"))?;

        let devices = manager
            .get_all_devices()
            .into_iter()
            .map(|device| ProtoDeviceData {
                device_name: device.device_name,
                device_short_name: device.device_short_name,
                device_id: device.device_id,
            })
            .collect();

        Ok(Response::new(GetAllDevicesReply { devices }))
    }

    async fn get_location_id(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetLocationIdReply>, Status> {
        let manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Failed to lock DeviceManager"))?;

        let device_id = manager.get_location_id();

        Ok(Response::new(GetLocationIdReply { device_id }))
    }

    async fn set_location_id(
        &self,
        request: Request<SetLocationIdRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        {
            let mut manager = self
                .device_manager
                .lock()
                .map_err(|_| Status::internal("Failed to lock DeviceManager"))?;

            manager
                .set_location_id(req.device_id)
                .map_err(|e| Status::internal(e.to_string()))?;
        }

        let mut call_setup = self
            .call_setup
            .lock()
            .map_err(|_| Status::internal("Failed to lock CallSetup"))?;

        call_setup
            .subscribe_to_call_message(req.device_id)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(()))
    }

    async fn get_ip_address(&self, _: Request<()>) -> Result<Response<GetIpAddressReply>, Status> {
        let net_iface = self
            .net_iface
            .lock()
            .map_err(|_| Status::internal("Failed to lock NetIface"))?;

        let ip_address = net_iface
            .get_ip_address()
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetIpAddressReply { ip_address }))
    }
}
