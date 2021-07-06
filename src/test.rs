//! test module

extern crate hidapi;
extern crate once_cell;

use super::{
    color::{ColorBuffer, ColorRgb},
    config, constants, keyboard,
    layout::{layout_fr_ch::LayoutFrCh, Layout},
};
use once_cell::sync::Lazy;
use std::{
    sync::Mutex,
    thread::sleep,
    time::{Duration, Instant},
};

// we store no data and not hidapi::HidApi in the mutex to avoid posoning error if one of the test fail
static MUTEX_API_TEST: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

/// test the mutex for use in tests and getting the HID api
#[test]
fn test_hid_api_and_lock() {
    let _guard = MUTEX_API_TEST.lock();
    let _api = hidapi::HidApi::new().unwrap();
}

/// basic render
#[test]
fn get_keyboard() {
    let _guard = MUTEX_API_TEST.lock();
    let api: hidapi::HidApi = hidapi::HidApi::new().unwrap();
    let keyboard = keyboard::KeyboardApi::get_api_from_interface_hidapi(
        &api,
        &config::get_default_interface_info(),
    )
    .unwrap();
    let buffer = ColorBuffer::new(ColorRgb::new(0, 255, 255));
    keyboard.render(&buffer).unwrap();
    sleep(Duration::from_secs(1));
}

/// layout search function
#[test]
fn search_key_layout_ch_fr() {
    let layout = LayoutFrCh::new();
    for key_info in layout.layout_key_info().iter() {
        assert_eq!(
            layout.find_key_info_from_light(key_info.key_code_light()),
            Some(key_info)
        );
        assert_eq!(
            layout.find_key_info_from_press_code(key_info.key_code_press()),
            Some(key_info)
        );
        assert_eq!(
            layout.find_key_info_from_key(key_info.key()),
            Some(key_info)
        );
        assert_eq!(
            layout.find_key_info_from_string(key_info.key_string()),
            Some(key_info)
        );
    }
}

/// test the definitions of key can be sent in packets
#[test]
fn test_led_buffer_length() {
    assert_eq!(
        constants::NUMBER_KEY_LED_BUFFER % constants::KEY_PACKET_SIZE,
        0
    )
}

/// test the firt u8 of the color buffer
#[test]
fn test_color_buffer() {
    let buffer = ColorBuffer::<ColorRgb>::new(ColorRgb::new(255, 255, 255));
    let raw = buffer.get_led_buffer();
    let bite_to_write = constants::BITE_PACKET_SIZE + 1;
    for i in 0..(raw.len() / bite_to_write) {
        assert_eq!(raw[i * bite_to_write], 0)
    }
}

#[test]
fn test_time() {
    let _guard = MUTEX_API_TEST.lock();
    let api = hidapi::HidApi::new().unwrap();
    let now = Instant::now();
    let device = api.device_list().next().unwrap();
    for _i in 0..10000 {
        device.open_device(&api).unwrap();
    }
    println!("{}", now.elapsed().as_millis());
}
