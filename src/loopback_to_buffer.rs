use crate::miniaudio_rs::{
    ma_backend_ma_backend_wasapi, ma_device, ma_device_config_init, ma_device_init_ex,
    ma_device_start, ma_device_type_ma_device_type_loopback, ma_device_uninit,
    ma_format_ma_format_f32, ma_result_MA_SUCCESS,
};
use std::os::raw::c_void;
use std::ptr;
use std::slice::from_raw_parts;
use std::sync::{Arc, Mutex};

type SharedBuffer = Arc<Mutex<Vec<f32>>>;

unsafe extern "C" fn capture_callback(
    p_device: *mut ma_device,
    _p_output: *mut c_void,
    p_input: *const c_void,
    frame_count: u32,
) {
    if p_input.is_null() {
        return;
    }

    let buffer = &mut *((*p_device).pUserData as *mut SharedBuffer);
    let input_samples = from_raw_parts(p_input as *const f32, frame_count as usize * 2);

    // 可加能量门限判断
    let rms: f32 = input_samples.iter().map(|x| x * x).sum::<f32>() / input_samples.len() as f32;
    if rms.sqrt() > 0.01 {
        let mut buf = buffer.lock().unwrap();
        buf.extend_from_slice(input_samples);
        println!("{:.3} dB", 20.0 * rms.log10());
    }
}

pub fn record_speaker_real_time(buffer: SharedBuffer) {
    unsafe {
        let backends = [ma_backend_ma_backend_wasapi];

        let mut device_config = ma_device_config_init(ma_device_type_ma_device_type_loopback);
        device_config.capture.pDeviceID = ptr::null_mut();
        device_config.capture.format = ma_format_ma_format_f32;
        device_config.capture.channels = 2;
        device_config.sampleRate = 44100;
        device_config.dataCallback = Some(capture_callback);
        device_config.pUserData = &buffer as *const _ as *mut c_void;

        let mut device: ma_device = std::mem::zeroed();
        let result = ma_device_init_ex(
            backends.as_ptr(),
            backends.len() as u32,
            ptr::null_mut(),
            &device_config,
            &mut device,
        );

        if result != ma_result_MA_SUCCESS {
            eprintln!("Failed to init loopback device");
            return;
        }

        if ma_device_start(&mut device) != ma_result_MA_SUCCESS {
            eprintln!("Failed to start device");
            ma_device_uninit(&mut device);
            return;
        }

        println!("Recording... Ctrl+C to stop");

        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // ma_device_stop(&mut device);
        // ma_device_uninit(&mut device);
    }
}
