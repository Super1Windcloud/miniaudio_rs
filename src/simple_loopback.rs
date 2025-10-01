use crate::miniaudio_rs::*;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;
const MA_SUCCESS: ma_result = ma_result_MA_SUCCESS as ma_result;

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

pub fn record_speaker_audio(output_file: &str) {
    unsafe {
        let backends: [ma_backend; 1] = [ma_backend_ma_backend_wasapi];

        let c_file = CString::new(output_file).unwrap();

        let mut encoder: ma_encoder = std::mem::zeroed();

        let editor_config = ma_encoder_config_init(
            ma_encoding_format_ma_encoding_format_wav,
            ma_format_ma_format_f32,
            2,
            44100,
        );

        let result = ma_encoder_init_file(c_file.as_ptr(), &editor_config, &mut encoder);
        if result != MA_SUCCESS {
            eprintln!("ma_encoder_init_file returned {}", result);
            return;
        }

        let mut device_config = ma_device_config_init(ma_device_type_ma_device_type_loopback);
        device_config.capture.pDeviceID = ptr::null_mut();
        device_config.capture.format = encoder.config.format;
        device_config.capture.channels = encoder.config.channels;
        device_config.sampleRate = encoder.config.sampleRate;
        device_config.dataCallback = Some(capture_callback);
        device_config.pUserData = &mut encoder as *mut _ as *mut c_void;

        let mut device: ma_device = std::mem::zeroed();
        let result = ma_device_init_ex(
            backends.as_ptr(),
            1,
            ptr::null(),
            &device_config,
            &mut device,
        );

        if result != MA_SUCCESS {
            eprintln!("Failed to initialize loopback device.\n");
            return;
        }

        let result = ma_device_start(&mut device);
        if result != MA_SUCCESS {
            eprintln!("Failed to start loopback device.\n");
            ma_device_uninit(&mut device);
            return;
        }

        println!("Press Enter to stop recording...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        ma_device_stop(&mut device);
        ma_device_uninit(&mut device);
        ma_encoder_uninit(&mut encoder);
        return;
    }
}
