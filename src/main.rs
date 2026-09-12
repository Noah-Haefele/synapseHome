mod core;
mod networking;
mod platform;

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    thread,
};
use tonic::transport::Server;

// --- gRPC services ---
use crate::core::api::proto;
use proto::synapsed::api::{
    call::{
        call_actions_server::CallActionsServer, call_helpers_server::CallHelpersServer,
        call_signals_server::CallSignalsServer,
    },
    pref::{
        pref_call_ids_server::PrefCallIdsServer, pref_icon_paths_server::PrefIconPathsServer,
        pref_models_server::PrefModelsServer, pref_short_names_server::PrefShortNamesServer,
    },
    settings::{
        audio_server::AudioServer, display_server::DisplayServer, system_server::SystemServer,
    },
};

// --- gRPC servers ---
use crate::core::api::{
    audio_settings_service::AudioSettingsService, call_actions_service::CallActionsService,
    call_helpers_service::CallHelpersService, call_signals_service::CallSignalsService,
    display_settings_service::DisplaySettingsService, pref_call_ids_service::PrefCallIdsService,
    pref_icon_paths_service::PrefIconPathsService, pref_model_service::PrefModelService,
    pref_short_names_service::PrefShortNamesService,
    system_settings_service::SystemSettingsService,
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

    // --- Initialize components ---

    let audio_receiver = AudioReceiver::new("0.0.0.0", 5000)?;
    let audio_sender = AudioSender::new("0.0.0.0", 0)?;
    let audio_handler = AudioHandler::new(audio_receiver, audio_sender)?;
    let audio_devices_handler = Mutex::new(AudioDevicesHandler::new()?);

    let net_iface = Arc::new(Mutex::new(NetIface::new()));

    let mqtt_config = MqttConfig::new()?;
    let mqtt_handler = Arc::new(Mutex::new(MqttHandler::new(mqtt_config, event_sender)?));

    let device_manager = Arc::new(Mutex::new(DeviceManager::new()?));

    let display_controller = DspCtrl::new();
    let display_manager = Mutex::new(DisplayManager::new(display_controller)?);

    let location_id = {
        let device_manager = device_manager
            .lock()
            .map_err(|_| "Failed to lock DeviceManager")?;

        device_manager.get_location_id()
    };

    let call_setup = Mutex::new(CallSetup::new(Arc::clone(&mqtt_handler), location_id)?);

    // --- Initialize gRPC services ---
    let system_settings_service = SystemSettingsService::new(
        Arc::clone(&device_manager),
        call_setup,
        Arc::clone(&net_iface),
    );
    let display_settings_service = DisplaySettingsService::new(display_manager);
    let audio_settings_service = AudioSettingsService::new(audio_devices_handler);
    let pref_call_ids_service = PrefCallIdsService::new(Arc::clone(&device_manager));
    let pref_model_service = PrefModelService::new(Arc::clone(&device_manager));
    let pref_icon_paths_service = PrefIconPathsService::new(Arc::clone(&device_manager));
    let pref_short_names_service = PrefShortNamesService::new(Arc::clone(&device_manager));
    let call_signals_service = CallSignalsService::new();
    // Important:
    // CallHandler must be initialized after CallSignalsService because it depends on it,
    // and before CallActionsService / CallHelpersService because they depend on CallHandler.
    let call_handler = Arc::new(Mutex::new(CallHandler::new(
        call_signals_service.clone(),
        mqtt_handler,
        audio_handler,
    )));
    let call_actions_service = CallActionsService::new(
        Arc::clone(&call_handler),
        Arc::clone(&device_manager),
        net_iface,
    );
    let call_helpers_service =
        CallHelpersService::new(Arc::clone(&call_handler), Arc::clone(&device_manager));

    // --- Start MQTT event handler ---
    let mut call_mqtt_event_handler =
        CallEventHandler::new(call_handler, device_manager, event_receiver);
    // Start call_event_handler to listen to events from mqtt_handler
    thread::spawn(move || {
        if let Err(e) = call_mqtt_event_handler.run() {
            eprintln!("CallEventHandler error: {}", e);
        }
    });

    // --- Wrap services as tonic gRPC servers ---
    let system_server = SystemServer::new(system_settings_service);
    let display_server = DisplayServer::new(display_settings_service);
    let audio_server = AudioServer::new(audio_settings_service);
    let pref_call_ids_server = PrefCallIdsServer::new(pref_call_ids_service);
    let pref_models_server = PrefModelsServer::new(pref_model_service);
    let pref_icon_paths_server = PrefIconPathsServer::new(pref_icon_paths_service);
    let pref_short_names_server = PrefShortNamesServer::new(pref_short_names_service);
    let call_signals_server = CallSignalsServer::new(call_signals_service);
    let call_actions_server = CallActionsServer::new(call_actions_service);
    let call_helpers_server = CallHelpersServer::new(call_helpers_service);

    // --- Start gRPC server ---
    let addr: SocketAddr = "0.0.0.0:50051".parse()?;

    Server::builder()
        .add_service(system_server)
        .add_service(display_server)
        .add_service(audio_server)
        .add_service(pref_call_ids_server)
        .add_service(pref_models_server)
        .add_service(pref_icon_paths_server)
        .add_service(pref_short_names_server)
        .add_service(call_signals_server)
        .add_service(call_actions_server)
        .add_service(call_helpers_server)
        .serve(addr)
        .await?;

    Ok(())
}
