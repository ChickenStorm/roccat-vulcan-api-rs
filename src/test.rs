//! test module

use std::{
    sync::Mutex,
    thread::sleep,
    time::{Duration, Instant},
};

use hidapi::HidApi;
use once_cell::sync::Lazy;

use crate::{
    color, ColorBuffer, ColorRgb, Hue, KeyboardApi, KeyboardIntrefacesFilter, Layout, LayoutFrCh,
    Position, Saturation, Value,
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
    let keyboard =
        KeyboardApi::new_from_model_list(&api, &KeyboardIntrefacesFilter::DEFAULT_MODEL).unwrap();
    let buffer = ColorBuffer::from_element(ColorRgb::new(0, 255, 255));
    keyboard.render(&buffer).unwrap();
    sleep(Duration::from_secs(1));
}

/// Test the color send to the keyboard
#[test]
#[cfg(not(feature = "no-keyboard-test"))]
fn color_cycle() {
    const TIME_WAIT: u64 = 1_000;

    let api = MUTEX_API_TEST.lock().unwrap();
    let keyboard =
        KeyboardApi::new_from_model_list(&api, &KeyboardIntrefacesFilter::DEFAULT_MODEL).unwrap();
    let buffer = ColorBuffer::from_element(ColorRgb::new(255, 0, 0));
    keyboard.render(&buffer).unwrap();
    sleep(Duration::from_millis(TIME_WAIT));
    let buffer = ColorBuffer::from_element(ColorRgb::new(0, 255, 0));
    keyboard.render(&buffer).unwrap();
    sleep(Duration::from_millis(TIME_WAIT));
    let buffer = ColorBuffer::from_element(ColorRgb::new(0, 0, 255));
    keyboard.render(&buffer).unwrap();
    sleep(Duration::from_millis(TIME_WAIT));
    let buffer = ColorBuffer::from_element(ColorRgb::new(255, 255, 255));
    keyboard.render(&buffer).unwrap();
    sleep(Duration::from_millis(TIME_WAIT));
    let buffer = ColorBuffer::from_element(ColorRgb::new(0, 0, 0));
    keyboard.render(&buffer).unwrap();
    sleep(Duration::from_millis(TIME_WAIT));

    const TOTAL_TIME_MILLI: u128 = 10_000;
    let start = Instant::now();
    while let Some(hue) = Hue::new(start.elapsed().as_millis() as f64 / TOTAL_TIME_MILLI as f64) {
        let buffer = ColorBuffer::from_element(ColorRgb::new_hsv(
            hue,
            Saturation::new(1_f64).unwrap(),
            Value::new(1_f64).unwrap(),
        ));
        keyboard.render(&buffer).unwrap();
        sleep(Duration::from_millis(30));
    }
}

/// test the first u8 of the color buffer
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

#[test]
#[cfg(not(feature = "no-keyboard-test"))]
fn pos_cycle() {
    const TIME_WAIT: u64 = 100;

    let api = MUTEX_API_TEST.lock().unwrap();
    let keyboard =
        KeyboardApi::new_from_model_list(&api, &KeyboardIntrefacesFilter::DEFAULT_MODEL).unwrap();
    let buffer = ColorBuffer::from_element(ColorRgb::new(0, 0, 0));
    keyboard.render(&buffer).unwrap();
    let layout = LayoutFrCh::new();
    for x in 0_u32..=22 {
        for y in 0_u32..=5 {
            //keyboard.wait_for_key_press();
            sleep(Duration::from_millis(TIME_WAIT));
            let pos = Position::new(x as f64, y as f64);
            let key = layout.find_closest(pos).unwrap();
            let index_key = (key.key_code_light().code() as usize).min(143);
            let mut buffer = ColorBuffer::from_element(ColorRgb::new(0, 0, 0));

            buffer[index_key] = ColorRgb::new(255, 255, 255);
            keyboard.render(&buffer).unwrap();
        }
    }

    let mut buffer = ColorBuffer::from_element(ColorRgb::new(0, 0, 0));
    for y in 0_u32..=5 {
        for x in 0_u32..=22 {
            //keyboard.wait_for_key_press();
            sleep(Duration::from_millis(TIME_WAIT));
            let pos = Position::new(x as f64, y as f64);
            let key = layout.find_closest(pos).unwrap();
            let index_key = (key.key_code_light().code() as usize).min(143);
            buffer[index_key] = ColorRgb::new(255, 255, 255);
            keyboard.render(&buffer).unwrap();
        }
    }
}
