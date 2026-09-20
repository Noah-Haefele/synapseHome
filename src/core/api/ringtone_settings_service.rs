use std::sync::Mutex;
use tonic::{Request, Response, Status};

use crate::core::api::proto;
use crate::core::state::ringtone::RingtoneManager;

use proto::synapsed::api::settings::ringtone_server::Ringtone;

// --- Helper (data formats) ---
use proto::synapsed::api::helper::RingtoneData as ProtoRingtoneData;

// --- Requests ---
use proto::synapsed::api::settings::SetRingtoneIdRequest;

// --- Replies ---
use proto::synapsed::api::settings::GetRingtoneIdReply;
use proto::synapsed::api::settings::GetRingtoneModelReply;

pub struct RingtoneSettingsService {
    ringtone_manager: Mutex<RingtoneManager>,
}

impl RingtoneSettingsService {
    pub fn new(ringtone_manager: Mutex<RingtoneManager>) -> Self {
        Self { ringtone_manager }
    }
}

#[tonic::async_trait]
impl Ringtone for RingtoneSettingsService {
    async fn get_ringtone_model(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetRingtoneModelReply>, Status> {
        let ringtone_manager = self
            .ringtone_manager
            .lock()
            .map_err(|_| Status::internal("Failed to lock RingtoneManager"))?;

        let ringtones: Vec<ProtoRingtoneData> = ringtone_manager
            .get_ringtone_model()
            .into_iter()
            .map(|item| ProtoRingtoneData {
                id: item.ringtone_id,
                formatted_name: item.ringtone_name,
                path: item.ringtone_path.to_string_lossy().into_owned(),
            })
            .collect();

        Ok(Response::new(GetRingtoneModelReply { ringtones }))
    }

    async fn get_ringtone_id(
        &self,
        _: Request<()>,
    ) -> Result<Response<GetRingtoneIdReply>, Status> {
        let manager = self
            .ringtone_manager
            .lock()
            .map_err(|_| Status::internal("Failed to lock RingtoneManager"))?;

        let id = manager.get_ringtone_id();

        Ok(Response::new(GetRingtoneIdReply { id }))
    }

    async fn set_ringtone_id(
        &self,
        request: Request<SetRingtoneIdRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let mut ringtone_manager = self
            .ringtone_manager
            .lock()
            .map_err(|_| Status::internal("Failed to lock RingtoneManager"))?;

        ringtone_manager
            .set_ringtone_id(req.id)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(()))
    }
}
