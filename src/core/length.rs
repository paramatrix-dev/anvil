use std::ops::Mul;

use uom::si::length::{centimeter, decimeter, foot, inch, meter, millimeter, yard};

use crate::{Dir, IntoF64, Point};

/// A physical length (i.e. a distance).
///
/// Length exists to remove ambiguity about distance units, which are not supported by default by
/// major CAD kernels.
pub type Length = uom::si::f64::Length;

impl<const DIM: usize> Mul<Dir<DIM>> for Length {
    type Output = Point<DIM>;
    /// Multiply this `Length` with a `Dir` to get a `Point`.
    ///
    /// ```rust
    /// use anvil::{IntoLength, dir, point};
    ///
    /// // for 2d
    /// assert_eq!(
    ///     2.m() * dir!(1, 0),
    ///     point!(2.m(), 0.m())
    /// );
    ///
    /// // for 3d
    /// assert_eq!(
    ///     2.m() * dir!(1, 0, 0),
    ///     point!(2.m(), 0.m(), 0.m())
    /// );
    /// ```
    fn mul(self, other: Dir<DIM>) -> Point<DIM> {
        other * self
    }
}

/// Return true if any IntoLength in the input array is zero.
pub fn is_zero(lengths: &[Length]) -> bool {
    for length in lengths {
        if length.get::<meter>() == 0. {
            return true;
        }
    }
    false
}

/// Import this trait to easily convert numbers into `Length`s.
///
/// ```rust
/// use anvil::{IntoLength, Length};
/// use uom::si::length::{foot, meter};
///
/// assert_eq!(5.m(), Length::new::<meter>(5.));
/// assert_eq!(5.123.ft(), Length::new::<foot>(5.123));
/// ```
pub trait IntoLength: IntoF64 {
    /// Convert this number into a `Length` in yard.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Length};
    /// use uom::si::length::yard;
    ///
    /// assert_eq!(5.yd(), Length::new::<yard>(5.));
    /// ```
    fn yd(&self) -> Length {
        Length::new::<yard>(self.to_f64())
    }
    /// Convert this number into a `Length` in meters.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Length};
    /// use uom::si::length::meter;
    ///
    /// assert_eq!(5.m(), Length::new::<meter>(5.));
    /// ```
    fn m(&self) -> Length {
        Length::new::<meter>(self.to_f64())
    }
    /// Convert this number into a `Length` in feet.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Length};
    /// use uom::si::length::foot;
    ///
    /// assert_eq!(5.ft(), Length::new::<foot>(5.));
    /// ```
    fn ft(&self) -> Length {
        Length::new::<foot>(self.to_f64())
    }
    /// Convert this number into a `Length` in decimeters.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Length};
    /// use uom::si::length::decimeter;
    ///
    /// assert_eq!(5.dm(), Length::new::<decimeter>(5.));
    /// ```
    fn dm(&self) -> Length {
        Length::new::<decimeter>(self.to_f64())
    }
    /// Convert this number into a `Length` in inches.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Length};
    /// use uom::si::length::inch;
    ///
    /// assert_eq!(5.in_(), Length::new::<inch>(5.));
    /// ```
    fn in_(&self) -> Length {
        Length::new::<inch>(self.to_f64())
    }
    /// Convert this number into a `Length` in centimeters.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Length};
    /// use uom::si::length::centimeter;
    ///
    /// assert_eq!(5.cm(), Length::new::<centimeter>(5.));
    /// ```
    fn cm(&self) -> Length {
        Length::new::<centimeter>(self.to_f64())
    }
    /// Convert this number into a `Length` in millimeters.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Length};
    /// use uom::si::length::millimeter;
    ///
    /// assert_eq!(5.mm(), Length::new::<millimeter>(5.));
    /// ```
    fn mm(&self) -> Length {
        Length::new::<millimeter>(self.to_f64())
    }
}

impl IntoLength for usize {}
impl IntoLength for isize {}
impl IntoLength for u8 {}
impl IntoLength for u16 {}
impl IntoLength for u32 {}
impl IntoLength for u64 {}
impl IntoLength for u128 {}
impl IntoLength for i8 {}
impl IntoLength for i16 {}
impl IntoLength for i32 {}
impl IntoLength for i64 {}
impl IntoLength for i128 {}
impl IntoLength for f32 {}
impl IntoLength for f64 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(2.m() + 3.m(), 5.m());
    }

    #[test]
    fn subtract() {
        assert_eq!(3.m() - 2.m(), 1.m());
    }

    #[test]
    fn multiply_with_f64() {
        assert_eq!(5.m() * 4., 20.m());
        assert_eq!(4. * 5.m(), 20.m());
    }

    #[test]
    fn divide_with_f64() {
        assert_eq!(6.m() / 2., 3.m());
    }
}
