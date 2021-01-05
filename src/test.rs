extern crate hidapi;
extern crate once_cell;

use super::keyboard;
use super::config;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// we store no data and not hidapi::HidApi in the mutex to avoid posoning error if one of the test fail
static MUTEX_API_TEST: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

#[test]
fn test_hid_api_and_lock() {
    let guard = MUTEX_API_TEST.lock();
    let api = hidapi::HidApi::new().unwrap();
}

#[test]
fn get_keyboard() {
    let guard = MUTEX_API_TEST.lock();
    let api: hidapi::HidApi = hidapi::HidApi::new().unwrap();
    let mut keyboard = keyboard::KeyboardApi::get_api_from_list(&api, &config::get_default_interface_info()).unwrap();
    keyboard.initialise_control_device(&keyboard::ControlerFeatureKind::Rainbow).unwrap();
} 
