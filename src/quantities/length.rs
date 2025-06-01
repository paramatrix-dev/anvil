use std::ops::{Add, Div, Mul, Neg, Sub};

use super::{Dir2D, Dir3D, IntoF64, Point2D, Point3D};

/// A physical length (i.e. a distance).
///
/// Length exists to remove ambiguity about distance units, which are not supported by default by
/// major CAD kernels.
///
/// ```rust
/// use anvil::Length;
///
/// // You can construct a `Length` using the Length::from_[unit] methods like
/// let meters_length = Length::from_m(1.2);
/// let centimeters_length = Length::from_cm(4.5);
/// let inches_length = Length::from_in(12.);
///
/// // To get back a `Length` value in a specific unit, call the Length.[unit] method
/// assert_eq!(meters_length.cm(), 120.);
/// assert_eq!(centimeters_length.m(), 0.045);
/// assert!((inches_length.ft() - 1.).abs() < 1e-9);
///
/// // Length construction can be simplified using the `IntoLength` trait.
/// use anvil::IntoLength;
///
/// assert_eq!(1.2.m(), Length::from_m(1.2));
/// assert_eq!(4.5.cm(), Length::from_cm(4.5));
/// assert_eq!(12.in_(), Length::from_in(12.));
/// ```
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Length {
    meters: f64,
}
impl Length {
    /// Construct a `Length` with a value of zero.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Length;
    ///
    /// let len = Length::zero();
    /// assert_eq!(len.m(), 0.);
    /// ```
    pub fn zero() -> Self {
        Self { meters: 0. }
    }
    /// Construct a `Length` from a value of unit meters.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Length;
    ///
    /// let len = Length::from_m(3.2);
    /// assert_eq!(len.mm(), 3200.);
    /// ```
    pub fn from_m(value: f64) -> Self {
        Self { meters: value }
    }
    /// Return the value of this `Length` in millimeters.
    pub fn m(&self) -> f64 {
        self.meters
    }
    /// Construct a `Length` from a value of unit yards.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Length;
    ///
    /// let len = Length::from_yd(1.);
    /// assert_eq!(len.m(), 0.9144);
    /// ```
    pub fn from_yd(value: f64) -> Self {
        Self::from_m(value * 0.9144)
    }
    /// Return the value of this `Length` in yards.
    pub fn yd(&self) -> f64 {
        self.m() / 0.9144
    }
    /// Construct a `Length` from a value of unit feet.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Length;
    ///
    /// let len = Length::from_ft(1.);
    /// assert_eq!(len.cm(), 30.48);
    /// ```
    pub fn from_ft(value: f64) -> Self {
        Self::from_m(value * 0.3048)
    }
    /// Return the value of this `Length` in feet.
    pub fn ft(&self) -> f64 {
        self.m() / 0.3048
    }
    /// Construct a `Length` from a value of unit decimeters.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Length;
    ///
    /// let len = Length::from_dm(5.1);
    /// assert_eq!(len.mm(), 510.);
    /// ```
    pub fn from_dm(value: f64) -> Self {
        Self::from_m(value / 10.)
    }
    /// Return the value of this `Length` in decimeters.
    pub fn dm(&self) -> f64 {
        self.m() * 10.
    }
    /// Construct a `Length` from a value of unit inches.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Length;
    ///
    /// let len = Length::from_in(1.);
    /// assert_eq!(len.cm(), 2.54);
    /// ```
    pub fn from_in(value: f64) -> Self {
        Self::from_m(value * 0.0254)
    }
    /// Return the value of this `Length` in inches.
    ///
    /// This method breaks the pattern with the trailing underscore, because `in` is a reserved
    /// keyword in Rust.
    pub fn in_(&self) -> f64 {
        self.m() / 0.0254
    }
    /// Construct a `Length` from a value of unit centimeters.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Length;
    ///
    /// let len = Length::from_cm(5.1);
    /// assert_eq!(len.mm(), 51.);
    /// ```
    pub fn from_cm(value: f64) -> Self {
        Self::from_m(value / 100.)
    }
    /// Return the value of this `Length` in centimeters.
    pub fn cm(&self) -> f64 {
        self.m() * 100.
    }
    /// Construct a `Length` from a value of unit millimeters.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Length;
    ///
    /// let len = Length::from_mm(5.4);
    /// assert_eq!(len.m(), 0.0054);
    /// ```
    pub fn from_mm(value: f64) -> Self {
        Self::from_m(value / 1000.)
    }
    /// Return the value of this `Length` in millimeters.
    pub fn mm(&self) -> f64 {
        self.m() * 1000.
    }

    /// Return the absolute value of this `Length`.
    ///
    /// ```rust
    /// use anvil::IntoLength;
    ///
    /// assert_eq!((-5).m().abs(), 5.m());
    /// assert_eq!(5.m().abs(), 5.m());
    /// ```
    pub fn abs(&self) -> Self {
        Self {
            meters: self.meters.abs(),
        }
    }
    /// Return the smaller of two lengths.
    ///
    /// # Example
    /// ```rust
    /// use anvil::IntoLength;
    ///
    /// let len1 = 1.m();
    /// let len2 = 2.m();
    /// assert_eq!(len1.min(&len2), len1);
    /// assert_eq!(len2.min(&len1), len1);
    /// ```
    pub fn min(&self, other: &Self) -> Self {
        Length::from_m(self.m().min(other.m()))
    }
    /// Return the larger of two lengths.
    ///
    /// # Example
    /// ```rust
    /// use anvil::IntoLength;
    ///
    /// let len1 = 1.m();
    /// let len2 = 2.m();
    /// assert_eq!(len1.max(&len2), len2);
    /// assert_eq!(len2.max(&len1), len2);
    /// ```
    pub fn max(&self, other: &Self) -> Self {
        Length::from_m(self.m().max(other.m()))
    }
}

impl Add<Length> for Length {
    type Output = Length;
    fn add(self, other: Length) -> Length {
        Length::from_m(self.m() + other.m())
    }
}

impl Sub<Length> for Length {
    type Output = Length;
    fn sub(self, other: Length) -> Length {
        Length::from_m(self.m() - other.m())
    }
}

impl Mul<f64> for Length {
    type Output = Length;
    fn mul(self, other: f64) -> Length {
        Length::from_m(self.m() * other)
    }
}

impl Mul<Length> for f64 {
    type Output = Length;
    fn mul(self, other: Length) -> Length {
        other * self
    }
}

impl Div<f64> for Length {
    type Output = Length;
    fn div(self, other: f64) -> Length {
        Length::from_m(self.m() / other)
    }
}

impl Div<Length> for Length {
    type Output = f64;
    /// Divide this `Length` by another `Length`.
    /// ```rust
    /// use anvil::IntoLength;
    ///
    /// assert_eq!(6.m() / 2.m(), 3.)
    /// ```
    fn div(self, other: Length) -> f64 {
        self.meters / other.meters
    }
}

impl Mul<Dir2D> for Length {
    type Output = Point2D;
    /// Multiply this `Length` with a `Dir2D` to get a `Point2D`.
    ///
    /// ```rust
    /// use anvil::{dir, IntoLength, point};
    ///
    /// let dir2 = dir!(1, 0);
    /// assert_eq!(
    ///     2.m() * dir2,
    ///     point!(2.m(), 0.m())
    /// )
    /// ```
    fn mul(self, other: Dir2D) -> Point2D {
        other * self
    }
}

impl Mul<Dir3D> for Length {
    type Output = Point3D;
    /// Multiply this `Length` with a `Dir3D` to get a `Point3D`.
    ///
    /// ```rust
    /// use anvil::{dir, IntoLength, point};
    ///
    /// let dir3 = dir!(1, 0, 0);
    /// assert_eq!(
    ///     2.m() * dir3,
    ///     point!(2.m(), 0.m(), 0.m())
    /// )
    /// ```
    fn mul(self, other: Dir3D) -> Point3D {
        other * self
    }
}

impl Neg for Length {
    type Output = Length;
    fn neg(self) -> Self::Output {
        self * -1.
    }
}

/// Return true if any IntoLength in the input array is zero.
pub fn is_zero(lengths: &[Length]) -> bool {
    for length in lengths {
        if length.m() == 0. {
            return true;
        }
    }
    false
}

/// Import this trait to easily convert numbers into `Length`s.
///
/// ```rust
/// use anvil::{IntoLength, Length};
///
/// assert_eq!(5.m(), Length::from_m(5.));
/// assert_eq!(5.123.ft(), Length::from_ft(5.123));
/// ```
pub trait IntoLength: IntoF64 {
    /// Convert this number into a `Length` in yard.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Length};
    ///
    /// assert_eq!(5.yd(), Length::from_yd(5.));
    /// ```
    fn yd(&self) -> Length {
        Length::from_yd(self.to_f64())
    }
    /// Convert this number into a `Length` in meters.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Length};
    ///
    /// assert_eq!(5.m(), Length::from_m(5.));
    /// ```
    fn m(&self) -> Length {
        Length::from_m(self.to_f64())
    }
    /// Convert this number into a `Length` in feet.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Length};
    ///
    /// assert_eq!(5.ft(), Length::from_ft(5.));
    /// ```
    fn ft(&self) -> Length {
        Length::from_ft(self.to_f64())
    }
    /// Convert this number into a `Length` in decimeters.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Length};
    ///
    /// assert_eq!(5.dm(), Length::from_dm(5.));
    /// ```
    fn dm(&self) -> Length {
        Length::from_dm(self.to_f64())
    }
    /// Convert this number into a `Length` in inches.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Length};
    ///
    /// assert_eq!(5.in_(), Length::from_in(5.));
    /// ```
    fn in_(&self) -> Length {
        Length::from_in(self.to_f64())
    }
    /// Convert this number into a `Length` in centimeters.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Length};
    ///
    /// assert_eq!(5.cm(), Length::from_cm(5.));
    /// ```
    fn cm(&self) -> Length {
        Length::from_cm(self.to_f64())
    }
    /// Convert this number into a `Length` in millimeters.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Length};
    ///
    /// assert_eq!(5.mm(), Length::from_mm(5.));
    /// ```
    fn mm(&self) -> Length {
        Length::from_mm(self.to_f64())
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
        assert_eq!(5.m() * 4., Length::from_m(20.));
        assert_eq!(4. * 5.m(), Length::from_m(20.));
    }

    #[test]
    fn divide_with_f64() {
        assert_eq!(Length::from_m(6.) / 2., 3.m());
    }
}
