use crate::miniaudio_rs::{
    ma_device, ma_device_config, ma_device_config_init, ma_device_init, ma_device_start,
    ma_device_type_ma_device_type_capture, ma_device_uninit, ma_encoder, ma_encoder_config,
    ma_encoder_config_init, ma_encoder_init_file, ma_encoder_uninit, ma_encoder_write_pcm_frames,
    ma_encoding_format_ma_encoding_format_wav, ma_format_ma_format_f32, ma_result, ma_uint64,
};
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

unsafe extern "C" fn capture_callback(
    p_device: *mut ma_device,
    _p_output: *mut c_void,
    p_input: *const c_void,
    frame_count: u32,
) {
    unsafe {
        let encoder = (*p_device).pUserData as *mut ma_encoder;
        if encoder.is_null() {
            return;
        }

        // 写入 PCM 数据到 encoder
        ma_encoder_write_pcm_frames(encoder, p_input, frame_count as ma_uint64, ptr::null_mut());
    }
}

pub struct AudioCapture {
    device: ma_device,
    encoder: ma_encoder,
}

impl AudioCapture {
    pub fn new(output_file: &str) -> Result<Self, ma_result> {
        unsafe {
            let c_file = CString::new(output_file).unwrap();

            let encoder_config: ma_encoder_config = ma_encoder_config_init(
                ma_encoding_format_ma_encoding_format_wav,
                ma_format_ma_format_f32,
                2,
                44100,
            );
            let mut encoder: ma_encoder = std::mem::zeroed();

            ma_encoder_init_file(c_file.as_ptr(), &encoder_config, &mut encoder);

            let mut device_config: ma_device_config =
                ma_device_config_init(ma_device_type_ma_device_type_capture);
            device_config.capture.format = encoder.config.format;
            device_config.capture.channels = encoder.config.channels;
            device_config.sampleRate = encoder.config.sampleRate;
            device_config.dataCallback = Some(capture_callback);
            device_config.pUserData = &mut encoder as *mut _ as *mut c_void;

            let mut device: ma_device = std::mem::zeroed();
            ma_device_init(ptr::null_mut(), &device_config, &mut device);

            ma_device_start(&mut device);

            Ok(Self { device, encoder })
        }
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) {
        unsafe {
            ma_device_uninit(&mut self.device);
            ma_encoder_uninit(&mut self.encoder);
        }
    }
}

#[test]
fn test_recorder() {
    let recorder = match AudioCapture::new("output.wav") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to start recorder: {}", { e });
            return;
        }
    };

    println!("Recording... Press Enter to stop");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // AudioCapture drop 后会自动停止设备和关闭文件
}
