extern crate hidapi;
extern crate once_cell;

use super::{
    keyboard,
    layout,
    config,
    layout::layout_fr_ch::LayoutFrCh,
    layout::Layout,
    color::{
        ColorRgb,
        ColorBuffer
    },
    constants,
};
use std::sync::Mutex;
use once_cell::sync::Lazy;

// we store no data and not hidapi::HidApi in the mutex to avoid posoning error if one of the test fail
static MUTEX_API_TEST: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));


/*#[test]
fn test_hid_api_and_lock() {
    let guard = MUTEX_API_TEST.lock();
    let api = hidapi::HidApi::new().unwrap();
}

#[test]
fn get_keyboard() {
    let guard = MUTEX_API_TEST.lock();
    let api: hidapi::HidApi = hidapi::HidApi::new().unwrap();
    let mut keyboard = keyboard::KeyboardApi::get_api_from_interface_hidapi(&api, &config::get_default_interface_info()).unwrap();
    keyboard.initialise_control_device(&keyboard::ControlerFeatureKind::Rainbow).unwrap();
} 
*/

#[test]
fn search_key_layout_ch_fr() {
    let layout = LayoutFrCh::new();
    for key_info in layout.layout_key_info().iter() {
        assert_eq!(layout.find_key_info_from_light(key_info.key_code_light()), Some(key_info));
        assert_eq!(layout.find_key_info_from_press_code(key_info.key_code_press()), Some(key_info));
        assert_eq!(layout.find_key_info_from_key(key_info.key()), Some(key_info));
        assert_eq!(layout.find_key_info_from_string(key_info.key_string()), Some(key_info));
    }
}

#[test]
fn test_led_buffer_length () {
    assert_eq!(constants::NUMBER_KEY_LED_BUFFER % constants::KEY_PACKET_SIZE, 0)
}

#[test]
fn test_color_buffer() {
    let buffer = ColorBuffer::<ColorRgb>::new(ColorRgb::new(255,255,255));
    let raw = buffer.get_led_buffer();
    let bite_to_write = (constants::BITE_PACKET_SIZE + 1);
    for i in 0..(raw.len() / bite_to_write){
        assert_eq!(raw[ i * bite_to_write], 0)
    }
    
}
