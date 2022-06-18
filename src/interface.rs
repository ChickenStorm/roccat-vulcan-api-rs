//! Contain interfaces filters.

use std::fmt::{Display, Formatter};

use hidapi::DeviceInfo;
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

/// Product is of the Vulcan 100.
const VULCAN_100_PRODUCT_ID: u16 = 12_410;
/// Product is of the Vulcan 120.
const VULCAN_120_PRODUCT_ID: u16 = 12_440;
/// Default usage page for the read device.
/// The reason the usage page is 10 is unknown and may change for different device.
const USAGE_PAGE_READ_DEVICE: u16 = 10;
/// Default interface number of the read device.
const READ_INTERFACE_NUMBER: i32 = 1_i32;
/// Default interface number of the control device.
const CONTROL_INTERFACE_NUMBER: i32 = 1_i32;
/// Default interface number of the led device.
const LED_INTERFACE_NUMBER: i32 = 3_i32;

/// Basic HID interface filter with a product id and an interface number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct HidInterfaceFilter {
    /// Product number
    product_id: u16,
    /// interface of communication
    interface_number: i32,
    /// Optionally an usage page
    usage_page: Option<u16>,
}

impl HidInterfaceFilter {
    /// Create a new [`HidInterfaceFilter`]
    pub const fn new(product_id: u16, interface_number: i32) -> Self {
        Self {
            product_id,
            interface_number,
            usage_page: None,
        }
    }

    /// Create a new filter with an usage page
    pub const fn new_with_usage_page(
        product_id: u16,
        interface_number: i32,
        usage_page: u16,
    ) -> Self {
        Self {
            product_id,
            interface_number,
            usage_page: Some(usage_page),
        }
    }

    /// Get the product id.
    pub const fn product_id(&self) -> u16 {
        self.product_id
    }

    /// Get the product id as a mut ref.
    pub fn product_id_mut(&mut self) -> &mut u16 {
        &mut self.product_id
    }

    /// Get the interface number.
    pub const fn interface_number(&self) -> i32 {
        self.interface_number
    }

    /// Get the interface number as a mut reference.
    pub fn interface_number_mut(&mut self) -> &mut i32 {
        &mut self.interface_number
    }

    /// Get the usage page
    pub const fn usage_page(&self) -> Option<u16> {
        self.usage_page
    }

    /// Get a mut reference to the usage page
    pub fn usage_page_mut(&mut self) -> &mut Option<u16> {
        &mut self.usage_page
    }

    /// returns whether or not a device match the filter
    pub fn match_filter(&self, device: &DeviceInfo) -> bool {
        let match_usage = if let Some(val) = self.usage_page() {
            val == device.usage_page()
        } else {
            true
        };
        match_usage
            && self.product_id() == device.product_id()
            && self.interface_number() == device.interface_number()
    }
}

impl Display for HidInterfaceFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO improve
        write!(
            f,
            "product id: {}, interface number: {}",
            self.product_id(),
            self.interface_number()
        )?;
        if let Some(page) = self.usage_page {
            return write!(f, ", usage page {}", page);
        }
        Ok(())
    }
}

/// list the filter for the different devices used by [`super::KeyboardApi`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct KeyboardIntrefacesFilter {
    /// The read device filter
    read_interface: HidInterfaceFilter,
    /// The control device filter
    control_interface: HidInterfaceFilter,
    /// The led device filter
    led_interface: HidInterfaceFilter,
}

impl KeyboardIntrefacesFilter {
    /// Get the read interface.
    pub const fn read_interface(&self) -> &HidInterfaceFilter {
        &self.read_interface
    }

    /// Get the read interface as a mutable reference.
    pub fn read_interface_mut(&mut self) -> &mut HidInterfaceFilter {
        &mut self.read_interface
    }

    /// Get the control interface.
    pub const fn control_interface(&self) -> &HidInterfaceFilter {
        &self.control_interface
    }

    /// Get the control interface as a mutable reference.
    pub fn control_interface_mut(&mut self) -> &mut HidInterfaceFilter {
        &mut self.control_interface
    }

    /// Get the led interface.
    pub const fn led_interface(&self) -> &HidInterfaceFilter {
        &self.led_interface
    }

    /// Get the led interface as a mutable reference.
    pub fn led_interface_mut(&mut self) -> &mut HidInterfaceFilter {
        &mut self.led_interface
    }

    /// Default vulcan 100 info.
    pub const fn vulcan_100() -> Self {
        let product_id = VULCAN_100_PRODUCT_ID;
        Self {
            read_interface: HidInterfaceFilter::new_with_usage_page(
                product_id,
                READ_INTERFACE_NUMBER,
                USAGE_PAGE_READ_DEVICE,
            ),
            control_interface: HidInterfaceFilter::new(product_id, CONTROL_INTERFACE_NUMBER),
            led_interface: HidInterfaceFilter::new(product_id, LED_INTERFACE_NUMBER),
        }
    }

    /// Default vulcan 120 info.
    pub const fn vulcan_120() -> Self {
        let product_id = VULCAN_120_PRODUCT_ID;
        Self {
            read_interface: HidInterfaceFilter::new_with_usage_page(
                product_id,
                READ_INTERFACE_NUMBER,
                USAGE_PAGE_READ_DEVICE,
            ),
            control_interface: HidInterfaceFilter::new(product_id, CONTROL_INTERFACE_NUMBER),
            led_interface: HidInterfaceFilter::new(product_id, LED_INTERFACE_NUMBER),
        }
    }

    /// Array containing the default models.
    pub const DEFAULT_MODEL: [Self; 2] = [Self::vulcan_100(), Self::vulcan_120()];
}

impl Display for KeyboardIntrefacesFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "read_interface: {}, control_interface: {}, led_interface: {}",
            self.read_interface(),
            self.control_interface(),
            self.led_interface()
        )
    }
}

impl Default for KeyboardIntrefacesFilter {
    fn default() -> Self {
        Self::vulcan_120()
    }
}
