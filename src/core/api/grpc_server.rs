use std::sync::Arc;
use std::sync::Mutex;
use tonic::{Request, Response, Status};

use crate::core::set::devices::DeviceManager;

pub mod synapsed {
    pub mod settings {
        pub mod api {
            tonic::include_proto!("synapsed.settings.api");
        }
    }
}

use synapsed::settings::api::system_server::System;

// --- Data Message ---
use synapsed::settings::api::DeviceData as ProtoDeviceData;

// --- Request Messages ---
use synapsed::settings::api::SetLocationIdRequest;
use synapsed::settings::api::SetPrefCallIdRequest;

// --- Reply Messages ---
use synapsed::settings::api::GetAllDevicesReply;
use synapsed::settings::api::GetPrefModelReply;
use synapsed::settings::api::GetLocationIdReply;
use synapsed::settings::api::GetPrefCallId1Reply;
use synapsed::settings::api::GetPrefCallId2Reply;
use synapsed::settings::api::GetPrefCallId3Reply;

#[derive(Debug, Clone)]
pub struct ThisSystem {
    device_manager: Arc<Mutex<DeviceManager>>,
}

impl ThisSystem {
    pub fn new(device_manager: Arc<Mutex<DeviceManager>>) -> Self {
        Self { device_manager }
    }
}

#[tonic::async_trait]
impl System for ThisSystem {
    async fn get_all_devices(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetAllDevicesReply>, Status> {
        let manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

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
            .map_err(|_| Status::internal("Lock failed"))?;

        let device_id = manager.get_location_id();

        Ok(Response::new(GetLocationIdReply { device_id }))
    }

    async fn get_pref_call_id1(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetPrefCallId1Reply>, Status> {
        let manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        let device_id = manager.get_pref_call_id(1);

        Ok(Response::new(GetPrefCallId1Reply { device_id }))
    }

    async fn get_pref_call_id2(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetPrefCallId2Reply>, Status> {
        let manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        let device_id = manager.get_pref_call_id(2);

        Ok(Response::new(GetPrefCallId2Reply { device_id }))
    }

    async fn get_pref_call_id3(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetPrefCallId3Reply>, Status> {
        let manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        let device_id = manager.get_pref_call_id(3);

        Ok(Response::new(GetPrefCallId3Reply { device_id }))
    }

    async fn get_pref_model(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetPrefModelReply>, Status> {
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

    async fn set_location_id(
        &self,
        request: Request<SetLocationIdRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let mut manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        manager
            .set_location_id(req.device_id)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(()))
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
