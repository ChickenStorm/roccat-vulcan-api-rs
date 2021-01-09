use super::config;
use super::config::constants;
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

/// Differents error returned by the API
#[derive(Debug)]
pub enum ErrorRoccatVulcanApi {
    DeviceNotFound,
    NoLedDevice,
    LedDeviceError(hidapi::HidError),
    NoControlDevice,
    ControlDeviceError(hidapi::HidError),
    NoReadDevice,
    ReadDeviceError(hidapi::HidError),
    /// Too much time elapsed while wasting for the device to be ready
    ToMuchTimeWaited(Duration),
    /// error while trying the get the hdiapi.
    HidApiError(hidapi::HidError),
}

/// get the product id of the detected divice
fn get_present_model(product_ids : &std::vec::Vec<u16>, api: &hidapi::HidApi) -> Option<u16> {
    return api.device_list()
        .filter(|device| product_ids.contains(&device.product_id()))
        .map(|device| device.product_id())
        .next()
}

/// filter the intreface
fn get_device_infos<'a>(interface_filter : &'a config::HidInterfaceFilter, api: &'a hidapi::HidApi) -> impl Iterator<Item = &'a hidapi::DeviceInfo> {
    api.device_list().filter(move |device| {
        match interface_filter {
            config::HidInterfaceFilter::Basic(interface_filter_basic) => 
                interface_filter_basic.product_id() == device.product_id() 
                && interface_filter_basic.interface_number() == device.interface_number(),
            config::HidInterfaceFilter::UsagePage(interface_filter_usage_page) =>
                interface_filter_usage_page.product_id() == device.product_id() 
                && interface_filter_usage_page.interface_number() == device.interface_number()
                && interface_filter_usage_page.usage_page() == device.usage_page(),
        }
    })
}

/// verify if the given device info is the correcte one
/// note that it closed the device and therefore reopening the dice might lead to an unvalide state
fn is_correct_control_device_info(device_info: &hidapi::DeviceInfo, api: &hidapi::HidApi) -> bool {
    return match device_info.open_device(api) {
        Ok(device) => {
            is_correct_control_device(&device)
        },
        Err(_) => false,
    }
}

/// verify if the given device is the correct control device.
fn is_correct_control_device(device: &hidapi::HidDevice) -> bool {
    let mut buffer : [u8; 255] = [0x00; 255];
    buffer[0] = 0x0f;
    let a = device.get_feature_report(&mut buffer);
    match a{
        Ok(val) => val > 0,
        Err(err) => {
            false
        },
    }
}

/// Main object to comunicate with the API
pub struct KeyboardApi {
    //api: &'a hidapi::HidApi, // Not sure yet if the structur shoul own api, but I would say no
    read: hidapi::HidDevice,
    control: hidapi::HidDevice,
    led: hidapi::HidDevice,
    has_initialised : bool,
}

pub enum ControlerFeatureKind {
    /// Default rainbow behaviour.
    Rainbow,
    /// This onew look like some strip changing color form blue and green.
    /// this does not work so well, I dont think the hardware was made to support this mode.
    Alternative,
}

///  sleep duration for KeyboardApi::wait_for_control_device.
const WAIT_FOR_CONTROL_DURATION: Duration = Duration::from_millis(1); // if I enter 0 it does not work
/// max time wating for device in KeyboardApi::wait_for_control_device.
const MAX_WAIT_DURATION: Duration = Duration::from_millis(50);

impl KeyboardApi {
    
    /// return the KeyboardApi contructed form hidapi::HidApi and a liste of configuration to look for.
    /// It only consider the first ellement that match the product id
    pub fn get_api_from_interface_hidapi(hidapi: &hidapi::HidApi, interface_infos : &[config::KeyboardIntrefacesInfo]) -> Result<Self, ErrorRoccatVulcanApi> {
        let model_list = interface_infos.iter()
            .map(|element|  element.product_id())
            .collect();
        let model_present = get_present_model(&model_list, hidapi)
            .ok_or(ErrorRoccatVulcanApi::DeviceNotFound)?;
        let interface_info : &config::KeyboardIntrefacesInfo = interface_infos.iter()
            .filter(|element| element.product_id() == model_present)
            .next()
            .unwrap(); // if there is no element here it is a logical error
        let read_hid_info = get_device_infos(interface_info.read_interface(), hidapi)
            .next()
            .ok_or(ErrorRoccatVulcanApi::NoReadDevice)?;
        let led_device = get_device_infos(interface_info.led_interface(), hidapi)
            .next()
            .ok_or(ErrorRoccatVulcanApi::NoLedDevice)?;
        let ctrl_device_list = get_device_infos(interface_info.control_interface(), hidapi);
        let control = ctrl_device_list.map(|device| device.open_device(hidapi))
            .filter(|value| {
                match value {
                    Ok(device) => is_correct_control_device(&device),
                    Err(_) => false,
                }
            })
            .next()
            .ok_or(ErrorRoccatVulcanApi::NoControlDevice)?
            .or(Err(ErrorRoccatVulcanApi::NoControlDevice))?;
        let read = read_hid_info.open_device(hidapi).map_err(|error| ErrorRoccatVulcanApi::ReadDeviceError(error))?;
        let led = read_hid_info.open_device(hidapi).map_err(|error| ErrorRoccatVulcanApi::LedDeviceError(error))?;
        Ok(Self {
            read,
            control,
            led,
            has_initialised: false,
        })
    }
    
    /// get KeyboardApi using a hidapi::HidApi
    pub fn get_api_from_hidapi(hidapi: &hidapi::HidApi) -> Result<Self, ErrorRoccatVulcanApi>{
        return KeyboardApi::get_api_from_interface_hidapi(hidapi, &config::get_default_interface_info());
    }
    
    pub fn get_api_from_list(interface_infos : &[config::KeyboardIntrefacesInfo]) -> Result<Self, ErrorRoccatVulcanApi>{
        let hidapi = hidapi::HidApi::new().map_err(|error| ErrorRoccatVulcanApi::HidApiError(error))?;
        return KeyboardApi::get_api_from_interface_hidapi(&hidapi, &config::get_default_interface_info());
    }
    
    /// Default way to get KeyboardApi. If you want to use hidapi::HidApi you should use KeyboardApi::get_api_from_hidapi
    /// as multiple hidapi::HidApi cannot coexist at the same time
    pub fn get_api() -> Result<Self, ErrorRoccatVulcanApi>{
        let hidapi = hidapi::HidApi::new().map_err(|error| ErrorRoccatVulcanApi::HidApiError(error))?;
        return KeyboardApi::get_api_from_hidapi(&hidapi);
    }
    
    /// Return if the control device of the keyboard has been initialised
    pub fn has_initialised(&self) -> bool{
        return self.has_initialised;
    }
    
    /// Initialise the control device for the default color behaviour.
    /// The return Ok(false) if the device has already been initialised
    /// and Ok(true) if the device has been correctly initialised.
    pub fn initialise_control_device(&mut self, kind: &ControlerFeatureKind) -> Result<bool, ErrorRoccatVulcanApi> {
        if self.has_initialised() {
            return Ok(false);
        }
        let feature_reports = {
            match kind{
                Rainbow => &constants::FEATURE_REPORT,
                Alternative => &constants::FEATURE_REPORT_ALT,
            }
        };
        for feature_report in feature_reports.iter() {
            self.control.send_feature_report(feature_report)
                .map_err(|error| ErrorRoccatVulcanApi::ControlDeviceError(error))?;
            self.wait_for_control_device()?;
        }
        self.has_initialised = true;
        return Ok(true);
    }
    
    /// Wait for the control device to be ready.
    /// It is unclear if the sleep is enought or the verification on the get_feature report is necessary.
    fn wait_for_control_device(&self) -> Result<(), ErrorRoccatVulcanApi> {
        let mut control_device_ready = false;
        let now = Instant::now();
        while !control_device_ready {
            sleep(WAIT_FOR_CONTROL_DURATION);
            // the get feature report might be unecessary
            let mut buffer : [u8; 255] = [0x00; 255];
            buffer[0] = 0x04;
            let size = self.control.get_feature_report(&mut buffer);
            match size {
                Ok(val) => control_device_ready = val > 0,
                Err(err) => (),
            };
            if now.elapsed() > MAX_WAIT_DURATION {
                return Err(ErrorRoccatVulcanApi::ToMuchTimeWaited(now.elapsed()));
            }
        }
        return Ok(());
    }
}
