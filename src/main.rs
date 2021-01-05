extern crate hidapi;
use roccat_vulcan_api_rs::keyboard;
use roccat_vulcan_api_rs::config;
use roccat_vulcan_api_rs::keyboard::KeyboardApi;

use std::time::{Duration, Instant};

fn test_time (api: &hidapi::HidApi) {
    let now = Instant::now();
    let device = api.device_list().next().unwrap();
    for _i in 0..10000{
        device.open_device(&api);
    }
    println!("{}", now.elapsed().as_millis());
}

fn main() {
    let api = hidapi::HidApi::new().unwrap();
    for device in api.device_list() {
        let product_id_list = roccat_vulcan_api_rs::config::get_products_id_default();
        if product_id_list.contains(&(device.product_id())){
            println!{"vendor : {}, usage page : {}, usage : {}, intreface : {}", device.vendor_id(), device.usage_page(), device.usage(), device.interface_number()}
        }
    }
    let mut keyboard = KeyboardApi::get_api_from_hidapi(&api).unwrap();
    //let result = keyboard.initialise_control_device(&keyboard::ControlerFeatureKind::Rainbow);
    let result = keyboard.initialise_control_device(&keyboard::ControlerFeatureKind::Alternative);
    match result{
        Ok(_) => println!("ok"),
        Err(err) => println!("{:?}", err),
    }
    //test_time(&api);
}
