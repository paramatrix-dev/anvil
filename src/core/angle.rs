use core::f64;
use std::ops::{Add, Div, Mul, Neg, Sub};

use super::IntoF64;

/// A physical angle (i.e. a distance).
///
/// Angle exists to remove ambiguity about angle units, which are not supported by default by
/// major CAD kernels.
///
/// ```rust
/// use anvil::Angle;
///
/// // You can construct an angle using the Angle::from_[unit] methods:
/// let degrees_angle = Angle::from_deg(1.2);
/// let radians_angle = Angle::from_rad(3.4);
///
/// // To get back a angle value in a specific unit, call the Angle.[unit] method
/// assert_eq!(degrees_angle.deg(), 1.2);
/// assert_eq!(radians_angle.rad(), 3.4);
///
/// // Angle construction can be simplified using the `IntoAngle` trait.
/// use anvil::IntoAngle;
///
/// assert_eq!(1.2.deg(), Angle::from_deg(1.2));
/// assert_eq!(4.5.rad(), Angle::from_rad(4.5));
/// ```
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Angle {
    rad: f64,
}
impl Angle {
    /// Construct a `Angle` with a value of zero.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Angle;
    ///
    /// let angle = Angle::zero();
    /// assert_eq!(angle.deg(), 0.);
    /// ```
    pub fn zero() -> Self {
        Self { rad: 0. }
    }
    /// Construct a `Angle` from a value in radians.
    ///
    /// # Example
    /// ```rust
    /// use core::f64;
    /// use anvil::Angle;
    ///
    /// let angle = Angle::from_rad(f64::consts::PI);
    /// assert_eq!(angle.deg(), 180.);
    /// ```
    pub fn from_rad(value: f64) -> Self {
        Self {
            rad: value % f64::consts::TAU,
        }
    }
    /// Return the value of this angle in radians.
    pub fn rad(&self) -> f64 {
        self.rad
    }
    /// Construct a `Angle` from a value in degrees.
    ///
    /// # Example
    /// ```rust
    /// use core::f64;
    /// use anvil::Angle;
    ///
    /// let angle = Angle::from_deg(180.);
    /// assert_eq!(angle.rad(), f64::consts::PI);
    /// ```
    pub fn from_deg(value: f64) -> Self {
        Angle {
            rad: value / 360. * f64::consts::TAU,
        }
    }
    /// Return the value of this angle in degrees.
    pub fn deg(&self) -> f64 {
        self.rad / f64::consts::TAU * 360.
    }

    /// Return the absolute value of this `Angle`.
    ///
    /// ```rust
    /// use anvil::IntoAngle;
    ///
    /// assert_eq!((-45).deg().abs(), 45.deg());
    /// assert_eq!(10.deg().abs(), 10.deg());
    /// ```
    pub fn abs(&self) -> Self {
        Self {
            rad: self.rad.abs(),
        }
    }

    /// Return the smaller of two angles.
    ///
    /// # Example
    /// ```rust
    /// use anvil::IntoAngle;
    ///
    /// let angle1 = 1.deg();
    /// let angle2 = 2.deg();
    /// assert_eq!(angle1.min(&angle2), angle1);
    /// assert_eq!(angle2.min(&angle1), angle1);
    /// ```
    pub fn min(&self, other: &Self) -> Self {
        Angle {
            rad: self.rad.min(other.rad),
        }
    }
    /// Return the larger of two lengths.
    ///
    /// # Example
    /// ```rust
    /// use anvil::IntoAngle;
    ///
    /// let angle1 = 1.deg();
    /// let angle2 = 2.deg();
    /// assert_eq!(angle1.max(&angle2), angle2);
    /// assert_eq!(angle2.max(&angle1), angle2);
    /// ```
    pub fn max(&self, other: &Self) -> Self {
        Angle {
            rad: self.rad.max(other.rad),
        }
    }
}

impl Add<Angle> for Angle {
    type Output = Angle;
    fn add(self, other: Angle) -> Angle {
        Angle {
            rad: self.rad + other.rad,
        }
    }
}

impl Sub<Angle> for Angle {
    type Output = Angle;
    fn sub(self, other: Angle) -> Angle {
        Angle {
            rad: self.rad - other.rad,
        }
    }
}

impl Mul<f64> for Angle {
    type Output = Angle;
    fn mul(self, other: f64) -> Angle {
        Angle {
            rad: self.rad * other,
        }
    }
}

impl Mul<Angle> for f64 {
    type Output = Angle;
    fn mul(self, other: Angle) -> Angle {
        other * self
    }
}

impl Div<f64> for Angle {
    type Output = Angle;
    fn div(self, other: f64) -> Angle {
        Angle {
            rad: self.rad / other,
        }
    }
}

impl Div<Angle> for Angle {
    type Output = f64;
    /// Divide a `Angle` by another `Angle`.
    /// ```rust
    /// use anvil::IntoAngle;
    ///
    /// assert_eq!(6.deg() / 2.deg(), 3.)
    /// ```
    fn div(self, other: Angle) -> f64 {
        self.rad / other.rad
    }
}

impl Neg for Angle {
    type Output = Angle;
    fn neg(self) -> Self::Output {
        self * -1.
    }
}

/// Import this trait to easily convert numbers into `Angle`s.
///
/// ```rust
/// use anvil::{Angle, IntoAngle};
///
/// assert_eq!(5.deg(), Angle::from_deg(5.));
/// assert_eq!(5.123.rad(), Angle::from_rad(5.123));
/// ```
pub trait IntoAngle: IntoF64 {
    /// Convert this number into a `Angle` in degrees.
    ///
    /// ```rust
    /// use anvil::{IntoAngle, Angle};
    ///
    /// assert_eq!(5.deg(), Angle::from_deg(5.));
    /// ```
    fn deg(&self) -> Angle {
        Angle::from_deg(self.to_f64())
    }
    /// Convert this number into a `Angle` in radians.
    ///
    /// ```rust
    /// use anvil::{IntoAngle, Angle};
    ///
    /// assert_eq!(5.rad(), Angle::from_rad(5.));
    /// ```
    fn rad(&self) -> Angle {
        Angle::from_rad(self.to_f64())
    }
}

impl IntoAngle for usize {}
impl IntoAngle for isize {}
impl IntoAngle for u8 {}
impl IntoAngle for u16 {}
impl IntoAngle for u32 {}
impl IntoAngle for u64 {}
impl IntoAngle for u128 {}
impl IntoAngle for i8 {}
impl IntoAngle for i16 {}
impl IntoAngle for i32 {}
impl IntoAngle for i64 {}
impl IntoAngle for i128 {}
impl IntoAngle for f32 {}
impl IntoAngle for f64 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(2.rad() + 3.rad(), 5.rad());
    }

    #[test]
    fn subtract() {
        assert_eq!(3.rad() - 2.rad(), 1.rad());
    }

    #[test]
    fn multiply_with_f64() {
        assert_eq!(0.2.rad() * 4., 0.8.rad());
        assert_eq!(4. * 0.2.rad(), 0.8.rad());
    }

    #[test]
    fn divide_with_f64() {
        assert_eq!(6.rad() / 2., 3.rad());
    }
}
