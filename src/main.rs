use miniaudio_rs::loopback_to_buffer::record_speaker_real_time;
use miniaudio_rs::simple_loopback::record_speaker_audio;
use std::sync::{Arc, Mutex};

fn output_file() {
    record_speaker_audio("output.wav");
}

fn main() {
    const SAMPLE_RATE: usize = 44100;
    const CHANNELS: usize = 2;
    const BUFFER_SECONDS: usize = 2;

    let buffer: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::with_capacity(
        SAMPLE_RATE * CHANNELS * BUFFER_SECONDS,
    )));
    record_speaker_real_time(buffer);
}
