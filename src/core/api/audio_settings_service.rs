use std::sync::Mutex;
use tonic::{Request, Response, Status};

use crate::core::act::audio::audio_devices_handler::AudioDevicesHandler;
use crate::core::api::proto;

use proto::synapsed::api::settings::audio_server::Audio;

// --- Helper (data formats) ---
use proto::synapsed::api::helper::SinkSourceData as ProtoSinkSourceData;

// --- Requests ---
use proto::synapsed::api::settings::SetSinkSourceRequest;

// --- Replies ---
use proto::synapsed::api::settings::GetDefaultSinkIdReply;
use proto::synapsed::api::settings::GetDefaultSourceIdReply;
use proto::synapsed::api::settings::GetSinkModelReply;
use proto::synapsed::api::settings::GetSourceModelReply;

pub struct AudioSettingsService {
    audio_devices_handler: Mutex<AudioDevicesHandler>,
}

impl AudioSettingsService {
    pub fn new(audio_devices_handler: Mutex<AudioDevicesHandler>) -> Self {
        Self {
            audio_devices_handler,
        }
    }
}

#[tonic::async_trait]
impl Audio for AudioSettingsService {
    async fn get_sink_model(&self, _: Request<()>) -> Result<Response<GetSinkModelReply>, Status> {
        let mut audio_devices_handler = self
            .audio_devices_handler
            .lock()
            .map_err(|_| Status::internal("Failed to lock AudioDeviceHandler"))?;

        let sinks = audio_devices_handler
            .get_sinks()
            .map_err(|e| Status::internal(e.to_string()))?
            .into_iter()
            .map(|(id, name)| ProtoSinkSourceData {
                id: *id,
                name: name.clone(),
            })
            .collect::<Vec<_>>();

        Ok(Response::new(GetSinkModelReply { sinks }))
    }

    async fn get_source_model(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetSourceModelReply>, Status> {
        let mut audio_devices_handler = self
            .audio_devices_handler
            .lock()
            .map_err(|_| Status::internal("Failed to lock AudioDeviceHandler"))?;

        let sources = audio_devices_handler
            .get_sources()
            .map_err(|e| Status::internal(e.to_string()))?
            .into_iter()
            .map(|(id, name)| ProtoSinkSourceData {
                id: *id,
                name: name.clone(),
            })
            .collect::<Vec<_>>();

        Ok(Response::new(GetSourceModelReply { sources }))
    }

    async fn get_default_sink_id(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetDefaultSinkIdReply>, Status> {
        let mut audio_devices_handler = self
            .audio_devices_handler
            .lock()
            .map_err(|_| Status::internal("Failed to lock AudioDevicesHandler"))?;

        let default_sink_id = audio_devices_handler
            .get_default_sink_id()
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetDefaultSinkIdReply {
            id: default_sink_id.unwrap_or(-1),
        }))
    }

    async fn get_default_source_id(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetDefaultSourceIdReply>, Status> {
        let mut audio_devices_handler = self
            .audio_devices_handler
            .lock()
            .map_err(|_| Status::internal("Failed to lock AudioDevicesHandler"))?;

        let default_source_id = audio_devices_handler
            .get_default_source_id()
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetDefaultSourceIdReply {
            id: default_source_id.unwrap_or(-1),
        }))
    }

    async fn set_sink_source(
        &self,
        request: Request<SetSinkSourceRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let audio_devices_handler = self
            .audio_devices_handler
            .lock()
            .map_err(|_| Status::internal("Failed to lcok AudioDeviceHanlder"))?;

        audio_devices_handler
            .set_default(req.id)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(()))
    }
}
