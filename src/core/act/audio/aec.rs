use webrtc_audio_processing::*;
use webrtc_audio_processing::config::EchoCanceller;

pub struct AecHandler {
    audio_processor: webrtc_audio_processing::Processor,
}

impl AecHandler {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sample_rate = 48_000;
        let audio_processor = Processor::new(sample_rate)?;

        let config = Config { echo_canceller: Some(EchoCanceller::default()), ..Default::default() };
        audio_processor.set_config(config);

        let aec_handler = Self {
            audio_processor,
        };

        Ok(aec_handler)
    }

    pub fn process_render_frame(
        &mut self,
        frame: &mut Vec<Vec<f32>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.audio_processor.process_render_frame(frame)?;
        Ok(())
    }

    pub fn process_capture_frame(
        &mut self,
        frame: &mut Vec<Vec<f32>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.audio_processor.process_capture_frame(frame)?;
        Ok(())
    }
}
