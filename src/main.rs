mod core;

//use std::thread;
//use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;
use tonic::transport::Server;

use crate::core::state::devices::DeviceManager;
use crate::core::api::grpc_server::ThisSystem;

use crate::core::api::grpc_server::synapsed::settings::api::system_server::SystemServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let device_manager = DeviceManager::new()?;
    let shared_device_manager = Arc::new(Mutex::new(device_manager));

    let addr = "0.0.0.0:50051".parse()?;
    let grpc_server = ThisSystem::new(shared_device_manager);
    let service = SystemServer::new(grpc_server);

    Server::builder()
        .add_service(service)
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