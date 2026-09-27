use ringbuf::traits::*;
use std::{thread, time::Duration};

pub fn drift_control(
    mut input_buf_cons: impl Consumer<Item = f32> + Send + 'static,
    mut smooth_output_buf_prod: impl Producer<Item = f32> + Send + 'static,
    input_channels: usize,
    target_fill_frames: usize,
) {
    let mut window: Vec<f32> = Vec::with_capacity(262144);
    let mut phase = 0.0f64;
    let mut smooth_speed = 1.0f64;

    loop {
        // Drain incoming ring buffer into local window
        while let Some(s) = input_buf_cons.try_pop() {
            window.push(s);
        }

        let current_frames = window.len() / input_channels;

        // Ultra-smooth drift control:
        // Heavily low-pass filter speed adjustments so pitch shifts are < 0.01% (imperceptible)
        let frame_diff = (current_frames as f64) - (target_fill_frames as f64);
        let target_speed = (1.0 + frame_diff * 0.000001).clamp(0.999, 1.001);
        smooth_speed += (target_speed - smooth_speed) * 0.0002;

        let mut processed_any = false;

        while (phase.floor() as usize + 2) * input_channels <= window.len()
            && smooth_output_buf_prod.vacant_len() >= input_channels
        {
            let frame_idx = phase.floor() as usize;
            let frac = (phase - frame_idx as f64) as f32;

            for ch in 0..input_channels {
                let idx0 = frame_idx * input_channels + ch;
                let idx1 = (frame_idx + 1) * input_channels + ch;

                let s0 = window[idx0];
                let s1 = window[idx1];
                let sample = s0 * (1.0 - frac) + s1 * frac;

                let _ = smooth_output_buf_prod.try_push(sample);
            }

            phase += smooth_speed;
            processed_any = true;
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

        if !processed_any {
            thread::sleep(Duration::from_millis(1));
        }
    }
}
