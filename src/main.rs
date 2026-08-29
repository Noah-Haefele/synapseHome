mod core;
mod networking;
mod platform;

use std::sync::Arc;
use std::sync::Mutex;
use tonic::transport::Server;

use crate::core::api::grpc_call_server::CallApi;
use crate::core::api::grpc_call_server::LiveSignalsService;
use crate::core::api::grpc_server::CallIcons;
use crate::core::api::grpc_server::ThisSystem;

// --- Preference Api ---
use crate::core::api::grpc_server::synapsed::api::pref::pref_call_ids_server::PrefCallIdsServer;
use crate::core::api::grpc_server::synapsed::api::pref::pref_icon_paths_server::PrefIconPathsServer;
use crate::core::api::grpc_server::synapsed::api::pref::pref_models_server::PrefModelsServer;
use crate::core::api::grpc_server::synapsed::api::pref::pref_short_names_server::PrefShortNamesServer;

// --- Settings Api ---
use crate::core::api::grpc_server::synapsed::api::settings::display_server::DisplayServer;
use crate::core::api::grpc_server::synapsed::api::settings::system_server::SystemServer;

// --- Signals Api ---
use crate::core::api::grpc_call_server::synapsed::api::call::call_signals_server::CallSignalsServer;

// --- Call Api ---
use crate::core::api::grpc_call_server::synapsed::api::call::call_actions_server::CallActionsServer;

use crate::core::display::brightness::DisplayManager;

use crate::core::state::devices::DeviceManager;

use crate::core::act::call::call_handler::CallHandler;
use crate::core::act::call::setup::CallSetup;

use crate::platform::linux::display_controller::DspCtrl;

use crate::networking::mqtt::mqtt_config::MqttConfig;
use crate::networking::mqtt::mqtt_handler::MqttHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mqtt_config = MqttConfig::new()?;
    let mqtt_handler = MqttHandler::new(mqtt_config)?;

    let device_manager = Arc::new(Mutex::new(DeviceManager::new()?));
    let display_controller = Arc::new(Mutex::new(DspCtrl::new()));
    let display_manager = Arc::new(Mutex::new(DisplayManager::new(display_controller)?));

    let _call_setup = CallSetup::new(mqtt_handler, Arc::clone(&device_manager))?;

    let addr = "0.0.0.0:50051".parse()?;

    let grpc_server = ThisSystem::new(Arc::clone(&device_manager), display_manager);
    let grpc_server_call_icon = CallIcons::new(Arc::clone(&device_manager));
    let grpc_call_signals_server = LiveSignalsService::new();

    let call_handler = CallHandler::new(grpc_call_signals_server.clone());

    let grpc_call_actions_server = CallApi::new(call_handler);

    let system_service = SystemServer::new(grpc_server.clone());
    let display_service = DisplayServer::new(grpc_server.clone());
    let pref_call_ids_server = PrefCallIdsServer::new(grpc_server.clone());
    let pref_models_server = PrefModelsServer::new(grpc_server);

    let pref_icon_paths = PrefIconPathsServer::new(grpc_server_call_icon.clone());
    let pref_short_names = PrefShortNamesServer::new(grpc_server_call_icon);

    let call_signals_service = CallSignalsServer::new(grpc_call_signals_server);
    let call_actions_service = CallActionsServer::new(grpc_call_actions_server);

    Server::builder()
        .add_service(system_service)
        .add_service(display_service)
        .add_service(pref_call_ids_server)
        .add_service(pref_models_server)
        .add_service(pref_icon_paths)
        .add_service(pref_short_names)
        .add_service(call_signals_service)
        .add_service(call_actions_service)
        .serve(addr)
        .await?;

    Ok(())
}
