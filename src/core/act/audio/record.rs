use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;
use ringbuf::{HeapProd, traits::*};

pub fn record(
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

    let stream = device.build_input_stream(
        config.clone(),
        move |data: &[f32], _| {
            for sample in data {
                let _ = buffer.try_push(*sample);
            }
        },
        err_fn,
        None,
    )?;

    stream.play()?;
    Ok((stream, config))
}
