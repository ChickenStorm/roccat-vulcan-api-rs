#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

mod layout_fr_ch;
pub use layout_fr_ch::*;

mod keypress;
pub use keypress::*;

mod key_name;
pub use key_name::*;

mod position;
pub use position::*;

/// Code for key light. This represent the key position in the buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct KeyLight {
    code: u8,
}

impl KeyLight {
    /// Create a new [`KeyLight`]
    pub const fn new(n: u8) -> Self {
        Self { code: n }
    }

    /// Get the key code.
    pub const fn code(self) -> u8 {
        self.code
    }

    /// Get the key code as mutbale.
    pub fn code_mut(&mut self) -> &mut u8 {
        &mut self.code
    }
}

///  associative data for a key.
#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct KeyInfo {
    key_name: KeyName,
    key_code_light: KeyLight,
    key_code_press: KeyCode,
    key_pos: Position,
}

impl KeyInfo {
    /// Create a new key info.
    pub const fn new(
        key_code_light: KeyLight,
        key_code_press: KeyCode,
        key_name: KeyName,
        key_pos: Position,
    ) -> Self {
        Self {
            key_name,
            key_code_light,
            key_code_press,
            key_pos,
        }
    }

    /// Get the key code ofr the led.
    pub const fn key_code_light(&self) -> &KeyLight {
        &self.key_code_light
    }

    /// Get the key code for press events
    pub const fn key_code_press(&self) -> &KeyCode {
        &self.key_code_press
    }

    /// Get the key name
    pub const fn key_name(&self) -> &KeyName {
        &self.key_name
    }

    /// Get the key code ofr the led.
    pub fn key_code_light_mut(&mut self) -> &mut KeyLight {
        &mut self.key_code_light
    }

    /// Get the key code for press events
    pub fn key_code_press_mut(&mut self) -> &mut KeyCode {
        &mut self.key_code_press
    }

    /// Get the key name
    pub fn key_name_mut(&mut self) -> &mut KeyName {
        &mut self.key_name
    }
}

/// Defines a Keyboard layout
pub trait Layout {
    /// returns the layout
    fn layout(&self) -> &[KeyInfo];

    /// Find key info from y [`KeyName`]
    fn find_from_key_name(&self, key_name: KeyName) -> Option<&KeyInfo> {
        self.layout().iter().find(|info| info.key_name == key_name)
    }

    /// Find key info from a [`KeyCode`]
    fn find_from_key_code(&self, key_code: KeyCode) -> Option<&KeyInfo> {
        self.layout()
            .iter()
            .find(|info| info.key_code_press == key_code)
    }

    /// Find key info from a [`KeyLight`]
    fn find_from_key_light(&self, key_code: KeyLight) -> Option<&KeyInfo> {
        self.layout()
            .iter()
            .find(|info| info.key_code_light == key_code)
    }
}
