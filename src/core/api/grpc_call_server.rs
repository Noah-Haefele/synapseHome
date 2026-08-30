use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub mod synapsed {
    pub mod api {
        pub mod call {
            tonic::include_proto!("synapsed.api.call");
        }
    }
}

use crate::core::act::call::call_handler::CallHandler;
use crate::core::state::devices::DeviceManager;
use crate::networking::net_iface::NetIface;

use synapsed::api::call::call_actions_server::CallActions;
use synapsed::api::call::call_signals_server::CallSignals;

use synapsed::api::call::Event;
use synapsed::api::call::SubscribeRequest;

use synapsed::api::call::InitiateRequest;

#[derive(Debug, Clone)]
pub struct LiveSignalsService {
    event_tx: broadcast::Sender<Event>,
}

pub struct CallApi {
    call_handler: Mutex<CallHandler>,
    device_manager: Arc<Mutex<DeviceManager>>,
    net_iface: Arc<Mutex<NetIface>>,
}

impl CallApi {
    pub fn new(
        call_handler: CallHandler,
        device_manager: Arc<Mutex<DeviceManager>>,
        net_iface: Arc<Mutex<NetIface>>,
    ) -> Self {
        Self {
            call_handler: Mutex::new(call_handler),
            device_manager,
            net_iface,
        }
    }
}

impl LiveSignalsService {
    pub fn new() -> Self {
        let (event_tx, _) = broadcast::channel(50);

        Self { event_tx }
    }

    /// Trigger to notify frontend when value in CallIcon struct changes (e.g. icon path)
    pub fn trigger_call_state_changed(&self, state: &str) {
        let event = Event {
            event_type: "on_call_state_changed".to_string(),
            state: state.to_string(),
        };

        // send event to subscribers
        let _ = self.event_tx.send(event);
    }
}

#[tonic::async_trait]
impl CallSignals for LiveSignalsService {
    type SubscribeStream = ReceiverStream<Result<Event, Status>>;

    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let topic = request.into_inner().topic;

        let mut event_rx = self.event_tx.subscribe();

        let (tx, rx) = mpsc::channel(50);

        tokio::spawn(async move {
            while let Ok(event) = event_rx.recv().await {
                if event.event_type == topic {
                    if tx.send(Ok(event)).await.is_err() {
                        // Disconect
                        break;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tonic::async_trait]
impl CallActions for CallApi {
    async fn initiate(&self, request: Request<InitiateRequest>) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        let mut handler = self
            .call_handler
            .lock()
            .map_err(|_| Status::internal("Failed to lock CallHandler"))?;
        let device_manager = self
            .device_manager
            .lock()
            .map_err(|_| Status::internal("Failed to lock DeviceManager"))?;
        let net_iface = self
            .net_iface
            .lock()
            .map_err(|_| Status::internal("Failed to lock NetIface"))?;

        let location_id = device_manager.get_location_id();
        let ip_address = net_iface
            .get_ip_address()
            .map_err(|e| Status::internal(e.to_string()))?;

        handler
            .initiate_call(req.device_id, location_id, &ip_address)
            .map_err(|e| Status::internal(e.to_string()))?;

        println!("Initiate {}", req.device_id);
        Ok(Response::new(()))
    }

    async fn accept(&self, _: Request<()>) -> Result<Response<()>, Status> {
        let handler = self.call_handler.lock().unwrap();
        handler.accept_call();

        println!("Accept");
        Ok(Response::new(()))
    }

    async fn end(&self, _: Request<()>) -> Result<Response<()>, Status> {
        let mut handler = self.call_handler.lock().unwrap();
        handler.end_call();

        println!("End");
        Ok(Response::new(()))
    }
}
