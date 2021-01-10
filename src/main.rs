extern crate hidapi;

use roccat_vulcan_api_rs::keyboard::KeyboardApi;
use roccat_vulcan_api_rs::layout::{
    Key,
    Layout,
    layout_fr_ch::LayoutFrCh
};
use roccat_vulcan_api_rs::color::ColorBuffer;
use roccat_vulcan_api_rs::color::ColorRgb;
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

fn main() {
    
    let api = hidapi::HidApi::new().unwrap();
    for device in api.device_list() {
        let product_id_list = roccat_vulcan_api_rs::config::get_products_id_default();
        if product_id_list.contains(&(device.product_id())){
            println!{"vendor : {}, usage page : {}, usage : {}, intreface : {}", device.vendor_id(), device.usage_page(), device.usage(), device.interface_number()}
        }
    }
    
    
    let keyboard = KeyboardApi::get_api_from_hidapi(&api).unwrap();
    
    let mut buffer = ColorBuffer::<ColorRgb>::new(ColorRgb::new(0,255,255));
    let layout = LayoutFrCh::new();
    loop {
        keyboard.render(&buffer).unwrap();
        let result = keyboard.wait_for_key_press();
        if let Ok(val) = result {
            let a = layout.find_key_info_from_press_code(&val.key_code());
            if let Some(key) = a {
                let index_key = *key.key_code_light() as usize;
                if index_key < buffer.buffer().len(){
                    if !val.is_pressed(){
                        if let Key::Escape = key.key(){
                            break;
                        }
                        buffer.buffer_mut()[index_key] = ColorRgb::new(0,255,255)
                    }
                    else {
                        buffer.buffer_mut()[index_key] = ColorRgb::new(0,0,255)
                    }
                }
            }
        }
    }
}
