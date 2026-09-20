use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink};
use std::fs::File;
use std::path::Path;
use std::sync::Mutex;

static CURRENT_HANDLE: Mutex<Option<MixerDeviceSink>> = Mutex::new(None);

/// Plays a ringtone from disk. Stops any currently playing ringtone first.
pub fn play_ringtone(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    stop_ringtone();

    let handle = DeviceSinkBuilder::open_default_sink()?;
    let file = File::open(path)?;
    let source = Decoder::try_from(file)?;

    handle.mixer().add(source);

    if let Ok(mut guard) = CURRENT_HANDLE.lock() {
        *guard = Some(handle);
    }

    Ok(())
}

/// Instantly stops the currently playing ringtone.
pub fn stop_ringtone() {
    if let Ok(mut guard) = CURRENT_HANDLE.lock() {
        *guard = None;
    }
}
