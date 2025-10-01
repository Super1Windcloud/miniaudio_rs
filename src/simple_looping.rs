use crate::miniaudio_rs::{
    ma_data_source, ma_data_source_read_pcm_frames, ma_data_source_set_looping, ma_decoder,
    ma_decoder_init_file, ma_decoder_uninit, ma_device, ma_device_config_init, ma_device_init,
    ma_device_start, ma_device_uninit,
};
use crate::{MA_TRUE, ma_device_type_ma_device_type_playback};
use std::ffi::CString;
use std::ptr;

unsafe extern "C" fn data_callback(
    p_device: *mut ma_device,
    p_output: *mut std::ffi::c_void,
    _p_input: *const std::ffi::c_void,
    frame_count: u32,
) {
    unsafe {
        let p_decoder = (unsafe { *p_device }).pUserData;
        if p_decoder.is_null() {
            return;
        }

        // 读取 PCM 帧
        ma_data_source_read_pcm_frames(p_decoder, p_output, frame_count as u64, ptr::null_mut());
    }
}

pub struct Engine {
    device: ma_device,
    decoder: ma_decoder,
}

impl Engine {
    pub fn new(file_path: &str) -> Result<Self, i32> {
        let c_file = CString::new(file_path).unwrap();
        let mut decoder = unsafe { std::mem::zeroed() };
        let mut device_config = unsafe { std::mem::zeroed() };
        let mut device = unsafe { std::mem::zeroed() };

        // 初始化 decoder
        unsafe {
            ma_decoder_init_file(c_file.as_ptr(), ptr::null(), &mut decoder);
        }

        unsafe {
            ma_data_source_set_looping(&mut decoder as *mut _ as *mut ma_data_source, MA_TRUE);
        }

        unsafe {
            device_config = ma_device_config_init(ma_device_type_ma_device_type_playback);
        }
        device_config.playback.format = decoder.outputFormat;
        device_config.playback.channels = decoder.outputChannels;
        device_config.sampleRate = decoder.outputSampleRate;
        device_config.dataCallback = Some(data_callback);
        device_config.pUserData = &mut decoder as *mut _ as *mut _;

        unsafe {
            ma_device_init(ptr::null_mut(), &device_config, &mut device);
        }

        unsafe {
            ma_device_start(&mut device);
        }

        Ok(Self { device, decoder })
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe {
            ma_device_uninit(&mut self.device);
            ma_decoder_uninit(&mut self.decoder);
        }
    }
}
