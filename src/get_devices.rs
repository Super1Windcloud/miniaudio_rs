use crate::miniaudio_rs::*;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

pub fn get_devices_info() -> Vec<String> {
    unsafe {
        let mut context: ma_context = std::mem::zeroed();

        let res = ma_context_init(ptr::null(), 0, ptr::null(), &mut context);
        if res != ma_result_MA_SUCCESS {
            eprintln!("Failed to initialize context.");
            return vec![];
        }

        let mut p_playback_device_infos: *mut ma_device_info = ptr::null_mut();
        let mut playback_device_count: ma_uint32 = 0;
        let mut p_capture_device_infos: *mut ma_device_info = ptr::null_mut();
        let mut capture_device_count: ma_uint32 = 0;

        let mut devices_result_names = vec![];
        let result = ma_context_get_devices(
            &mut context,
            &mut p_playback_device_infos,
            &mut playback_device_count,
            &mut p_capture_device_infos,
            &mut capture_device_count,
        );

        if result != ma_result_MA_SUCCESS {
            eprintln!("Failed to retrieve device information.");
            ma_context_uninit(&mut context);
            return vec![];
        }

        println!("Playback Devices");
        for i in 0..playback_device_count {
            let dev = *p_playback_device_infos.add(i as usize);
            let name_ptr = dev.name.as_ptr() as *const c_char;

            let cstr = CStr::from_ptr(name_ptr);
            // println!("    {}: {}", i, cstr.to_string_lossy());
            devices_result_names.push(cstr.to_string_lossy().to_string());
        }

        println!("\nCapture Devices");
        for i in 0..capture_device_count {
            let dev = *p_capture_device_infos.add(i as usize);
            let name_ptr = dev.name.as_ptr() as *const c_char;

            let cstr = CStr::from_ptr(name_ptr);
            // println!("    {}: {}", i, cstr.to_string_lossy());
            devices_result_names.push(cstr.to_string_lossy().to_string());
        }

        ma_context_uninit(&mut context);
        devices_result_names
    }
}

#[test]
fn test_get_devices_info() {
    get_devices_info();
}
