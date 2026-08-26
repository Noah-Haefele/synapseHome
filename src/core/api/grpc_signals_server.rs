use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub mod synapsed {
    pub mod api {
        pub mod events {
            tonic::include_proto!("synapsed.api.events");
        }
    }
}

use synapsed::api::events::event_service_server::EventService;

use synapsed::api::events::Event;
use synapsed::api::events::SubscribeRequest;

#[derive(Debug, Clone)]
pub struct LiveEventService {
    event_tx: broadcast::Sender<Event>,
}

impl LiveEventService {
    pub fn new() -> Self {
        let (event_tx, _) = broadcast::channel(50);

        Self { event_tx }
    }

    /// Trigger to notify frontend when value in CallIcon struct changes (e.g. icon path)
    pub fn trigger_call_icon_changed(&self) {
        let event = Event {
            event_type: "on_call_icon_changed".to_string(),
        };

        // send event to subscribers
        let _ = self.event_tx.send(event);
    }
}

#[tonic::async_trait]
impl EventService for LiveEventService {
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
