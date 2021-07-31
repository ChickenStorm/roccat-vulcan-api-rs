//! Contains the struct for key event [`KeyPress`] and [`KeyCode`]

use std::fmt::{Display, Formatter};

#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

/// Key press event from the read device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct KeyPress {
    /// Which key recived an event
    key_code: KeyCode,
    /// Is the key pressed or released
    is_pressed: bool,
}

impl KeyPress {
    /// read key press from buffer hid read buffer.
    pub const fn new_from_buffer(buffer: [u8; 5]) -> Self {
        Self {
            key_code: KeyCode::new(buffer[2], buffer[3]),
            is_pressed: {
                match buffer[2] {
                    10 => buffer[4] == 0, // for some reason the caps lock signal is inverted
                    _ => buffer[4] != 0,
                }
            },
        }
    }

    /// Create a new event from a [`KeyCode`] and a [`bool`]
    pub const fn new(key_code: KeyCode, is_pressed: bool) -> Self {
        Self {
            key_code,
            is_pressed,
        }
    }

    /// Get the key code.
    pub const fn key_code(&self) -> &KeyCode {
        &self.key_code
    }

    /// Is pressed event.
    pub const fn is_pressed(self) -> bool {
        self.is_pressed
    }

    /// Get the key code.
    pub fn key_code_mut(&mut self) -> &mut KeyCode {
        &mut self.key_code
    }

    /// Is pressed event.
    pub fn is_pressed_mut(&mut self) -> &mut bool {
        &mut self.is_pressed
    }
}

impl From<KeyPress> for KeyCode {
    fn from(d: KeyPress) -> Self {
        d.key_code
    }
}

impl Display for KeyPress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.is_pressed() {
            true => write!(f, "key code {} being pressed", self.key_code()),
            false => write!(f, "key code {} being release", self.key_code()),
        }
    }
}

/// structur of data to incode the key when a key press is read form hid device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct KeyCode {
    /// first u8 representing the key code
    first_u8: u8,
    /// Second u8 representing the key code
    seconde_u8: u8,
}

impl KeyCode {
    /// Create a new key code
    pub const fn new(first_u8: u8, seconde_u8: u8) -> Self {
        Self {
            first_u8,
            seconde_u8,
        }
    }

    /// First u8 of the code.
    pub const fn first_u8(self) -> u8 {
        self.first_u8
    }

    /// Seconde u8 of the code.
    pub const fn seconde_u8(self) -> u8 {
        self.seconde_u8
    }

    /// First u8 of the code.
    pub fn first_u8_mut(&mut self) -> &mut u8 {
        &mut self.first_u8
    }

    /// Seconde u8 of the code.
    pub fn seconde_u8_mut(&mut self) -> &mut u8 {
        &mut self.seconde_u8
    }
}

impl From<[u8; 2]> for KeyCode {
    fn from(array: [u8; 2]) -> Self {
        KeyCode::new(array[0], array[1])
    }
}

impl From<(u8, u8)> for KeyCode {
    fn from(tuple: (u8, u8)) -> Self {
        let (a, b) = tuple;
        KeyCode::new(a, b)
    }
}

impl From<KeyCode> for [u8; 2] {
    fn from(d: KeyCode) -> Self {
        [d.first_u8(), d.seconde_u8()]
    }
}

impl Display for KeyCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.first_u8(), self.seconde_u8())
    }
}
