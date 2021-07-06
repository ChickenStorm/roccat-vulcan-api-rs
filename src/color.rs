//! Module to manage the color buffer for the led device
//!
//! # Examples
//! ```
//! use roccat_vulcan_api_rs::{ColorBuffer, ColorRgb};
//!
//! let color_buffer = ColorBuffer::new(ColorRgb::new(255, 255, 255));
//! ```

use super::constants;
use std::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    convert::From,
    default::Default,
};

/// Trait that define a Color with trhat can yield a (R,G,B) representation
pub trait Color {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;

    fn rgb(&self) -> [u8; 3] {
        [self.r(), self.g(), self.b()]
    }
}

/// RGB color representation
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Copy)]
pub struct ColorRgb {
    r: u8,
    g: u8,
    b: u8,
}

impl ColorRgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// create [`ColorRgb`] form `[R, G, B]`
    pub fn new_from_array(array: [u8; 3]) -> Self {
        Self {
            r: array[0],
            g: array[1],
            b: array[2],
        }
    }

    pub fn r_mut(&mut self) -> &mut u8 {
        &mut self.r
    }

    pub fn g_mut(&mut self) -> &mut u8 {
        &mut self.g
    }

    pub fn b_mut(&mut self) -> &mut u8 {
        &mut self.b
    }

    pub fn set_r(&mut self, r: u8) {
        self.r = r;
    }

    pub fn set_g(&mut self, g: u8) {
        self.g = g;
    }

    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }
}

impl Color for ColorRgb {
    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }
}

impl Default for ColorRgb {
    fn default() -> Self {
        ColorRgb::new(0, 0, 0)
    }
}

impl From<[u8; 3]> for ColorRgb {
    fn from(array: [u8; 3]) -> Self {
        ColorRgb::new_from_array(array)
    }
}

impl From<(u8, u8, u8)> for ColorRgb {
    fn from(tuple: (u8, u8, u8)) -> Self {
        let (r, g, b) = tuple;
        ColorRgb::new(r, g, b)
    }
}

/* error due to reimplementing  From<ColorRgb> for ColorRgb
impl<T: Color> From<T> for ColorRgb {
    fn from(c: T) -> Self {
        ColorRgb::new(c.r(), c.g(), c.b())
    }
}
*/

/// Color with alpha parameter
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct ColorRgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl ColorRgba {
    pub fn new_from_float(r: u8, g: u8, b: u8, f: f32) -> Self {
        Self {
            r,
            g,
            b,
            a: (f.max(0f32).min(1f32) * 255f32).floor() as u8,
        }
    }

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn r_raw(&self) -> u8 {
        self.r
    }

    pub fn g_raw(&self) -> u8 {
        self.g
    }

    pub fn b_raw(&self) -> u8 {
        self.b
    }

    pub fn a(&self) -> u8 {
        self.a
    }
}

impl Color for ColorRgba {
    fn r(&self) -> u8 {
        ((self.r as u16 * self.a as u16) / 255_u16) as u8
    }

    fn g(&self) -> u8 {
        ((self.g as u16 * self.a as u16) / 255_u16) as u8
    }

    fn b(&self) -> u8 {
        ((self.b as u16 * self.a as u16) / 255_u16) as u8
    }
}

impl From<ColorRgb> for ColorRgba {
    fn from(c: ColorRgb) -> Self {
        ColorLuminosity::new(c.r, c.g, c.b, 255_u8)
    }
}

impl From<ColorRgba> for ColorRgb {
    fn from(c: ColorRgba) -> Self {
        ColorRgb::new(c.r(), c.g(), c.b())
    }
}

/// Color with luminosity
pub type ColorLuminosity = ColorRgba;

/// Encode the set of color for [`super::KeyboardApi::render`].
pub struct ColorBuffer<T>
where
    T: Color + Copy,
{
    buffer: [T; constants::NUMBER_KEY_LED_BUFFER],
}

// raw buffer site based on buffer header and number of key
const BUFFER_SIZE_RAW: usize =
    constants::LED_FEATURE_REPORT_HEAD.len() + constants::NUMBER_KEY_LED_BUFFER * 3;
// Total size of teh buffer separated in packer of [0x00, 64 bits]
const BUFFER_SIZE_PACKETED: usize =
    (((get_packeted_index_from_raw(BUFFER_SIZE_RAW - 1, constants::BITE_PACKET_SIZE) + 1)
        / (constants::BITE_PACKET_SIZE + 1))
        + 1)
        * (constants::BITE_PACKET_SIZE + 1);

impl<T> ColorBuffer<T>
where
    T: Color + Copy,
{
    /// Create the buffer with the same color for each key
    pub fn new(color: T) -> Self {
        Self {
            buffer: [color; constants::NUMBER_KEY_LED_BUFFER],
        }
    }

    /// get the buffer
    pub fn buffer(&self) -> &[T; constants::NUMBER_KEY_LED_BUFFER] {
        &self.buffer
    }

    /// get a mutable ref to the buffer
    pub fn buffer_mut(&mut self) -> &mut [T; constants::NUMBER_KEY_LED_BUFFER] {
        &mut self.buffer
    }

    ///get an array of u8 that is ready to be send to the led device
    pub fn get_led_buffer(&self) -> [u8; BUFFER_SIZE_PACKETED] {
        const LENGTH_HEAD: usize = constants::LED_FEATURE_REPORT_HEAD.len();
        let mut buffer_return = [0x00; BUFFER_SIZE_PACKETED];
        // each packet must start with 0x00
        for index in 0..LENGTH_HEAD {
            buffer_return[get_packeted_index_from_raw(index, constants::BITE_PACKET_SIZE)] =
                constants::LED_FEATURE_REPORT_HEAD[index];
        }
        for index in 0..self.buffer.len() {
            // we need to send the color by packet for 12 key
            let packet_number = index / constants::KEY_PACKET_SIZE; // int division
            let buffer_index = (index % constants::KEY_PACKET_SIZE)
                + constants::KEY_PACKET_SIZE * 3 * packet_number;
            buffer_return[get_packeted_index_from_raw(
                LENGTH_HEAD + buffer_index,
                constants::BITE_PACKET_SIZE,
            )] = self.buffer[index].r();
            buffer_return[get_packeted_index_from_raw(
                LENGTH_HEAD + buffer_index + constants::KEY_PACKET_SIZE,
                constants::BITE_PACKET_SIZE,
            )] = self.buffer[index].g();
            buffer_return[get_packeted_index_from_raw(
                LENGTH_HEAD + buffer_index + constants::KEY_PACKET_SIZE * 2,
                constants::BITE_PACKET_SIZE,
            )] = self.buffer[index].b();
        }
        buffer_return
    }
}

/// get the index in the array packetet form the raw array
const fn get_packeted_index_from_raw(index: usize, packet_size: usize) -> usize {
    (index + 1) + (index) / packet_size
}

impl<T> Default for ColorBuffer<T>
where
    T: Color + Copy + Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// test [`get_packeted_index_from_raw`]
#[cfg(test)]
#[test]
fn test_packet_index() {
    assert_eq!(get_packeted_index_from_raw(0, 64), 1);
    assert_eq!(get_packeted_index_from_raw(63, 64), 64);
    assert_eq!(get_packeted_index_from_raw(64, 64), 66);
    assert_eq!(get_packeted_index_from_raw(65, 64), 67);
    assert_eq!(get_packeted_index_from_raw(128, 64), 131);
    assert_eq!(get_packeted_index_from_raw(31, 32), 32);
    assert_eq!(get_packeted_index_from_raw(32, 32), 34);
    assert_eq!(get_packeted_index_from_raw(33, 32), 35);
    assert_eq!(get_packeted_index_from_raw(0, 32), 1);
    assert_eq!(get_packeted_index_from_raw(4, 2), 7);
}

/// test that the size of the array reterned by [`ColorBuffer::get_led_buffer`] is of correct size
#[cfg(test)]
#[test]
fn test_buffer_size() {
    assert_eq!(BUFFER_SIZE_PACKETED % (constants::BITE_PACKET_SIZE + 1), 0)
}
