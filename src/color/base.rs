//! Contains base color structure
//! Specifically [`ColorRgb`] and [`ColorRgba`]

use std::cmp::Ordering;
use std::fmt::{Binary, Display, Formatter, LowerHex, Octal, UpperHex};

#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

/// RGB color representation
/// # Example
/// ```
/// use roccat_vulcan_api_rs::{ColorBuffer, ColorRgb};
///
/// let _buffer = ColorBuffer::from_element(ColorRgb::new(255, 255, 255));
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Hash, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct ColorRgb {
    /// Red
    r: u8,
    /// Green
    g: u8,
    /// Blue
    b: u8,
}

impl ColorRgb {
    /// Create a bew color from rgb values
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// create [`ColorRgb`] form `[R, G, B]`
    /// # Example
    /// ```
    /// use roccat_vulcan_api_rs::ColorRgb;
    ///
    /// let array = [255, 0, 0];
    /// assert_eq!(ColorRgb::new_from_array(array), ColorRgb::new(255, 0, 0));
    /// ```
    pub const fn new_from_array(array: [u8; 3]) -> Self {
        Self {
            r: array[0],
            g: array[1],
            b: array[2],
        }
    }

    /// Get the red intensity
    pub const fn r(self) -> u8 {
        self.r
    }

    /// Get the green intensity
    pub const fn g(self) -> u8 {
        self.g
    }

    /// Get the blue intensity
    pub const fn b(self) -> u8 {
        self.b
    }

    /// Get the red intensity
    pub fn r_mut(&mut self) -> &mut u8 {
        &mut self.r
    }

    /// Get the green intensity
    pub fn g_mut(&mut self) -> &mut u8 {
        &mut self.g
    }

    /// Get the blue intensity
    pub fn b_mut(&mut self) -> &mut u8 {
        &mut self.b
    }

    /// Return the color as a u32 encoded as `0x00_RR_GG_BB`
    /// # Example
    /// ```
    /// use roccat_vulcan_api_rs::ColorRgb;
    ///
    /// let color = ColorRgb::new(0xFF, 0xC5, 0x09);
    /// let number = 0x00_FF_C5_09_u32;
    /// assert_eq!(color.into_u32(), number);
    /// ```
    pub const fn into_u32(self) -> u32 {
        ((self.r as u32) << 16_u32) + ((self.g as u32) << 8_u32) + self.b as u32
    }

    /// Create the color form a u32.
    /// The number is encoded as `0x**_RR_GG_BB`, where `**` are dropped bites.
    /// # Example
    /// ```
    /// use roccat_vulcan_api_rs::ColorRgb;
    ///
    /// let number = 0x00_FF_C5_09_u32;
    /// let color = ColorRgb::from_u32(number);
    /// assert_eq!(color, ColorRgb::new(0xFF, 0xC5, 0x09));
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    pub const fn from_u32(c: u32) -> Self {
        Self::new((c >> 16) as u8, (c >> 8) as u8, c as u8)
    }
}

// TODO

// pub enum ValueCreationError {
//     TooSmall,
//     TooBig,
//     Nan,
// }

/// Color saturation, guarantees that the value is between 0 and 1.
///
/// Used by [`ColorRgb::new_hsv`].
///
/// # Example
/// ```
/// use roccat_vulcan_api_rs::Saturation;
///
/// assert_eq!(Saturation::new(0.5_f64).unwrap().value(), 0.5_f64);
/// assert_eq!(Saturation::new(1.5_f64), None);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Saturation(f64);

impl Saturation {
    /// Create a new value if `s` is between 0 and 1 both included. Otherwise return [`None`]
    pub fn new(s: f64) -> Option<Self> {
        if (0_f64..=1_f64).contains(&s) && !s.is_nan() {
            Some(Self(s))
        } else {
            None
        }
    }

    /// Get the value wrapped
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// Color value (also called brightness). Gauranties that the value is between 0 and 1.
///
/// Used by [`ColorRgb::new_hsv`].
///
/// # Example
/// ```
/// use roccat_vulcan_api_rs::Value;
///
/// assert_eq!(Value::new(0.5_f64).unwrap().value(), 0.5_f64);
/// assert_eq!(Value::new(1.5_f64), None);
/// ```
pub type Value = Saturation;

impl Eq for Saturation {}

impl Ord for Saturation {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => unreachable!(),
        }
    }
}

impl PartialOrd for Saturation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Display for Saturation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Saturation {
    fn default() -> Self {
        match Self::new(0_f64) {
            Some(s) => s,
            None => unreachable!(),
        }
    }
}

/// Color hue. Guarantees that the value is between 0 and 1.
///
/// Used by [`ColorRgb::new_hsv`].
///
/// # Example
/// ```
/// use roccat_vulcan_api_rs::Hue;
///
/// assert_eq!(Hue::new(0.5_f64).unwrap().value(), 0.5_f64);
/// assert_eq!(Hue::new(1.5_f64), None);
/// assert_eq!(Hue::new_from_degree(180_f64), Hue::new(0.5_f64))
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hue(f64);

impl Hue {
    /// Create a new value between 0 and 1
    /// # Example
    /// ```
    /// use roccat_vulcan_api_rs::Hue;
    ///
    /// assert_eq!(Hue::new(0.5_f64).unwrap().value(), 0.5_f64);
    /// assert_eq!(Hue::new(1.5_f64), None);
    /// ```
    pub fn new(s: f64) -> Option<Self> {
        if (0_f64..=1_f64).contains(&s) && !s.is_nan() {
            Some(Self(s))
        } else {
            None
        }
    }

    /// Create a hue from an angle in degree
    /// # Example
    /// ```
    /// use roccat_vulcan_api_rs::Hue;
    ///
    /// assert_eq!(Hue::new_from_degree(180_f64), Hue::new(0.5_f64))
    /// ```
    pub fn new_from_degree(f: f64) -> Option<Self> {
        Self::new(f / 360_f64)
    }

    /// Create a new hue from an angle in radiant
    /// # Example
    /// ```
    /// use std::f64::consts::PI;
    ///
    /// use roccat_vulcan_api_rs::Hue;
    ///
    /// assert_eq!(Hue::new_from_radiant(PI), Hue::new(0.5_f64))
    /// ```
    pub fn new_from_radiant(f: f64) -> Option<Self> {
        Self::new(f / (2_f64 * std::f64::consts::PI))
    }

    /// Get the hue in degree
    /// # Example
    /// ```
    /// use roccat_vulcan_api_rs::Hue;
    ///
    /// let hue = Hue::new(0.5_f64).unwrap();
    /// assert_eq!(hue.degree(), 180_f64);
    /// let hue = Hue::new(0_f64).unwrap();
    /// assert_eq!(hue.degree(), 0_f64);
    /// let hue = Hue::new(0.25_f64).unwrap();
    /// assert_eq!(hue.degree(), 90_f64);
    /// ```
    pub fn degree(self) -> f64 {
        self.value() * 360_f64
    }

    /// Get the hue in radiant
    pub fn radiant(self) -> f64 {
        self.value() * 2_f64 * std::f64::consts::PI
    }

    /// Get the value wrapped
    pub const fn value(self) -> f64 {
        self.0
    }
}

impl Eq for Hue {}

impl PartialOrd for Hue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Hue {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => unreachable!(),
        }
    }
}

impl Display for Hue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Hue {
    /// Create a new Hue with value `0_f64`
    fn default() -> Self {
        match Self::new(0_f64) {
            Some(s) => s,
            None => unreachable!(),
        }
    }
}

impl ColorRgb {
    /// Convert a value between 0 and 1 en un [u8] entre 0 et 255
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn convert_to_u8(v: f64) -> u8 {
        // the cast is safe
        (v * 255_f64).round().min(255_f64).max(0_f64) as u8
    }

    /// Create a new color with hue, saturation and value space
    pub fn new_hsv(hue: Hue, saturation: Saturation, value: Value) -> Self {
        let chroma = saturation.value() * value.value();
        let h_prime = hue.value() * 6_f64;
        let x = chroma * (1_f64 - ((h_prime % 2_f64) - 1_f64).abs());

        #[allow(clippy::cast_possible_truncation)]
        let (r1, g1, b1) = match h_prime.ceil() as i32 {
            0_i32 | 1_i32 => (chroma, x, 0_f64),
            2_i32 => (x, chroma, 0_f64),
            3_i32 => (0_f64, chroma, x),
            4_i32 => (0_f64, x, chroma),
            5_i32 => (x, 0_f64, chroma),
            6_i32 => (chroma, 0_f64, x),
            _ => (0_f64, 0_f64, 0_f64),
        };

        let m = value.value() - chroma;
        let red = Self::convert_to_u8(r1 + m);
        let green = Self::convert_to_u8(g1 + m);
        let blue = Self::convert_to_u8(b1 + m);
        Self::new(red, green, blue)
    }

    /// Get the Hue Saturation Value representation of this color.
    pub fn into_hsv(self) -> (Hue, Saturation, Value) {
        let value = self.r.max(self.b).max(self.g);
        let min = self.r.min(self.b).min(self.g);
        let chroma = value - min;

        let hue = if chroma == 0 {
            0_f64
        } else if value == self.r {
            1_f64 / 6_f64 * (self.g as f64 - self.b as f64) / chroma as f64
        } else if value == self.g {
            1_f64 / 6_f64 * (2_f64 + (self.b as f64 - self.r as f64) / chroma as f64)
        } else {
            // value == b
            1_f64 / 6_f64 * (4_f64 + (self.r as f64 - self.g as f64) / chroma as f64)
        };

        let s_v = if value == 0 {
            0_f64
        } else {
            chroma as f64 / value as f64
        };

        (
            Hue::new(hue).unwrap(),
            Saturation::new(s_v).unwrap(),
            Value::new(value as f64 / 255_f64).unwrap(),
        )
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

impl From<u32> for ColorRgb {
    fn from(n: u32) -> Self {
        ColorRgb::from_u32(n)
    }
}

impl From<ColorRgb> for u32 {
    fn from(c: ColorRgb) -> Self {
        c.into_u32()
    }
}

impl From<ColorRgb> for (u8, u8, u8) {
    fn from(color: ColorRgb) -> Self {
        (color.r(), color.g(), color.b())
    }
}

impl Display for ColorRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(r: {}, g: {}, b: {})", self.r(), self.g(), self.b())
    }
}

impl Binary for ColorRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.into_u32())
    }
}

impl UpperHex for ColorRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.into_u32())
    }
}

impl LowerHex for ColorRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.into_u32())
    }
}

impl Octal for ColorRgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:o}", self.into_u32())
    }
}

/// Color with alpha parameter
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct ColorRgba {
    /// base color
    color: ColorRgb,
    /// opacity
    alpha: u8,
}

impl ColorRgba {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]

    /// Create a new RGBA color with the alpha value given by a [`f32`]
    pub fn new_from_float(r: u8, g: u8, b: u8, f: f32) -> Self {
        Self {
            color: ColorRgb::new(r, g, b),
            alpha: (f.max(0_f32).min(1_f32) * 255_f32).floor() as u8,
        }
    }

    /// Create a new RGBA color
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            color: ColorRgb::new(r, g, b),
            alpha: a,
        }
    }

    /// Return the transparency value (0: transparent, 255: opaque)
    pub const fn alpha(self) -> u8 {
        self.alpha
    }

    /// Return the row color without the alpha
    pub const fn color(self) -> ColorRgb {
        self.color
    }

    /// apply the luminosity level to the value as it would be mixed with black
    #[allow(clippy::cast_possible_truncation)]
    const fn apply_lumninosity(value: u8, lum: u8) -> u8 {
        ((value as u16 * lum as u16) / 255_u16) as u8
    }

    /// Get the red value taking into account the luminosity
    pub const fn r(self) -> u8 {
        Self::apply_lumninosity(self.color().r(), self.alpha())
    }

    /// Get the green value taking into account the luminosity
    pub const fn g(self) -> u8 {
        Self::apply_lumninosity(self.color().g(), self.alpha())
    }

    /// Get the blue value taking into account the luminosity
    pub const fn b(self) -> u8 {
        Self::apply_lumninosity(self.color().b(), self.alpha())
    }

    /// Convert the color into a color RGB mixing it with black
    pub const fn into_color(self) -> ColorRgb {
        ColorRgb::new(self.r(), self.g(), self.b())
    }

    /// Convert the color to an u32
    pub const fn into_u32(self) -> u32 {
        (self.color().into_u32() << 4_u32) + (self.alpha() as u32)
    }

    /// Convert the color to an u32
    #[allow(clippy::cast_possible_truncation)]
    pub const fn from_u32(n: u32) -> Self {
        Self {
            color: ColorRgb::from_u32(n >> 4),
            alpha: n as u8,
        }
    }

    //TODO color add sub etc...
}

impl Display for ColorRgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(r: {}, g: {}, b: {}, a: {})",
            self.color().r(),
            self.color().g(),
            self.color().b(),
            self.alpha()
        )
    }
}

impl Binary for ColorRgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.into_u32())
    }
}

impl UpperHex for ColorRgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.into_u32())
    }
}

impl LowerHex for ColorRgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.into_u32())
    }
}

impl Octal for ColorRgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:o}", self.into_u32())
    }
}

impl From<ColorRgb> for ColorRgba {
    fn from(c: ColorRgb) -> Self {
        Self::new(c.r(), c.g(), c.b(), 255)
    }
}

impl From<ColorRgba> for ColorRgb {
    fn from(c: ColorRgba) -> Self {
        c.into_color()
    }
}

impl From<u32> for ColorRgba {
    fn from(n: u32) -> Self {
        ColorRgba::from_u32(n)
    }
}

impl From<ColorRgba> for u32 {
    fn from(c: ColorRgba) -> Self {
        c.into_u32()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color() {
        let ca = ColorRgba::new(30, 87, 70, 255);
        let c = ColorRgb::new(30, 87, 70);
        assert_eq!(c, ca.color());
        assert_eq!(c.r(), ca.r());

        let ca = ColorRgba::new(20, 0, 0, 128);
        assert_eq!(ca.r(), 10);
        assert_eq!(ca.b(), 0);
    }

    #[test]
    fn hsv_color() {
        let c = ColorRgb::new_hsv(
            Hue::new(1_f64).unwrap(),
            Saturation::new(1_f64).unwrap(),
            Value::new(1_f64).unwrap(),
        );
        assert_eq!(c, ColorRgb::new(255, 0, 0));

        let c = ColorRgb::new_hsv(
            Hue::new(0.5_f64).unwrap(),
            Saturation::new(1_f64).unwrap(),
            Value::new(1_f64).unwrap(),
        );
        assert_eq!(c, ColorRgb::new(0, 255, 255));
    }
}
