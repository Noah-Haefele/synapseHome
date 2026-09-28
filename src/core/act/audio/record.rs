use std::sync::{Arc, Mutex};

use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;
use ringbuf::{HeapProd, traits::*};

use crate::core::act::audio::aec::AecHandler;

pub fn record(
    aec_handler: Arc<Mutex<AecHandler>>,
    host: &cpal::Host,
    mut buffer: HeapProd<f32>,
) -> Result<(cpal::Stream, cpal::StreamConfig), Box<dyn std::error::Error>> {
    let device = host
        .default_input_device()
        .ok_or("No input source available")?;

    let err_fn = |err: cpal::Error| eprintln!("An error occurred {}", err);

    let config: cpal::StreamConfig = device
        .default_input_config()
        .map_err(|e| format!("No default input audio config: {}", e))?
        .into();

    println!("Record Config: {:?}", config);
    let channels = config.channels as usize;

    let mut capture_frame_buffer: Vec<Vec<f32>> = vec![vec![0.0; 480]; channels];
    let mut sample_count: usize = 0;

    let stream = device.build_input_stream(
        config.clone(),
        move |data: &[f32], _| {
            if channels == 1 {
                for sample in data {
                    capture_frame_buffer[0][sample_count] = *sample;
                    sample_count += 1;

                    if sample_count == 480 {
                        if let Ok(mut aec) = aec_handler.lock() {
                            let _ = aec.process_capture_frame(&mut capture_frame_buffer);
                        }

                        // Push clean audio to the network
                        for i in 0..480 {
                            let _ = buffer.try_push(capture_frame_buffer[0][i]);
                        }

                        sample_count = 0;
                    } else if sample_count > 479 {
                        eprintln!("Samplecount out of range. Samplecount: {}", sample_count);
                    }
                }
            } else {
                for frame in data.chunks_exact(channels) {
                    for ch in 0..channels {
                        capture_frame_buffer[ch][sample_count] = frame[ch];
                    }
                    sample_count += 1;

                    if sample_count == 480 {
                        if let Ok(mut aec) = aec_handler.lock() {
                            let _ = aec.process_capture_frame(&mut capture_frame_buffer);
                        }

                        // Push clean audio to the network
                        for i in 0..480 {
                            let _ = buffer.try_push(capture_frame_buffer[0][i]);
                        }

                        sample_count = 0;
                    } else if sample_count > 479 {
                        eprintln!("Samplecount out of range. Samplecount: {}", sample_count);
                    }
                }
            }
        },
        err_fn,
        None,
    )?;

    stream.play()?;
    Ok((stream, config))
}
