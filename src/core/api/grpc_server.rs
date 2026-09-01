use std::sync::Arc;
use std::sync::Mutex;
use tonic::{Request, Response, Status};

use crate::core::act::call::call_setup::CallSetup;
use crate::core::display::brightness::DisplayManager;
use crate::core::state::devices::DeviceManager;

pub mod synapsed {
    pub mod api {
        pub mod settings {
            tonic::include_proto!("synapsed.api.settings");
        }
        pub mod pref {
            tonic::include_proto!("synapsed.api.pref");
        }
        pub mod helper {
            tonic::include_proto!("synapsed.api.helper");
        }
    }
    pub mod pref {}
}

// --- Settings Api ---
use synapsed::api::settings::display_server::Display;
use synapsed::api::settings::system_server::System;

// --- Preference Api ---
use synapsed::api::pref::pref_call_ids_server::PrefCallIds;
use synapsed::api::pref::pref_icon_paths_server::PrefIconPaths;
use synapsed::api::pref::pref_models_server::PrefModels;
use synapsed::api::pref::pref_short_names_server::PrefShortNames;

// --- Data Message ---
use synapsed::api::helper::DeviceData as ProtoDeviceData;

// --- Request Messages ---
// System
use synapsed::api::settings::SetLocationIdRequest;
// Display
use synapsed::api::settings::SetBrightnessRequest;
use synapsed::api::settings::SetDisplayTimeRequest;
// Pref Ids
use synapsed::api::pref::GetPrefCallIdRequest;
use synapsed::api::pref::SetPrefCallIdRequest;

// --- Reply Messages ---
// System
use synapsed::api::settings::GetAllDevicesReply;
use synapsed::api::settings::GetLocationIdReply;
// Display
use synapsed::api::settings::GetBrightnessReply;
use synapsed::api::settings::GetDisplayTimeReply;
// Pref Paths
use synapsed::api::pref::GetPref1IconPathReply;
use synapsed::api::pref::GetPref2IconPathReply;
use synapsed::api::pref::GetPref3IconPathReply;
// Pref Ids
use synapsed::api::pref::GetPrefCallIdReply;
// Pref short Labels
use synapsed::api::pref::GetPref1ShortNameReply;
use synapsed::api::pref::GetPref2ShortNameReply;
use synapsed::api::pref::GetPref3ShortNameReply;
// Pref Models
use synapsed::api::pref::GetPrefModelReply;

/// Handles frontend gRPC requests and forwards them to the device manager.
#[derive(Clone)]
pub struct ThisSystem {
    device_manager: Arc<Mutex<DeviceManager>>,
    display_manager: Arc<Mutex<DisplayManager>>,
    call_setup: Arc<Mutex<CallSetup>>,
}

#[derive(Clone)]
pub struct CallIcons {
    device_manager: Arc<Mutex<DeviceManager>>,
}

impl ThisSystem {
    pub fn new(
        device_manager: Arc<Mutex<DeviceManager>>,
        display_manager: Arc<Mutex<DisplayManager>>,
        call_setup: Arc<Mutex<CallSetup>>,
    ) -> Self {
        Self {
            device_manager,
            display_manager,
            call_setup,
        }
    }
}

impl CallIcons {
    pub fn new(device_manager: Arc<Mutex<DeviceManager>>) -> Self {
        Self { device_manager }
    }
}

#[tonic::async_trait]
impl System for ThisSystem {
    // --- System Settings ---

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

    async fn set_location_id(
        &self,
        request: Request<SetLocationIdRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let mut manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        let mut call_setup = self
            .call_setup
            .lock()
            .map_err(|_| Status::internal("Lock failed"))?;

        manager
            .set_location_id(req.device_id)
            .map_err(|e| Status::internal(e.to_string()))?;

        call_setup
            .subscribe_to_call_message(req.device_id)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(()))
    }
}

#[tonic::async_trait]
impl PrefCallIds for ThisSystem {
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

#[tonic::async_trait]
impl PrefModels for ThisSystem {
    async fn get_pref_model(&self, _: Request<()>) -> Result<Response<GetPrefModelReply>, Status> {
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
}

#[tonic::async_trait]
impl Display for ThisSystem {
    // --- Display Settings ---

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

#[tonic::async_trait]
impl PrefIconPaths for CallIcons {
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
