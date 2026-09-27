use cpal::{Stream, StreamConfig};
use ringbuf::{
    HeapCons, HeapProd, HeapRb,
    traits::{Consumer, Observer, Producer, Split},
};
use std::{
    collections::HashSet,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use crate::networking::audio::receiver::AudioReceiver;
use crate::networking::audio::sender::AudioSender;
use crate::core::act::audio::aec::AecHandler;

use super::drift::drift_control;
use super::playback::playback;
use super::record::record;

pub struct AudioHandler {
    audio_receiver: Option<AudioReceiver>,
    audio_sender: Option<AudioSender>,
    _aec_handler: Arc<Mutex<AecHandler>>,

    output_buffer_prod: Option<HeapProd<f32>>,
    input_buffer_cons: Option<HeapCons<f32>>,

    is_running: Arc<AtomicBool>,

    _playback_stream: Stream,
    _record_stream: Stream,
    record_stream_config: StreamConfig,

    socket_port: u16,
    input_channels: usize,

    send_target_ips: Arc<Mutex<HashSet<String>>>,
}

impl AudioHandler {
    pub fn new(
        audio_receiver: AudioReceiver,
        audio_sender: AudioSender,
        aec_handler: Arc<Mutex<AecHandler>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let host = cpal::default_host();

        let port = 5000;

        let input_channels: usize = 1;

        println!("Starting High-Fidelity Audio Receiver...");
        println!("UDP Port: {}", port);
        println!("Input Channels: {}", input_channels);

        const TARGET_FILL_FRAMES: usize = 7200;
        const BUFFER_CAPACITY: usize = 262144;

        // Queue between UDP receiver and drift control
        let output_buffer = HeapRb::<f32>::new(BUFFER_CAPACITY);
        let (mut output_buffer_prod, output_buffer_cons) = output_buffer.split();
        // Queue between drift control and audio playback
        let smooth_output_buffer = HeapRb::<f32>::new(BUFFER_CAPACITY);
        let (smooth_output_buffer_prod, smooth_output_buffer_cons) = smooth_output_buffer.split();
        // Queue between audio recorder (downmixed mono) and UDP sender
        let input_buffer = HeapRb::<f32>::new(65536);
        let (input_buffer_prod, input_buffer_cons) = input_buffer.split();

        // Pre-fill UDP output_buffer with 150ms silence cushion
        let initial_samples = TARGET_FILL_FRAMES * input_channels;
        for _ in 0..initial_samples {
            let _ = output_buffer_prod.try_push(0.0);
        }

        // Spawn drift control thread
        thread::spawn(move || {
            drift_control(
                output_buffer_cons,
                smooth_output_buffer_prod,
                input_channels,
                TARGET_FILL_FRAMES,
            );
        });

        let is_running = Arc::new(AtomicBool::new(false));

        let _playback_stream = playback(Arc::clone(&aec_handler), &host, smooth_output_buffer_cons, input_channels)?;
        let (_record_stream, record_stream_config) = record(Arc::clone(&aec_handler), &host, input_buffer_prod)?;

        let mut audio_handler = Self {
            audio_receiver: Some(audio_receiver),
            audio_sender: Some(audio_sender),
            _aec_handler: aec_handler,

            output_buffer_prod: Some(output_buffer_prod),
            input_buffer_cons: Some(input_buffer_cons),

            is_running,

            _playback_stream,
            _record_stream,
            record_stream_config,

            socket_port: 5000,
            input_channels,

            send_target_ips: Arc::new(Mutex::new(HashSet::new())),
        };

        audio_handler.net_audio()?;

        Ok(audio_handler)
    }
}

impl AudioHandler {
    fn net_audio(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_running.store(true, Ordering::Relaxed);
        let is_running = Arc::clone(&self.is_running);

        let send_target_ips = Arc::clone(&self.send_target_ips);

        let input_channels = self.input_channels;

        let audio_receiver = self
            .audio_receiver
            .take()
            .ok_or("Audio receiver already started")?;

        let audio_sender = self
            .audio_sender
            .take()
            .ok_or("Audio sender already started")?;

        let mut output_buffer_prod = self
            .output_buffer_prod
            .take()
            .ok_or("Socket receiver already started")?;

        let mut input_buffer_cons = self
            .input_buffer_cons
            .take()
            .ok_or("Socket sender already started")?;

        thread::spawn(move || {
            let mut buffer = [0u8; 65535];
            let mut packet_count = 0u64;

            loop {
                if !is_running.load(Ordering::Relaxed) {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }

                let (size, address) = match audio_receiver.receive_audio(&mut buffer) {
                    Ok(result) => result,

                    Err(err) => {
                        eprintln!("UDP receive error: {}", err);
                        thread::sleep(Duration::from_millis(20));
                        continue;
                    }
                };

                for chunk in buffer[..size].chunks_exact(4) {
                    let bytes = [chunk[0], chunk[1], chunk[2], chunk[3]];
                    let sample = f32::from_le_bytes(bytes);

                    if output_buffer_prod.try_push(sample).is_err() {
                        thread::sleep(Duration::from_millis(20));
                        eprintln!("Audio buffer full");
                    }
                }

                packet_count += 1;
                if packet_count % 1000 == 0 {
                    println!(
                        "Packets received: {} from {}, Buffer fill: {} frames (~{} ms)",
                        packet_count,
                        address,
                        output_buffer_prod.occupied_len() / input_channels,
                        (output_buffer_prod.occupied_len() / input_channels) * 1000 / 48000
                    );
                }
            }
        });

        let sample_rate = self.record_stream_config.sample_rate;
        let socket_port = self.socket_port;
        let is_running = Arc::clone(&self.is_running);

        thread::spawn(move || {
            // Mono filtered mic audio stream (1 channel)
            let chunk_samples = (sample_rate as usize / 100).max(128); // ~10ms per packet
            let mut packet = Vec::with_capacity(chunk_samples * 4);

            loop {
                if !is_running.load(Ordering::Relaxed) {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }

                if input_buffer_cons.occupied_len() < chunk_samples {
                    thread::sleep(Duration::from_millis(2));
                    continue;
                }

                packet.clear();
                for _ in 0..chunk_samples {
                    match input_buffer_cons.try_pop() {
                        Some(sample) => {
                            packet.extend_from_slice(&sample.to_le_bytes());
                        }
                        None => break,
                    }
                }

                if !packet.is_empty() {
                    let targets: Vec<String> = {
                        let targets = send_target_ips.lock().unwrap();
                        targets.iter().cloned().collect()
                    };

                    for ip in targets {
                        let dest = format!("{}:{}", ip, socket_port);

                        if let Err(e) = audio_sender.send_audio(&packet, &dest) {
                            eprintln!("Error while sending to {}: {}", dest, e);
                        }
                    }
                }
            }
        });

        Ok(())
    }

    pub fn pause_net_audio(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_running.store(false, Ordering::Relaxed);

        let mut targets = self
            .send_target_ips
            .lock()
            .map_err(|e| format!("Failed to lock send_target_ips: {}", e))?;

        targets.clear();

        Ok(())
    }

    pub fn start_net_audio(&mut self, target_ip: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Target Ip: {}", target_ip.to_string());

        let mut targets = self
            .send_target_ips
            .lock()
            .map_err(|e| format!("Failed to lock send_target_ips: {}", e))?;

        targets.insert(target_ip.to_string());

        self.is_running.store(true, Ordering::Relaxed);

        Ok(())
    }
}
