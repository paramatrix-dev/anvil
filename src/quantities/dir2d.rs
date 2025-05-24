use std::ops::Mul;

use crate::Error;

use super::{Length, Point2D};

/// A direction in 2D space with a length of 1.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dir2D {
    x: f64,
    y: f64,
}
impl Dir2D {
    /// Construct a `Dir2D` from the directional components.
    ///
    /// Returns an Error::ZeroVector if all of the axis components are zero.
    ///
    /// ```rust
    /// use anvil::Dir2D;
    ///
    /// let dir2 = Dir2D::try_from(3., 4.).expect("");
    /// assert_eq!(dir2.x(), 3. / 5.);
    /// assert_eq!(dir2.y(), 4. / 5.);
    /// ```
    pub fn try_from(x: f64, y: f64) -> Result<Self, Error> {
        let magnitude = (x.powi(2) + y.powi(2)).sqrt();
        if magnitude == 0. {
            return Err(Error::ZeroVector);
        }
        Ok(Dir2D {
            x: x / magnitude,
            y: y / magnitude,
        })
    }

    /// Return the x-component of this `Dir2D`.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Return the y-component of this `Dir2D`.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Return the dot-product of this `Dir2D` with another.
    pub fn dot(&self, other: Dir2D) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Mul<Length> for Dir2D {
    type Output = Point2D;
    /// Multiply this `Dir2D` with a `Length` to get a `Point2D`.
    ///
    /// ```rust
    /// use anvil::{Dir2D, length, point};
    ///
    /// let dir2 = Dir2D::try_from(1., 0.).unwrap();
    /// assert_eq!(
    ///     dir2 * length!(2 m),
    ///     point!(2 m, 0 m)
    /// )
    /// ```
    fn mul(self, other: Length) -> Point2D {
        Point2D {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Mul<Dir2D> for Length {
    type Output = Point2D;
    /// Multiply this `Length` with a `Dir2D` to get a `Point2D`.
    ///
    /// ```rust
    /// use anvil::{Dir2D, length, point};
    ///
    /// let dir2 = Dir2D::try_from(1., 0.).unwrap();
    /// assert_eq!(
    ///     length!(2 m) * dir2,
    ///     point!(2 m, 0 m)
    /// )
    /// ```
    fn mul(self, other: Dir2D) -> Point2D {
        other * self
    }
}

impl Mul<&Length> for Dir2D {
    type Output = Point2D;
    /// Multiply this `Dir2D` with a `&Length` to get a `Point2D`.
    ///
    /// ```rust
    /// use anvil::{Dir2D, length, point};
    ///
    /// let dir2 = Dir2D::try_from(1., 0.).unwrap();
    /// assert_eq!(
    ///     dir2 * &length!(2 m),
    ///     point!(2 m, 0 m)
    /// )
    /// ```
    fn mul(self, other: &Length) -> Point2D {
        Point2D {
            x: self.x * *other,
            y: self.y * *other,
        }
    }
}

impl Mul<Dir2D> for &Length {
    type Output = Point2D;
    /// Multiply this `&Length` with a `Dir2D` to get a `Point2D`.
    ///
    /// ```rust
    /// use anvil::{Dir2D, length, point};
    ///
    /// let dir2 = Dir2D::try_from(1., 0.).unwrap();
    /// assert_eq!(
    ///     &length!(2 m) * dir2,
    ///     point!(2 m, 0 m)
    /// )
    /// ```
    fn mul(self, other: Dir2D) -> Point2D {
        other * self
    }
}
