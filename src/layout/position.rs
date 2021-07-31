//! module giving position related structs

use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

/// Represent the position of a key
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Position {
    /// X coord
    x: f64,
    /// Y coord
    y: f64,
}

impl Position {
    /// Create a new position
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Get x coordinate
    pub const fn x(&self) -> f64 {
        self.x
    }

    /// Get the y coordinate
    pub const fn y(&self) -> f64 {
        self.y
    }

    /// Get x coordinate
    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.x
    }

    /// Get the y coordinate
    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.y
    }

    /// Get the distance from the origin
    pub fn length(&self) -> f64 {
        self.x.hypot(self.y)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Add for Position {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self -= other;
        self
    }
}

impl Neg for Position {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;
        self
    }
}

/* TODO Rectangle
/// Size of a rectangle
pub type Size = Position;

/// Represent a rectangle : a key position and a size
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Rectangle {
    middle: Position,
    size: Size,
}

impl Rectangle {
    /// Get the middle of a rectangle
    pub const fn middle(&self) -> Position {
        self.middle
    }

    /// Get the size of a rectangle
    pub const fn size(&self) -> Size {
        self.size
    }
}
*/

#[cfg(test)]
mod test {

    use approx::assert_abs_diff_eq;

    use super::*;

    /// Test [`Position`]
    #[test]
    fn position() {
        let p1 = Position::new(1_f64, 2_f64);
        let p2 = Position::new(0_f64, 4_f64);

        assert_eq!(p1 + p2, Position::new(1_f64, 6_f64));
        assert_eq!(p1 - p2, Position::new(1_f64, -2_f64));
        assert_eq!(-p1, Position::new(-1_f64, -2_f64));
        assert_abs_diff_eq!(p1.length(), 1_f64.hypot(2_f64));
        assert_abs_diff_eq!(p2.length(), 4_f64);
        assert_eq!(format!("{}", p1), "(1, 2)");
        assert_eq!(format!("{}", p2), "(0, 4)");
    }
}
