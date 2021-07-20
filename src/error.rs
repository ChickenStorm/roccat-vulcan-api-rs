use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Duration;

/// Error returned by the API
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorRoccatVulcanApi {
    /// No keyboard detected.
    KeyboardNotFound,
    /// LED device not found
    NoLedDevice,
    /// LED device error
    LedDeviceError(hidapi::HidError),
    /// Control device not found
    NoControlDevice,
    /// Control device error
    ControlDeviceError(hidapi::HidError),
    /// Read device not found
    NoReadDevice,
    /// Read device error
    ReadDeviceError(hidapi::HidError),
    /// Too much time elapsed while wating for the device to be ready
    WaitedToMuchTime(Duration),
    /// error while trying the get the hdiapi.
    HidApiError(hidapi::HidError),
    /// Invalide input
    InvalidInput,
}

impl Display for ErrorRoccatVulcanApi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyboardNotFound => write!(f, "keyboard not found"),
            Self::NoLedDevice => write!(f, "led device not found"),
            Self::LedDeviceError(error) => write!(f, "led device error : {}", error),
            Self::NoControlDevice => write!(f, "control device not found"),
            Self::ControlDeviceError(error) => write!(f, "control device error : {}", error),
            Self::NoReadDevice => write!(f, "read device not found"),
            Self::ReadDeviceError(error) => write!(f, "read device error : {}", error),
            Self::WaitedToMuchTime(duration) => write!(
                f,
                "waited for too long, waited for {} ms",
                duration.as_secs_f64() * 1_000_f64
            ),
            Self::HidApiError(error) => write!(f, "hid api error : {}", error),
            Self::InvalidInput => write!(f, "invalide input"),
        }
    }
}

impl Error for ErrorRoccatVulcanApi {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::KeyboardNotFound
            | Self::NoLedDevice
            | Self::NoControlDevice
            | Self::NoReadDevice
            | Self::InvalidInput
            | Self::WaitedToMuchTime(_) => None,
            Self::LedDeviceError(error)
            | Self::ControlDeviceError(error)
            | Self::ReadDeviceError(error)
            | Self::HidApiError(error) => Some(error),
        }
    }
}
