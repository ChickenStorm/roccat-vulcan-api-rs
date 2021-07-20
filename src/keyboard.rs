use crate::{
    color, reports, ColorBuffer, ColorRgb, ErrorRoccatVulcanApi, KeyboardIntrefacesFilter, Keypress,
};
use hidapi::{HidApi, HidDevice};
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::{Duration, Instant};

type Res<T> = Result<T, ErrorRoccatVulcanApi>;

/// Sleep duration for [`KeyboardApi::wait_for_control_device`].
const WAIT_FOR_CONTROL_DURATION: Duration = Duration::from_millis(1);
/// Max time wating for device in [`KeyboardApi::wait_for_control_device`].
const MAX_WAIT_DURATION: Duration = Duration::from_millis(100);

/// Main API
// TODO more doc
pub struct KeyboardApi {
    /// Read device that look for key press
    read: HidDevice,
    /// Control device where feature are send to initialize the keyboard
    control: HidDevice,
    /// Led device which send color for the keyboard
    led: HidDevice,
}

impl KeyboardApi {
    /// Look for the default configuration
    /// # Errors
    /// - [`ErrorRoccatVulcanApi::KeyboardNotFound`] Keyboard not found,
    /// - [`ErrorRoccatVulcanApi::NoLedDevice`] Led device not found,
    /// - [`ErrorRoccatVulcanApi::LedDeviceError`] Led device error,
    /// - [`ErrorRoccatVulcanApi::NoControlDevice`] Control device not found,
    /// - [`ErrorRoccatVulcanApi::ControlDeviceError`] Control device error,
    /// - [`ErrorRoccatVulcanApi::NoReadDevice`] Read device not found,
    /// - [`ErrorRoccatVulcanApi::ReadDeviceError`] Read device error,
    /// - [`ErrorRoccatVulcanApi::WaitedToMuchTime`] Error while initalizing key board: waited for too long,
    /// - [`ErrorRoccatVulcanApi::HidApiError`] Api error,
    pub fn new() -> Res<Self> {
        let api = hidapi::HidApi::new().map_err(ErrorRoccatVulcanApi::HidApiError)?;
        Self::new_from_model_list(
            &api,
            &[
                KeyboardIntrefacesFilter::vulcan_100(),
                KeyboardIntrefacesFilter::vulcan_120(),
            ],
        )
    }

    /// Initialize the API by seraching for a keyboard matching an ellement of a list.
    /// # Errors
    /// see [Self::new]
    pub fn new_from_model_list(
        api: &HidApi,
        interfaces_info: &[KeyboardIntrefacesFilter],
    ) -> Res<Self> {
        for interface in interfaces_info {
            if let Ok(keyboard) = Self::new_model(api, interface) {
                return Ok(keyboard);
            }
        }
        Err(ErrorRoccatVulcanApi::KeyboardNotFound)
    }

    /// Initialize the API uing from a interface info.
    /// # Errors
    /// see [`Self::new`]
    pub fn new_model(api: &HidApi, interface: &KeyboardIntrefacesFilter) -> Res<Self> {
        if !api
            .device_list()
            .any(|device| device.product_id() == interface.control_interface().product_id())
        {
            return Err(ErrorRoccatVulcanApi::KeyboardNotFound);
        }
        let read_info = api
            .device_list()
            .find(|device| interface.read_interface().match_filter(&device))
            .ok_or(ErrorRoccatVulcanApi::NoReadDevice)?;
        let led_info = api
            .device_list()
            .find(|device| interface.led_interface().match_filter(&device))
            .ok_or(ErrorRoccatVulcanApi::NoLedDevice)?;
        let control_info_list = api
            .device_list()
            .filter(|device| interface.control_interface().match_filter(&device));

        let control = control_info_list
            .map(|device| device.open_device(api))
            .find(|value| match value {
                Ok(device) => Self::is_correct_control_device(&device),
                Err(_) => false,
            })
            .ok_or(ErrorRoccatVulcanApi::NoControlDevice)?
            .or(Err(ErrorRoccatVulcanApi::NoControlDevice))?;
        let read = read_info
            .open_device(api)
            .map_err(ErrorRoccatVulcanApi::ReadDeviceError)?;
        let led = led_info
            .open_device(api)
            .map_err(ErrorRoccatVulcanApi::LedDeviceError)?;
        read.set_blocking_mode(true)
            .map_err(ErrorRoccatVulcanApi::ReadDeviceError)?;
        let keyboard = Self { read, control, led };
        keyboard.initialise_control_device(ControlerFeatureKind::Custom)?;
        thread::sleep(WAIT_FOR_CONTROL_DURATION); // we seelp after initisation just to maje sure the fist render is done properly.
        Ok(keyboard)
    }

    /// Verify if the given device is the correct control device.
    /// Note that this change the state of the HidDevice.
    /// If you want to close it you will have to resend a similar feature report.
    fn is_correct_control_device(device: &HidDevice) -> bool {
        let mut buffer: [u8; 255] = [0x00; 255];
        buffer[0] = 0x0f;
        let a = device.get_feature_report(&mut buffer);
        match a {
            Ok(val) => val > 0,
            Err(_) => false,
        }
    }

    /// Initialize the control device with either rainbow mode or custom mode
    fn initialise_control_device(&self, kind: ControlerFeatureKind) -> Res<()> {
        let feature_reports = {
            match kind {
                ControlerFeatureKind::Rainbow => &reports::FEATURE_REPORT_RAINBOW,
                ControlerFeatureKind::Custom => &reports::FEATURE_REPORT_CUSTOM,
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
    /// # Errors
    /// returns [`ErrorRoccatVulcanApi::WaitedToMuchTime`] if we waited too long waiting the control device.
    fn wait_for_control_device(&self) -> Res<()> {
        let now = Instant::now();
        loop {
            // It seams to me that the sleep is requierd but the time requierd might be aribtarly small.
            thread::sleep(WAIT_FOR_CONTROL_DURATION);
            let mut buffer: [u8; 255] = [0x00; 255];
            buffer[0] = 0x04;
            let size = self.control.get_feature_report(&mut buffer);
            if let Ok(val) = size {
                if val > 0 {
                    break;
                }
            };
            if now.elapsed() > MAX_WAIT_DURATION {
                return Err(ErrorRoccatVulcanApi::WaitedToMuchTime(now.elapsed()));
            }
        }
        Ok(())
    }

    /// Renders a collor buffer
    /// # Errors
    /// [`ErrorRoccatVulcanApi::LedDeviceError`] if the lead device encountered an error
    pub fn render(
        &self,
        buffer: &ColorBuffer<impl Into<ColorRgb> + Copy>,
    ) -> Result<(), ErrorRoccatVulcanApi> {
        let buffer_bite = buffer.get_led_buffer();
        let bite_to_write = color::BITE_PACKET_SIZE + 1;
        for i in 0..(buffer_bite.len() / bite_to_write) {
            let buffer_write = &buffer_bite[(i * (bite_to_write))..(i + 1) * bite_to_write];
            self.led
                .write(buffer_write)
                .map_err(ErrorRoccatVulcanApi::LedDeviceError)?;
        }
        Ok(())
    }

    /// read key press for a time of at least duration and return a vector of the keypress that occured for this duration.
    /// # Errors
    /// - [`ErrorRoccatVulcanApi::InvalidInput`] the duration is not valide
    /// - [`ErrorRoccatVulcanApi::ReadDeviceError`] if the read device had an error
    #[allow(clippy::cast_possible_truncation)]
    pub fn read_key_press(&self, duration: Duration) -> Res<Vec<Keypress>> {
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
                vector_result.push(Keypress::new_from_buffer(buffer));
            }
        }
        Ok(vector_result)
    }

    /// Block the thread until a key event or an error occur
    /// # Errors
    /// [`ErrorRoccatVulcanApi::ReadDeviceError`] when the read device has an error
    pub fn wait_for_key_press(&self) -> Res<Keypress> {
        self.listen_key_press()
            .map_err(ErrorRoccatVulcanApi::ReadDeviceError)
    }

    /// wait for key press and rturn the raw value
    fn listen_key_press_raw(&self) -> Result<[u8; 5], hidapi::HidError> {
        let mut buffer: [u8; 5] = [0; 5];
        self.read.read(&mut buffer)?;
        Ok(buffer)
    }

    /// wait for a key perss and return a [`Keypress`]
    fn listen_key_press(&self) -> Result<Keypress, hidapi::HidError> {
        let buffer = self.listen_key_press_raw()?;
        Ok(Keypress::new_from_buffer(buffer))
    }
}

impl Drop for KeyboardApi {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        self.initialise_control_device(ControlerFeatureKind::Rainbow);
    }
}

/// Kind of feature report
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
enum ControlerFeatureKind {
    /// Default rainbow behaviour.
    Rainbow,
    /// Mode where the API can send custom configuration.
    /// If this mode would allowed to stay after opening and closing roccar swarm,
    /// it would look like that there is some strips changing color form blue and green.
    Custom,
}

impl Default for ControlerFeatureKind {
    /// Returns [`ControlerFeatureKind::Custom`]
    fn default() -> Self {
        Self::Custom
    }
}
