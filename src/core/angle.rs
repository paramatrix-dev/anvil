use core::f64;

use uom::si::angle::{degree, radian};

use super::IntoF64;

/// A physical angle (i.e. a distance).
///
/// Angle exists to remove ambiguity about angle units, which are not supported by default by
/// major CAD kernels.
pub type Angle = uom::si::f64::Angle;

/// Import this trait to easily convert numbers into `Angle`s.
///
/// ```rust
/// use anvil::{Angle, IntoAngle};
/// use uom::si::angle::{degree, radian};
///
/// assert_eq!(5.deg(), Angle::new::<degree>(5.));
/// assert_eq!(5.123.rad(), Angle::new::<radian>(5.123));
/// ```
pub trait IntoAngle: IntoF64 {
    /// Convert this number into a `Angle` in degrees.
    ///
    /// ```rust
    /// use anvil::{IntoAngle, Angle};
    /// use uom::si::angle::degree;
    ///
    /// assert_eq!(5.deg(), Angle::new::<degree>(5.));
    /// ```
    fn deg(&self) -> Angle {
        Angle::new::<degree>(self.to_f64())
    }
    /// Convert this number into a `Angle` in radians.
    ///
    /// ```rust
    /// use anvil::{IntoAngle, Angle};
    /// use uom::si::angle::radian;
    ///
    /// assert_eq!(5.rad(), Angle::new::<radian>(5.));
    /// ```
    fn rad(&self) -> Angle {
        Angle::new::<radian>(self.to_f64())
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
