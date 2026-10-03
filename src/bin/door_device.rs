use std::{
    path::Path,
    sync::{Arc, Mutex},
    thread,
};

use synapsed::core::{
    act::{
        audio::{aec::AecHandler, audio::AudioHandler},
        call::{call_handler::CallHandler, doorstation_handler::DoorstationHandler},
    },
    state::{devices::DeviceManager, ringtone::RingtoneManager},
};

use synapsed::networking::{
    audio::{receiver::AudioReceiver, sender::AudioSender},
    mqtt::{mqtt_config::MqttConfig, mqtt_handler::MqttHandler},
    net_iface::NetIface,
};
use synapsed::platform::linux::gpio::{DoorDeviceConfig, GpioController};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("     Starting Synapse Home Doorstation     ");
    println!("===========================================");

    // --- 1. Load Configurations ---
    let door_config_path = Path::new("internal/door_device.json");
    let door_config = DoorDeviceConfig::load_or_default(door_config_path);

    println!("[Config] Doorstation Location ID: {}", door_config.location_id);
    println!(
        "[Config] GPIO Pins -> Buzzer: {}, Ring: {}, Open: {}, Buzz Duration: {}ms",
        door_config.buzzer_pin,
        door_config.ring_pin,
        door_config.open_pin,
        door_config.buzz_duration_ms
    );

    // --- 2. Initialize Core Managers ---
    let device_manager = Arc::new(Mutex::new(DeviceManager::new()?));
    {
        let mut dm = device_manager.lock().unwrap();
        // Ensure device manager location_id matches the door station config
        let _ = dm.set_location_id(door_config.location_id);
    }

    let (event_sender, event_receiver) = std::sync::mpsc::channel();

    // --- 3. Initialize Audio & Networking ---
    let aec_handler = Arc::new(Mutex::new(AecHandler::new()?));
    let audio_receiver = AudioReceiver::new("0.0.0.0", 5000)?;
    let audio_sender = AudioSender::new("0.0.0.0", 0)?;
    let audio_handler = AudioHandler::new(audio_receiver, audio_sender, aec_handler)?;

    let net_iface = NetIface::new();
    let local_ip = net_iface.get_ip_address().unwrap_or_else(|_| "127.0.0.1".to_string());

    let mqtt_config = MqttConfig::new()?;
    let mqtt_handler = Arc::new(Mutex::new(MqttHandler::new(mqtt_config, event_sender)?));

    let ringtone_manager = Arc::new(Mutex::new(RingtoneManager::new()?));

    let call_handler = Arc::new(Mutex::new(CallHandler::new(
        Arc::clone(&mqtt_handler),
        audio_handler,
        ringtone_manager,
    )));


    // --- 4. Initialize Hardware GPIO ---
    let gpio_controller = Arc::new(GpioController::new(&door_config));

    // --- 5. Bind GPIO Hardware Inputs ---
    let ch_for_ring = Arc::clone(&call_handler);
    let ip_for_ring = local_ip.clone();
    let loc_id_ring = door_config.location_id;

    let gpio_for_open = Arc::clone(&gpio_controller);

    gpio_controller.start_input_listener(
        move || {
            println!("[Hardware] Ring button pressed! Initiating ALL CALL...");
            if let Ok(mut ch) = ch_for_ring.lock() {
                if let Err(e) = ch.initiate_call_all(loc_id_ring, &ip_for_ring) {
                    eprintln!("[Doorstation] Failed to initiate ALL CALL: {}", e);
                }
            }
        },
        move || {
            println!("[Hardware] Local open button pressed! Pulsing buzzer...");
            gpio_for_open.trigger_buzzer();
        },
    );

    // --- 6. Start Doorstation MQTT Event Handler ---
    let mut door_handler = DoorstationHandler::new(
        call_handler,
        device_manager,
        Arc::clone(&gpio_controller),
        event_receiver,
    );

    thread::spawn(move || {
        if let Err(e) = door_handler.run() {
            eprintln!("[DoorstationHandler] Event loop error: {}", e);
        }
    });

    println!("[Doorstation] Initialization complete. Daemon running.");
    println!("Press Ctrl+C to terminate.");

    tokio::signal::ctrl_c().await?;
    println!("\n[Doorstation] Shutting down daemon gracefully.");
    Ok(())
}
