use std::sync::{Arc, Mutex};

use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;
use ringbuf::traits::*;

use crate::core::act::audio::aec::AecHandler;

pub fn playback(
    aec_handler: Arc<Mutex<AecHandler>>,
    host: &cpal::Host,
    mut output_buffer_cons: impl Consumer<Item = f32> + Send + 'static,
    input_channels: usize,
) -> Result<cpal::Stream, Box<dyn std::error::Error>> {
    let device = host
        .default_output_device()
        .ok_or("No default audio output device found")?;

    let config: cpal::StreamConfig = device
        .default_output_config()
        .map_err(|e| format!("Failed to get default output config: {}", e))?
        .into();

    let out_channels = config.channels as usize;
    println!(
        "Audio Output Device: {} Hz, {} channels",
        config.sample_rate, out_channels
    );

    let mut last_fade = vec![0.0f32; out_channels];

    let mut render_frame_buffer: Vec<Vec<f32>> = vec![vec![0.0; 480]; out_channels];
    let mut sample_count: usize = 0;

    let stream = device.build_output_stream(
        config,
        move |output: &mut [f32], _| {
            let out_frames = output.len() / out_channels;
            let mut out_idx = 0;

            for _ in 0..out_frames {
                let mut frame_samples = [0.0f32; 16];
                let has_frame = output_buffer_cons.occupied_len() >= input_channels;

                if has_frame {
                    for ch in 0..input_channels {
                        frame_samples[ch] = output_buffer_cons.try_pop().unwrap();
                        render_frame_buffer[ch][sample_count] = frame_samples[ch];
                    }
                    for ch in 0..out_channels {
                        let src_ch = ch % input_channels;
                        let sample = frame_samples[src_ch];
                        output[out_idx + ch] = sample;
                        last_fade[ch] = sample;
                    }
                   
                    // From 0..479 = 480 samples
                    if sample_count == 479 {
                        if let Ok(mut aec) = aec_handler.lock() {
                            let _ = aec.process_render_frame(&mut render_frame_buffer);
                        }

                        sample_count = 0;
                    } else if sample_count > 479 {
                        eprintln!("Samplecount out of range. Samplecount: {}", sample_count);
                    } else {
                        sample_count += 1;
                    }

                } else {
                    for ch in 0..out_channels {
                        last_fade[ch] *= 0.95;
                        output[out_idx + ch] = last_fade[ch];
                    }
                }

                out_idx += out_channels;
            }
        },
        |err| eprintln!("Audio playback error: {}", err),
        None,
    )?;

    stream.play()?;

    println!("Playing pristine audio stream. Press Ctrl+C to exit.");
    Ok(stream)
}
