use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;
use ringbuf::traits::*;

pub fn playback(
    host: &cpal::Host,
    mut output_buffer_cons: impl Consumer<Item = f32> + Send + 'static,
    input_channels: usize,
    buffer_capacity: usize,
    target_fill_frames: usize,
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

    let mut window: Vec<f32> = Vec::with_capacity(buffer_capacity);
    let mut phase = 0.0f64;
    let mut smooth_speed = 1.0f64;
    let mut last_fade = vec![0.0f32; out_channels];

    let stream = device.build_output_stream(
        config,
        move |output: &mut [f32], _| {
            // Drain ring buffer into local window
            while let Some(s) = output_buffer_cons.try_pop() {
                window.push(s);
            }

            let current_frames = window.len() / input_channels;

            // Ultra-smooth drift control:
            // Heavily low-pass filter speed adjustments so pitch shifts are < 0.01% (imperceptible)
            let frame_diff = (current_frames as f64) - (target_fill_frames as f64);
            let target_speed = (1.0 + frame_diff * 0.000001).clamp(0.999, 1.001);
            smooth_speed += (target_speed - smooth_speed) * 0.0002;

            let out_frames = output.len() / out_channels;
            let mut out_idx = 0;

            for _ in 0..out_frames {
                let frame_idx = phase.floor() as usize;
                let frac = (phase - frame_idx as f64) as f32;

                let has_samples = (frame_idx + 1) * input_channels < window.len();

                if has_samples {
                    for ch in 0..out_channels {
                        let src_ch = ch % input_channels;
                        let idx0 = frame_idx * input_channels + src_ch;
                        let idx1 = (frame_idx + 1) * input_channels + src_ch;

                        let s0 = window[idx0];
                        let s1 = window[idx1];
                        let sample = s0 * (1.0 - frac) + s1 * frac;

                        output[out_idx + ch] = sample;
                        last_fade[ch] = sample;
                    }
                    phase += smooth_speed;
                } else {
                    // Soft decay underrun protection (fade to 0 instead of pop)
                    for ch in 0..out_channels {
                        last_fade[ch] *= 0.95;
                        output[out_idx + ch] = last_fade[ch];
                    }
                }

                out_idx += out_channels;
            }

            // Advance processing window by consumed frames
            let consumed_frames = phase.floor() as usize;
            if consumed_frames > 0 {
                let consumed_samples = consumed_frames * input_channels;
                if consumed_samples <= window.len() {
                    window.drain(0..consumed_samples);
                } else {
                    window.clear();
                }
                phase -= consumed_frames as f64;
            }
        },
        |err| eprintln!("Audio playback error: {}", err),
        None,
    )?;

    stream.play()?;

    println!("Playing pristine audio stream. Press Ctrl+C to exit.");
    Ok(stream)
}
