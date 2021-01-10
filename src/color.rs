
use std::{
    convert::{From},
    cmp::{PartialOrd, PartialEq, Ord, Eq},
    default::Default
};
use super::constants;

pub trait Color {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Copy)]
pub struct ColorRgb {
    r : u8,
    g : u8,
    b : u8,
}

impl ColorRgb {
    
    pub fn new(r : u8, g: u8, b: u8) -> Self {
        Self {r, g, b}
    }
    
    pub fn new_from_array(array: [u8; 3]) -> Self {
        Self {
            r: array[0],
            g: array[1],
            b: array[2],
        }
    }
}

impl Color for ColorRgb {
    fn r(&self) -> u8 {
        return self.r;
    }
    
    fn g(&self) -> u8 {
        return self.g;
    }
    
    fn b(&self) -> u8 {
        return self.b;
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

impl From<ColorLuminosity> for ColorRgb {
    fn from(c: ColorLuminosity) -> Self {
        ColorRgb::new(c.r(), c.g(), c.b())
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct ColorLuminosity {
    r : u8,
    g : u8,
    b : u8,
    luminosity: Luminosity
}

impl ColorLuminosity {
    
    pub fn new(r : u8, g: u8, b: u8, luminosity: Luminosity) -> Self {
        Self {r, g, b, luminosity}
    }
    
    pub fn r_raw(&self) -> u8 {
        return self.r;
    }
    
    pub fn g_raw(&self) -> u8 {
        return self.g;
    }
    
    pub fn b_raw(&self) -> u8 {
        return self.b;
    }
    
    pub fn luminosity(&self) -> Luminosity {
        return self.luminosity;
    }
}

impl Color for ColorLuminosity {
    fn r(&self) -> u8 {
        ((self.r as u16 * self.luminosity.l() as u16) / 255u16) as u8
    }
    
    fn g(&self) -> u8 {
        ((self.g as u16 * self.luminosity.l() as u16) / 255u16) as u8
    }
    
    fn b(&self) -> u8 {
        ((self.b as u16 * self.luminosity.l() as u16) / 255u16) as u8
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Copy)]
pub struct Luminosity {
    l: u8,
}

impl Luminosity {
    pub fn new(l : u8) -> Self {
        Self {l}
    }
    
    pub fn l(&self) -> u8 {
        self.l
    }
}

impl From<u8> for Luminosity {
    fn from(l: u8) -> Self {
        Luminosity::new(l)
    }
}

impl From<f32> for Luminosity {
    fn from(f: f32) -> Self {
        return Luminosity::from((f.max(0f32).min(1f32) * 255f32 ).floor());
    }
}

impl From<ColorRgb> for ColorLuminosity {
    fn from(c: ColorRgb) -> Self { 
        ColorLuminosity::new(c.r, c.g, c.b, Luminosity::from(255u8))
    }
}

pub struct ColorBuffer<T>
    where T: Color + Copy
{
    buffer: [T; constants::NUMBER_KEY_LED_BUFFER]
}


const BUFFER_SIZE_RAW: usize = constants::LED_FEATURE_REPORT_HEAD.len() + constants::NUMBER_KEY_LED_BUFFER * 3;
const BUFFER_SIZE_PACKETED: usize = (((get_packeted_index_from_raw(BUFFER_SIZE_RAW - 1, constants::BITE_PACKET_SIZE) + 1) /(constants::BITE_PACKET_SIZE + 1)) + 1) * (constants::BITE_PACKET_SIZE + 1);

impl<T> ColorBuffer<T> 
    where T: Color + Copy
{
    pub fn new(color : T) -> Self {
        Self {
            buffer : [color; constants::NUMBER_KEY_LED_BUFFER]
        }
    }
    
    pub fn buffer (&self) -> &[T; constants::NUMBER_KEY_LED_BUFFER] {
        &self.buffer
    }
    
    pub fn buffer_mut (&mut self) -> &mut[T; constants::NUMBER_KEY_LED_BUFFER]{
        &mut self.buffer
    }

    pub fn get_led_buffer(&self) -> [u8; BUFFER_SIZE_PACKETED] {
        const LENGTH_HEAD: usize = constants::LED_FEATURE_REPORT_HEAD.len();
        let mut buffer_return = [0x00; BUFFER_SIZE_PACKETED];
        // each packet must start with 0x00
        for index in 0..LENGTH_HEAD {
            buffer_return[get_packeted_index_from_raw(index, constants::BITE_PACKET_SIZE)] = constants::LED_FEATURE_REPORT_HEAD[index];
        }
        for index in 0..self.buffer.len() {
            // we need to send the color by packet for 12 key
            let packet_number = index / constants::KEY_PACKET_SIZE; // int division
            let buffer_index = (index % constants::KEY_PACKET_SIZE) + constants::KEY_PACKET_SIZE * 3 * packet_number;
            buffer_return[
                get_packeted_index_from_raw(LENGTH_HEAD + buffer_index, constants::BITE_PACKET_SIZE)
            ] = self.buffer[index].r();
            buffer_return[
                get_packeted_index_from_raw(LENGTH_HEAD + buffer_index + constants::KEY_PACKET_SIZE, constants::BITE_PACKET_SIZE)
            ] = self.buffer[index].g();
            buffer_return[
                get_packeted_index_from_raw(LENGTH_HEAD + buffer_index + constants::KEY_PACKET_SIZE * 2, constants::BITE_PACKET_SIZE)
            ] = self.buffer[index].b();
        }
        return buffer_return;
    }
}

impl<T> Default for ColorBuffer<T> 
    where T: Color + Copy + Default
{
    fn default() -> Self{
        Self::new(T::default())
    }
}

const fn get_packeted_index_from_raw(index: usize, packet_size: usize) -> usize {
    return (index + 1) + (index) / packet_size;
}

#[cfg(test)]
#[test]
fn test_packet_index(){
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
#[cfg(test)]
#[test]
fn test_buffer_size(){
    assert_eq!(BUFFER_SIZE_PACKETED % 65, 0)
}
