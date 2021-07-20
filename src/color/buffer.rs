//! module for the color buffer

use crate::{color::ColorRgb, reports};
use std::iter::FusedIterator;
// #[cfg(feature = "serde-serialize")]
// use serde::{Deserialize, Serialize};

/// Size of the [`ColorBuffer`].
pub const NUMBER_KEY_LED_BUFFER: usize = 144;
/// size of key packet, i.e we write the r value for 12 key then the g for 12 key etc
const KEY_PACKET_SIZE: usize = 12;
/// the size of bite packet send to the device
pub(crate) const BITE_PACKET_SIZE: usize = 64;

/// raw buffer site based on buffer header and number of key
const BUFFER_SIZE_RAW: usize = reports::LED_FEATURE_REPORT_HEAD.len() + NUMBER_KEY_LED_BUFFER * 3;
/// Total size of the buffer separated in packer of [0x00, 64 bits]
const BUFFER_SIZE_PACKETED: usize =
    (((get_packeted_index_from_raw(BUFFER_SIZE_RAW - 1, BITE_PACKET_SIZE) + 1)
        / (BITE_PACKET_SIZE + 1))
        + 1)
        * (BITE_PACKET_SIZE + 1);

/// get the index in the array packetet form the raw array
const fn get_packeted_index_from_raw(index: usize, packet_size: usize) -> usize {
    (index + 1) + (index) / packet_size
}

/// Encode the set of color for [`crate::KeyboardApi::render`].
/// TODO more doc
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
// TODO serd
//#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct ColorBuffer<T> {
    buffer: [T; NUMBER_KEY_LED_BUFFER],
}

impl<T> ColorBuffer<T> {
    /// Get the buffer as an array
    pub const fn buffer(&self) -> &[T; NUMBER_KEY_LED_BUFFER] {
        &self.buffer
    }

    /// Get a mutable reference to the buffer as an array
    pub fn buffer_mut(&mut self) -> &mut [T; NUMBER_KEY_LED_BUFFER] {
        &mut self.buffer
    }

    /// Returns an iterator in the colors
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &T> + FusedIterator + ExactSizeIterator + DoubleEndedIterator {
        self.buffer.iter()
    }

    /// Return an mutable iterator on the colors
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut T> + FusedIterator + ExactSizeIterator + DoubleEndedIterator
    {
        self.buffer.iter_mut()
    }
}
//TODO genericité ?
impl<C: Into<ColorRgb> + Clone> ColorBuffer<C> {
    /// Get an array of u8 that is ready to be send to the led device
    pub fn get_led_buffer(&self) -> [u8; BUFFER_SIZE_PACKETED] {
        const LENGTH_HEAD: usize = reports::LED_FEATURE_REPORT_HEAD.len();
        let mut buffer_return = [0x00; BUFFER_SIZE_PACKETED];
        // each packet must start with 0x00
        for (index, val) in reports::LED_FEATURE_REPORT_HEAD.iter().enumerate() {
            buffer_return[get_packeted_index_from_raw(index, BITE_PACKET_SIZE)] = *val;
        }
        for (index, val) in self.buffer.iter().enumerate() {
            // we need to send the color by packet for 12 key
            let packet_number = index / KEY_PACKET_SIZE; // int division
            let buffer_index = (index % KEY_PACKET_SIZE) + KEY_PACKET_SIZE * 3 * packet_number;
            let color = val.clone().into();
            buffer_return
                [get_packeted_index_from_raw(LENGTH_HEAD + buffer_index, BITE_PACKET_SIZE)] =
                color.r();
            buffer_return[get_packeted_index_from_raw(
                LENGTH_HEAD + buffer_index + KEY_PACKET_SIZE,
                BITE_PACKET_SIZE,
            )] = color.g();
            buffer_return[get_packeted_index_from_raw(
                LENGTH_HEAD + buffer_index + KEY_PACKET_SIZE * 2,
                BITE_PACKET_SIZE,
            )] = color.b();
        }
        buffer_return
    }
}

impl<T: Copy> ColorBuffer<T> {
    /// Create the buffer with the same color for each key
    pub fn from_element(color: T) -> Self {
        Self {
            buffer: [color; NUMBER_KEY_LED_BUFFER],
        }
    }
}

impl<T: Copy + Default> ColorBuffer<T> {
    /// Create a new buffer with default values
    pub fn new() -> Self {
        Self::from_element(T::default())
    }
}

impl<T: Default + Copy> Default for ColorBuffer<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_buffer_size() {
        assert_eq!(BUFFER_SIZE_PACKETED % (BITE_PACKET_SIZE + 1), 0);
    }

    #[test]
    #[allow(clippy::missing_const_for_fn)]
    fn test_led_buffer_length() {
        assert_eq!(NUMBER_KEY_LED_BUFFER % KEY_PACKET_SIZE, 0);
    }

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
}
