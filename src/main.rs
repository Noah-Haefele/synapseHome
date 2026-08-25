mod core;
mod networking;
mod platform;

//use std::thread;
//use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;
use tonic::transport::Server;

use crate::core::api::grpc_server::ThisSystem;
use crate::core::display::brightness::DisplayManager;
use crate::core::state::devices::DeviceManager;

use crate::core::api::grpc_server::synapsed::settings::api::display_server::DisplayServer;
use crate::core::api::grpc_server::synapsed::settings::api::system_server::SystemServer;

use crate::platform::linux::display_controller::DspCtrl;

use crate::networking::mqtt::mqtt_config::MqttConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _mqtt_setup = MqttConfig::new()?;

    let device_manager = Arc::new(Mutex::new(DeviceManager::new()?));
    let display_controller = Arc::new(Mutex::new(DspCtrl::new()));
    let display_manager = Arc::new(Mutex::new(DisplayManager::new(display_controller)?));

    let addr = "0.0.0.0:50051".parse()?;

    let grpc_server = ThisSystem::new(device_manager, display_manager);

    let system_service = SystemServer::new(grpc_server.clone());
    let display_service = DisplayServer::new(grpc_server);

    Server::builder()
        .add_service(system_service)
        .add_service(display_service)
        .serve(addr)
        .await?;

    Ok(())

    // Just for debuging now
    //loop {
    //    thread::sleep(Duration::from_secs(5));
    //    let _ = config_manager.save_config();
    //    let _ = config_manager.save_internal_settings();
    //    println!("Data saved");
    //}
}
