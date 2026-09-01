use rumqttc::{Client, Connection, Event, MqttOptions, Packet, QoS};
use std::{
    collections::HashSet,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
    },
    thread,
    time::Duration,
};

use crate::networking::mqtt::mqtt_config::MqttConfig;
use crate::{core::act::call::call_mqtt_event::CallEvent, networking::mqtt::mqtt_parser};

pub struct MqttHandler {
    mqtt_config: MqttConfig,
    client: Client,
    connection: Option<Connection>,
    running: Arc<AtomicBool>,
    subtopics: HashSet<String>,
    event_sender: Sender<CallEvent>,
}

impl MqttHandler {
    pub fn new(
        mqtt_config: MqttConfig,
        event_sender: Sender<CallEvent>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client_id = &mqtt_config.mqtt_config_cache.client_id;
        let broker_ip = &mqtt_config.mqtt_config_cache.broker_ip;
        let broker_port = mqtt_config.mqtt_config_cache.broker_port;

        let mut mqttoptions = MqttOptions::new(client_id, broker_ip, broker_port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (client, connection) = Client::new(mqttoptions, 10);

        let running = Arc::new(AtomicBool::new(true));

        let subtopics = HashSet::new();

        let mut handler = Self {
            mqtt_config,
            client,
            connection: Some(connection),
            running,
            subtopics,
            event_sender,
        };

        handler.start_client();

        Ok(handler)
    }

    pub fn subscribe(&mut self, subtopic: String) -> Result<(), rumqttc::ClientError> {
        let topic = format!("{}{}", self.mqtt_config.mqtt_topic_prefix, subtopic);

        println!("Subscribed to: {}", topic);
        self.client.subscribe(topic, QoS::AtMostOnce)?;
        self.subtopics.insert(subtopic);

        Ok(())
    }

    pub fn unsubscribe(&mut self, subtopic: String) -> Result<(), rumqttc::ClientError> {
        let topic = format!("{}{}", self.mqtt_config.mqtt_topic_prefix, subtopic);

        println!("Unsubscribed from {}", topic);
        self.client.unsubscribe(topic)?;
        self.subtopics.remove(&subtopic);

        Ok(())
    }

    pub fn get_subtopics(&self) -> Vec<String> {
        self.subtopics.iter().cloned().collect()
    }

    pub fn publish(&self, subtopic: &str, payload: String) -> Result<(), rumqttc::ClientError> {
        let topic = format!("{}{}", self.mqtt_config.mqtt_topic_prefix, subtopic);

        self.client.publish(topic, QoS::AtLeastOnce, false, payload)
    }

    // Starts the incoming event loop in seperate thread
    pub fn start_client(&mut self) {
        let running_thread = Arc::clone(&self.running);
        let event_sender = self.event_sender.clone();

        let mut connection = self.connection.take().unwrap();

        thread::spawn(move || {
            for notification in connection.iter() {
                if running_thread.load(Ordering::Relaxed) == false {
                    break;
                }

                match notification {
                    Ok(Event::Incoming(Packet::Publish(publish))) => {
                        println!("Topic: {}", publish.topic);

                        if let Ok(payload) = String::from_utf8(publish.payload.to_vec()) {
                            if let Some(event) = mqtt_parser::parse(&publish.topic, &payload) {
                                if let Err(e) = event_sender.send(event) {
                                    eprintln!("Failed to send CallEvent: {}", e);
                                }
                            }
                        }
                    }

                    Ok(notification) => {
                        println!("MQTT: {:?}", notification);
                    }
                    Err(e) => {
                        eprintln!("MQTT connection error: {:?}", e);
                        break;
                    }
                }
            }
        });
    }

    pub fn stop_client(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
