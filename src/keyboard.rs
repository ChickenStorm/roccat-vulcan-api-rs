//! main API structure

use std::fmt::{Debug, Display, Formatter};
use std::thread;
use std::time::{Duration, Instant};

use hidapi::{HidApi, HidDevice};
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

use crate::{
    color, reports, ColorBuffer, ColorRgb, ErrorRoccatVulcanApi, KeyPress, KeyboardIntrefacesFilter,
};

mod builder;

pub use builder::*;

/// Result returned by the API
type Res<T> = Result<T, ErrorRoccatVulcanApi>;

/// Sleep duration for [`KeyboardApi::wait_for_control_device`].
const WAIT_FOR_CONTROL_DURATION: Duration = Duration::from_millis(1);
/// Max time waiting for device in [`KeyboardApi::wait_for_control_device`].
const MAX_WAIT_DURATION: Duration = Duration::from_millis(100);

// TODO more doc
/// Main API
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
    /// - [`ErrorRoccatVulcanApi::WaitedToMuchTime`] Error while initializing key board: waited for too long,
    /// - [`ErrorRoccatVulcanApi::HidApiError`] Api error,
    pub fn new() -> Res<Self> {
        let api = hidapi::HidApi::new().map_err(ErrorRoccatVulcanApi::HidApiError)?;
        Self::new_from_model_list(&api, &KeyboardIntrefacesFilter::DEFAULT_MODEL)
    }

    /// Initialize the API by searching for a keyboard matching an element of a list.
    /// # Errors
    /// see [`Self::new`]
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

    /// Initialize the API using from a interface info.
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
            .find(|device| interface.read_interface().match_filter(device))
            .ok_or(ErrorRoccatVulcanApi::NoReadDevice)?;
        let led_info = api
            .device_list()
            .find(|device| interface.led_interface().match_filter(device))
            .ok_or(ErrorRoccatVulcanApi::NoLedDevice)?;
        let control_info_list = api
            .device_list()
            .filter(|device| interface.control_interface().match_filter(device));

        let control = control_info_list
            .map(|device| device.open_device(api))
            .find(|value| match value {
                Ok(device) => Self::is_correct_control_device(device),
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
        thread::sleep(WAIT_FOR_CONTROL_DURATION); // we sleep after initialization just to make sure the fist render is done properly.
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
    /// It is unclear if the sleep is enough or the verification on the get_feature report is necessary.
    /// # Errors
    /// returns [`ErrorRoccatVulcanApi::WaitedToMuchTime`] if we waited too long waiting the control device.
    fn wait_for_control_device(&self) -> Res<()> {
        let now = Instant::now();
        loop {
            // It seams to me that the sleep is required but the time required might be arbitrary small.
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

    /// Renders a color buffer
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

    /// read key press for a time of at least duration and return a vector of the keypress that occurred for this duration.
    /// # Errors
    /// - [`ErrorRoccatVulcanApi::InvalidInput`] the duration is not valid (too big)
    /// - [`ErrorRoccatVulcanApi::ReadDeviceError`] if the read device had an error
    /// # Example
    /// ```
    /// use std::time::Duration;
    ///
    /// use roccat_vulcan_api_rs::{ErrorRoccatVulcanApi, KeyboardApi};
    ///
    /// # fn main() -> Result<(), ErrorRoccatVulcanApi> {
    /// # #[cfg(not(feature = "no-keyboard-test"))]
    /// # {
    /// let keyboard = KeyboardApi::new()?;
    /// let result = keyboard.read_key_press(Duration::from_millis(400))?;
    /// println!("{:?}", result);
    /// # }
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    pub fn read_key_press(&self, duration: Duration) -> Res<Vec<KeyPress>> {
        if duration.as_millis() > i32::MAX as u128 {
            return Err(ErrorRoccatVulcanApi::InvalidInput);
        }
        let mut vector_result = Vec::new();
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
                vector_result.push(KeyPress::new_from_buffer(buffer));
            }
        }
        Ok(vector_result)
    }

    /// Block the thread until a key event or an error occur
    /// # Errors
    /// [`ErrorRoccatVulcanApi::ReadDeviceError`] when the read device has an error
    pub fn wait_for_key_press(&self) -> Res<KeyPress> {
        self.listen_key_press()
            .map_err(ErrorRoccatVulcanApi::ReadDeviceError)
    }

    /// wait for a key press and return a [`Keypress`]
    fn listen_key_press(&self) -> Result<KeyPress, hidapi::HidError> {
        let buffer = self.listen_key_press_raw()?;
        Ok(KeyPress::new_from_buffer(buffer))
    }

    /// wait for key press and return the raw value
    fn listen_key_press_raw(&self) -> Result<[u8; 5], hidapi::HidError> {
        let mut buffer: [u8; 5] = [0; 5];
        self.read.read(&mut buffer)?;
        Ok(buffer)
    }
}

impl Drop for KeyboardApi {
    fn drop(&mut self) {
        let _ = self.initialise_control_device(ControlerFeatureKind::Rainbow);
    }
}

/* impl Debug for KeyboardApi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO
        todo!()
    }
}

impl Display for KeyboardApi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO
        todo!()
    }
} */

/// Kind of feature report
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
#[non_exhaustive]
enum ControlerFeatureKind {
    /// Default rainbow behavior.
    Rainbow,
    /// Mode where the API can send custom configuration.
    Custom,
}

impl Default for ControlerFeatureKind {
    /// Returns [`ControlerFeatureKind::Custom`]
    fn default() -> Self {
        Self::Custom
    }
}

impl Display for ControlerFeatureKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rainbow => write!(f, "rainbow"),
            Self::Custom => write!(f, "custom"),
        }
    }
}
