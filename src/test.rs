//! test module

use crate::{color, ColorBuffer, ColorRgb, KeyboardApi, KeyboardIntrefacesFilter};
use hidapi::HidApi;
use once_cell::sync::Lazy;
use std::{
    sync::Mutex,
    thread::sleep,
    time::{Duration, Instant},
};

mod version_number;

/// We can only one HidApi working at once and test are done in parallel.
static MUTEX_API_TEST: Lazy<Mutex<HidApi>> = Lazy::new(|| Mutex::new(HidApi::new().unwrap()));

/// test the mutex for use in tests and getting the HID api
#[test]
fn test_hid_api_and_lock() {
    let _api = MUTEX_API_TEST.lock().unwrap();
}

/// basic render
#[test]
#[cfg(not(feature = "no-keyboard-test"))]
fn get_keyboard() {
    let api = MUTEX_API_TEST.lock().unwrap();
    let keyboard = KeyboardApi::new_from_model_list(
        &api,
        &[
            KeyboardIntrefacesFilter::vulcan_100(),
            KeyboardIntrefacesFilter::vulcan_120(),
        ],
    )
    .unwrap();
    let buffer = ColorBuffer::from_element(ColorRgb::new(0, 255, 255));
    keyboard.render(&buffer).unwrap();
    sleep(Duration::from_secs(1));
}

/// test the definitions of key can be sent in packets

/// test the firt u8 of the color buffer
#[test]
fn test_color_buffer() {
    let buffer = ColorBuffer::<ColorRgb>::from_element(ColorRgb::new(255, 255, 255));
    let raw = buffer.get_led_buffer();
    let bite_to_write = color::BITE_PACKET_SIZE + 1;
    for i in 0..(raw.len() / bite_to_write) {
        assert_eq!(raw[i * bite_to_write], 0);
    }
}

#[test]
#[cfg(not(feature = "no-keyboard-test"))]
fn test_time() {
    let api = MUTEX_API_TEST.lock().unwrap();
    let now = Instant::now();
    let device = api.device_list().next().unwrap();
    for _i in 0_u32..10000_u32 {
        device.open_device(&api).unwrap();
    }
    println!("{}", now.elapsed().as_millis());
}
