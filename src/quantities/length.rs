use std::ops::{Add, Div, Mul, Sub};

/// A physical length (i.e. a distance).
///
/// Length exists to remove ambiguity about distance units, which are not supported by default by
/// major CAD kernels.
///
/// ```rust
/// use anvil::Length;
///
/// // You can construct a length using the Length::from_[unit] methods like
/// let meters_length = Length::from_m(1.2);
/// let centimeters_length = Length::from_cm(4.5);
/// let inches_length = Length::from_in(12.);
///
/// // To get back a length value in a specific unit, call the Length.[unit] method
/// assert_eq!(meters_length.cm(), 120.);
/// assert_eq!(centimeters_length.m(), 0.045);
/// assert!((inches_length.ft() - 1.).abs() < 1e-9);
///
/// // Length construction can also be simplified using the length! macro
/// use anvil::length;
///
/// assert_eq!(length!(1.2 m), Length::from_m(1.2));
/// assert_eq!(length!(4.5 cm), Length::from_cm(4.5));
/// assert_eq!(length!(12 in), Length::from_in(12.));
///
/// // You can savely add or subtract Lengths in different units.
/// assert_eq!(length!(1 m) + length!(4 cm), length!(104 cm));
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
    /// Return the value of this length in millimeters.
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
    /// Return the value of this length in yards.
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
    /// Return the value of this length in feet.
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
    /// Return the value of this length in decimeters.
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
    /// Return the value of this length in inches.
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
    /// Return the value of this length in centimeters.
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
    /// Return the value of this length in millimeters.
    pub fn mm(&self) -> f64 {
        self.m() * 1000.
    }

    /// Return the smaller of two lengths.
    ///
    /// # Example
    /// ```rust
    /// use anvil::length;
    ///
    /// let len1 = length!(1 m);
    /// let len2 = length!(2 m);
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
    /// use anvil::length;
    ///
    /// let len1 = length!(1 m);
    /// let len2 = length!(2 m);
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
    /// Divide a `Length` by another `Length`.
    /// ```rust
    /// use anvil::length;
    ///
    /// assert_eq!(length!(6 m) / length!(2 m), 3.)
    /// ```
    fn div(self, other: Length) -> f64 {
        self.meters / other.meters
    }
}

impl Div<&Length> for Length {
    type Output = f64;
    /// Divide a `Length` by another `Length`.
    /// ```rust
    /// use anvil::length;
    ///
    /// assert_eq!(length!(6 m) / &length!(2 m), 3.)
    /// ```
    fn div(self, other: &Length) -> f64 {
        self.meters / other.meters
    }
}

/// Return true if any length in the input array is zero.
pub fn is_zero(lengths: &[Length]) -> bool {
    for length in lengths {
        if length.m() == 0. {
            return true;
        }
    }
    false
}

/// Macro for simplifying `Length` construction for static values.
///
/// Create a length with the correct unit by invoking `length!([value] [unit])`.
///
/// # Examples
/// ```rust
/// use anvil::{length, Length};
///
/// assert_eq!(length!(1 yd), Length::from_yd(1.));
/// assert_eq!(length!(5 m), Length::from_m(5.));
/// assert_eq!(length!(5.1 m), Length::from_m(5.1));
/// assert_eq!(length!(1 ft), Length::from_ft(1.));
/// assert_eq!(length!(1 dm), Length::from_dm(1.));
/// assert_eq!(length!(1 in), Length::from_in(1.));
/// assert_eq!(length!(2 cm), Length::from_cm(2.));
/// assert_eq!(length!(1 mm), Length::from_mm(1.));
/// assert_eq!(length!(0), Length::zero());
/// ```
#[macro_export]
macro_rules! length {
    ( 0 ) => {
        $crate::Length::zero()
    };
    ( $val:literal yd ) => {
        $crate::Length::from_yd($val as f64)
    };
    ( $val:literal m ) => {
        $crate::Length::from_m($val as f64)
    };
    ( $val:literal ft ) => {
        $crate::Length::from_ft($val as f64)
    };
    ( $val:literal dm ) => {
        $crate::Length::from_dm($val as f64)
    };
    ( $val:literal in ) => {
        $crate::Length::from_in($val as f64)
    };
    ( $val:literal cm ) => {
        $crate::Length::from_cm($val as f64)
    };
    ( $val:literal mm ) => {
        $crate::Length::from_mm($val as f64)
    };
    ($val:literal $unit:ident) => {
        compile_error!(concat!("Unsupported length unit: ", stringify!($unit)))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(length!(2 m) + length!(3 m), length!(5 m));
    }

    #[test]
    fn subtract() {
        assert_eq!(length!(3 m) - length!(2 m), length!(1 m));
    }

    #[test]
    fn multiply_with_f64() {
        assert_eq!(length!(5 m) * 4., Length::from_m(20.));
        assert_eq!(4. * length!(5 m), Length::from_m(20.));
    }

    #[test]
    fn divide_with_f64() {
        assert_eq!(Length::from_m(6.) / 2., length!(3 m));
    }
}
