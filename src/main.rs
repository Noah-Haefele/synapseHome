mod core;
mod networking;
mod platform;

use std::{
    sync::{Arc, Mutex},
    thread,
};
use tonic::transport::Server;

// --- gRPC services ---
use crate::core::api::proto;
use proto::synapsed::api::{
    pref::{
        pref_call_ids_server::PrefCallIdsServer, pref_icon_paths_server::PrefIconPathsServer,
        pref_models_server::PrefModelsServer, pref_short_names_server::PrefShortNamesServer,
    },
    settings::{
        audio_server::AudioServer, display_server::DisplayServer, system_server::SystemServer,
    },
};
// --- Call gRPC services ---
use crate::core::api::grpc_call_server::synapsed::api::call::{
    call_actions_server::CallActionsServer, call_helpers_server::CallHelpersServer,
    call_signals_server::CallSignalsServer,
};

// --- gRPC servers ---
use crate::core::api::{
    grpc_call_server::{CallApi, LiveSignalsService},
    grpc_server::{CallIcons, ThisSystem},
};

// --- Core ---
use crate::core::{
    act::{
        audio::{audio::AudioHandler, audio_devices_handler::AudioDevicesHandler},
        call::{
            call_handler::CallHandler, call_mqtt_event_handler::CallEventHandler,
            call_setup::CallSetup,
        },
    },
    display::brightness::DisplayManager,
    state::devices::DeviceManager,
};

// --- gRPC Servers ---
use crate::core::api::system_settings_server::SystemSettingsServer;

// --- Linux ---
use crate::platform::linux::display_controller::DspCtrl;

// --- Networking ---
use crate::networking::{
    audio::{receiver::AudioReceiver, sender::AudioSender},
    mqtt::{mqtt_config::MqttConfig, mqtt_handler::MqttHandler},
    net_iface::NetIface,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (event_sender, event_receiver) = std::sync::mpsc::channel();

    let audio_receiver = AudioReceiver::new("0.0.0.0", 5000)?;
    let audio_sender = AudioSender::new("0.0.0.0", 0)?;
    let audio_handler = AudioHandler::new(audio_receiver, audio_sender)?;
    let audio_devices_handler = Arc::new(Mutex::new(AudioDevicesHandler::new()?));

    let net_iface = Arc::new(Mutex::new(NetIface::new()));
    let mqtt_config = MqttConfig::new()?;
    let mqtt_handler = Arc::new(Mutex::new(MqttHandler::new(mqtt_config, event_sender)?));

    let device_manager = Arc::new(Mutex::new(DeviceManager::new()?));
    let display_controller = DspCtrl::new();
    let display_manager = Arc::new(Mutex::new(DisplayManager::new(display_controller)?));

    let location_id = device_manager
        .lock()
        .map_err(|_| "Device manager lock failed")?
        .get_location_id();

    let call_setup = Mutex::new(CallSetup::new(Arc::clone(&mqtt_handler), location_id)?);

    let addr = "0.0.0.0:50051".parse()?;

    let system_settings_server = SystemSettingsServer::new(
        Arc::clone(&device_manager),
        call_setup,
        Arc::clone(&net_iface),
    );

    let grpc_server = ThisSystem::new(
        Arc::clone(&device_manager),
        display_manager,
        audio_devices_handler,
        Arc::clone(&net_iface),
    );
    let grpc_server_call_icon = CallIcons::new(Arc::clone(&device_manager));
    let grpc_call_signals_server = LiveSignalsService::new();

    let call_handler = Arc::new(Mutex::new(CallHandler::new(
        grpc_call_signals_server.clone(),
        mqtt_handler,
        audio_handler,
    )));
    let mut call_mqtt_event_handler =
        CallEventHandler::new(Arc::clone(&call_handler), event_receiver);
    // Start call_event_handler to listen to events from mqtt_handler
    thread::spawn(move || {
        if let Err(e) = call_mqtt_event_handler.run() {
            eprintln!("CallEventHandler error: {}", e);
        }
    });

    let grpc_call_actions_server = CallApi::new(call_handler, device_manager, net_iface);

    let system_service = SystemServer::new(system_settings_server);
    let display_service = DisplayServer::new(grpc_server.clone());
    let audio_service = AudioServer::new(grpc_server.clone());
    let pref_call_ids_server = PrefCallIdsServer::new(grpc_server.clone());
    let pref_models_server = PrefModelsServer::new(grpc_server);

    let pref_icon_paths = PrefIconPathsServer::new(grpc_server_call_icon.clone());
    let pref_short_names = PrefShortNamesServer::new(grpc_server_call_icon);

    let call_signals_service = CallSignalsServer::new(grpc_call_signals_server);
    let call_actions_service = CallActionsServer::new(grpc_call_actions_server.clone());
    let call_helpers_service = CallHelpersServer::new(grpc_call_actions_server);

    Server::builder()
        .add_service(system_service)
        .add_service(display_service)
        .add_service(audio_service)
        .add_service(pref_call_ids_server)
        .add_service(pref_models_server)
        .add_service(pref_icon_paths)
        .add_service(pref_short_names)
        .add_service(call_signals_service)
        .add_service(call_actions_service)
        .add_service(call_helpers_service)
        .serve(addr)
        .await?;

    Ok(())
}
