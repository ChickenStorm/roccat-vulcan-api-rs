//! Main module of the API
//!

// todo verify that the device is not loaded.

use super::{
    color::{Color, ColorBuffer},
    config,
    config::constants,
    layout::Keypress,
};
use std::{
    thread::sleep,
    time::{Duration, Instant},
    vec::Vec,
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
    InvalidInput,
}

/// get the product id of the detected divice
fn get_present_model(product_ids: &[u16], api: &hidapi::HidApi) -> Option<u16> {
    Some(
        api.device_list()
            .find(|device| product_ids.contains(&device.product_id()))?
            .product_id(),
    )
}
/// filter the intreface
fn get_device_infos<'a>(
    interface_filter: &'a config::HidInterfaceFilter,
    api: &'a hidapi::HidApi,
) -> impl Iterator<Item = &'a hidapi::DeviceInfo> {
    api.device_list()
        .filter(move |device| match interface_filter {
            config::HidInterfaceFilter::Basic(interface_filter_basic) => {
                interface_filter_basic.product_id() == device.product_id()
                    && interface_filter_basic.interface_number() == device.interface_number()
            }
            config::HidInterfaceFilter::UsagePage(interface_filter_usage_page) => {
                interface_filter_usage_page.product_id() == device.product_id()
                    && interface_filter_usage_page.interface_number() == device.interface_number()
                    && interface_filter_usage_page.usage_page() == device.usage_page()
            }
        })
}

/// Verify if the given device is the correct control device.
/// Note that this change the state of the HidDevice.
/// If you want to close it you will have to resend a similar feature vreport.
fn is_correct_control_device(device: &hidapi::HidDevice) -> bool {
    let mut buffer: [u8; 255] = [0x00; 255];
    buffer[0] = 0x0f;
    let a = device.get_feature_report(&mut buffer);
    match a {
        Ok(val) => val > 0,
        Err(_) => false,
    }
}

/// Kind of feature report
enum ControlerFeatureKind {
    /// Default rainbow behaviour.
    Rainbow,
    /// Mode where the API can send custom configuration.
    /// If this mode would allowed to stay after opening and closing roccar swarm,
    /// it would look like that there is some strips changing color form blue and green.
    Custom,
}

/// Main object for the comunicate with the API.
pub struct KeyboardApi {
    read: hidapi::HidDevice,
    control: hidapi::HidDevice,
    led: hidapi::HidDevice,
}

/// Sleep duration for KeyboardApi::wait_for_control_device.
const WAIT_FOR_CONTROL_DURATION: Duration = Duration::from_millis(1);
/// Max time wating for device in KeyboardApi::wait_for_control_device.
const MAX_WAIT_DURATION: Duration = Duration::from_millis(100);

impl KeyboardApi {
    /// return the KeyboardApi contructed form hidapi::HidApi and a liste of configuration to look for.
    /// It only consider the first ellement that match the product id.
    pub fn get_api_from_interface_hidapi(
        hidapi: &hidapi::HidApi,
        interface_infos: &[config::KeyboardIntrefacesInfo],
    ) -> Result<Self, ErrorRoccatVulcanApi> {
        let model_list = interface_infos
            .iter()
            .map(|element| element.product_id())
            .collect::<Vec<_>>();
        let model_present =
            get_present_model(&model_list, hidapi).ok_or(ErrorRoccatVulcanApi::DeviceNotFound)?;
        let interface_info: &config::KeyboardIntrefacesInfo = interface_infos
            .iter()
            .find(|element| element.product_id() == model_present)
            .unwrap(); // if there is no element here it is a logical error
        let read_hid_info = get_device_infos(interface_info.read_interface(), hidapi)
            .next()
            .ok_or(ErrorRoccatVulcanApi::NoReadDevice)?;
        let led_device = get_device_infos(interface_info.led_interface(), hidapi)
            .next()
            .ok_or(ErrorRoccatVulcanApi::NoLedDevice)?;
        let ctrl_device_list = get_device_infos(interface_info.control_interface(), hidapi);
        let control = ctrl_device_list
            .map(|device| device.open_device(hidapi))
            .find(|value| match value {
                Ok(device) => is_correct_control_device(&device),
                Err(_) => false,
            })
            .ok_or(ErrorRoccatVulcanApi::NoControlDevice)?
            .or(Err(ErrorRoccatVulcanApi::NoControlDevice))?;
        let read = read_hid_info
            .open_device(hidapi)
            .map_err(ErrorRoccatVulcanApi::ReadDeviceError)?;
        let led = led_device
            .open_device(hidapi)
            .map_err(ErrorRoccatVulcanApi::LedDeviceError)?;
        read.set_blocking_mode(true)
            .map_err(ErrorRoccatVulcanApi::ReadDeviceError)?;
        let keyboard = Self { read, control, led };
        keyboard.initialise_control_device(&ControlerFeatureKind::Custom)?;
        sleep(WAIT_FOR_CONTROL_DURATION); // we seelp after initisation just to maje sure the fist render is done properly.
        Ok(keyboard)
    }

    /// Get KeyboardApi using a [`hidapi::HidApi`].
    pub fn get_api_from_hidapi(hidapi: &hidapi::HidApi) -> Result<Self, ErrorRoccatVulcanApi> {
        KeyboardApi::get_api_from_interface_hidapi(hidapi, &config::get_default_interface_info())
    }

    /// Get the api form a liste of interface filter.
    pub fn get_api_from_list(
        interface_infos: &[config::KeyboardIntrefacesInfo],
    ) -> Result<Self, ErrorRoccatVulcanApi> {
        let hidapi = hidapi::HidApi::new().map_err(ErrorRoccatVulcanApi::HidApiError)?;
        KeyboardApi::get_api_from_interface_hidapi(&hidapi, interface_infos)
    }

    /// Default way to get KeyboardApi. If you want to use [`hidapi::HidApi`] you should use [`KeyboardApi::get_api_from_hidapi`].
    /// as multiple [`hidapi::HidApi`] cannot coexist at the same time.
    pub fn get_api() -> Result<Self, ErrorRoccatVulcanApi> {
        let hidapi = hidapi::HidApi::new().map_err(ErrorRoccatVulcanApi::HidApiError)?;
        KeyboardApi::get_api_from_hidapi(&hidapi)
    }

    /// Initialise the control device for the given color behaviour.
    fn initialise_control_device(
        &self,
        kind: &ControlerFeatureKind,
    ) -> Result<(), ErrorRoccatVulcanApi> {
        let feature_reports = {
            match kind {
                ControlerFeatureKind::Rainbow => &constants::FEATURE_REPORT_RAINBOW,
                ControlerFeatureKind::Custom => &constants::FEATURE_REPORT_CUSTOM,
            }
        };
        for feature_report in feature_reports.iter() {
            self.control
                .send_feature_report(feature_report)
                .map_err(ErrorRoccatVulcanApi::ControlDeviceError)?;
            self.wait_for_control_device()?;
        }
        Ok(())
    }

    /// Wait for the control device to be ready.
    /// It is unclear if the sleep is enought or the verification on the get_feature report is necessary.
    fn wait_for_control_device(&self) -> Result<(), ErrorRoccatVulcanApi> {
        let now = Instant::now();
        loop {
            // It seams to me that the sleep is requierd but the time requierd might be aribtarly small.
            sleep(WAIT_FOR_CONTROL_DURATION);
            let mut buffer: [u8; 255] = [0x00; 255];
            buffer[0] = 0x04;
            let size = self.control.get_feature_report(&mut buffer);
            if let Ok(val) = size {
                if val > 0 {
                    break;
                }
            };
            if now.elapsed() > MAX_WAIT_DURATION {
                return Err(ErrorRoccatVulcanApi::ToMuchTimeWaited(now.elapsed()));
            }
        }
        Ok(())
    }

    /// Wait until a key event and return the [`super::Keypress`] associated with it.
    pub fn wait_for_key_press(&self) -> Result<Keypress, ErrorRoccatVulcanApi> {
        listen_key_press(&self.read).map_err(ErrorRoccatVulcanApi::ReadDeviceError)
    }

    /// Render the given [`ColorBuffer`].
    pub fn render(
        &self,
        buffer: &ColorBuffer<impl Color + Copy>,
    ) -> Result<(), ErrorRoccatVulcanApi> {
        let buffer_bite = buffer.get_led_buffer();
        let bite_to_write = constants::BITE_PACKET_SIZE + 1;
        for i in 0..(buffer_bite.len() / bite_to_write) {
            let buffer_write = &buffer_bite[(i * (bite_to_write))..(i + 1) * bite_to_write];
            self.led
                .write(buffer_write)
                .map_err(ErrorRoccatVulcanApi::LedDeviceError)?;
        }
        Ok(())
    }

    /// read key press for a time of at least duration and return a vector of the keypress that occured for this duration.
    pub fn read_key_press(
        &self,
        duration: Duration,
    ) -> Result<Vec<Keypress>, ErrorRoccatVulcanApi> {
        if duration.as_millis() > i32::MAX as u128 {
            return Err(ErrorRoccatVulcanApi::InvalidInput);
        }
        let mut vector_result: Vec<Keypress> = Vec::new();
        let now = Instant::now();
        loop {
            let elapsed = now.elapsed();
            if duration <= elapsed {
                break;
            }
            let mut buffer: [u8; 5] = [0; 5];
            self.read
                .read_timeout(&mut buffer, (duration - elapsed).as_millis() as i32)
                .map_err(ErrorRoccatVulcanApi::ReadDeviceError)?;
            if buffer[2] > 0 {
                vector_result.push(Keypress::new_from_buffer(&buffer));
            }
        }
        Ok(vector_result)
    }
}

impl Drop for KeyboardApi {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        self.initialise_control_device(&ControlerFeatureKind::Rainbow);
    }
}

fn listen_key_press_raw(device: &hidapi::HidDevice) -> Result<[u8; 5], hidapi::HidError> {
    let mut buffer: [u8; 5] = [0; 5];
    device.read(&mut buffer)?;
    Ok(buffer)
}

fn listen_key_press(device: &hidapi::HidDevice) -> Result<Keypress, hidapi::HidError> {
    let buffer = listen_key_press_raw(device)?;
    Ok(Keypress::new_from_buffer(&buffer))
}
