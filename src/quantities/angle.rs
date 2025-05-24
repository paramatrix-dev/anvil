use core::f64;
use std::ops::{Add, Div, Mul, Sub};

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
/// // Angle construction can also be simplified using the angle! macro
/// use anvil::angle;
///
/// assert_eq!(angle!(1.2 deg), Angle::from_deg(1.2));
/// assert_eq!(angle!(4.5 rad), Angle::from_rad(4.5));
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
        Angle { rad: value }
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

    /// Return the smaller of two angles.
    ///
    /// # Example
    /// ```rust
    /// use anvil::angle;
    ///
    /// let angle1 = angle!(1 deg);
    /// let angle2 = angle!(2 deg);
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
    /// use anvil::angle;
    ///
    /// let angle1 = angle!(1 deg);
    /// let angle2 = angle!(2 deg);
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
    /// use anvil::angle;
    ///
    /// assert_eq!(angle!(6 deg) / angle!(2 deg), 3.)
    /// ```
    fn div(self, other: Angle) -> f64 {
        self.rad / other.rad
    }
}

impl Div<&Angle> for Angle {
    type Output = f64;
    /// Divide a `Angle` by another `&Angle`.
    /// ```rust
    /// use anvil::angle;
    ///
    /// assert_eq!(angle!(6 deg) / &angle!(2 deg), 3.)
    /// ```
    fn div(self, other: &Angle) -> f64 {
        self.rad / other.rad
    }
}

/// Macro for simplifying `Angle` construction for static values.
///
/// Create an angle with the correct unit by invoking `angle!([value] [unit])`.
///
/// # Examples
/// ```rust
/// use anvil::{angle, Angle};
///
/// assert_eq!(angle!(5 deg), Angle::from_deg(5.));
/// assert_eq!(angle!(5.1 deg), Angle::from_deg(5.1));
/// assert_eq!(angle!(2 rad), Angle::from_rad(2.));
/// assert_eq!(angle!(0), Angle::zero());
/// ```
#[macro_export]
macro_rules! angle {
    ( 0 ) => {
        $crate::Angle::zero()
    };
    ( $val:literal deg ) => {
        $crate::Angle::from_deg($val as f64)
    };
    ( $val:literal rad ) => {
        $crate::Angle::from_rad($val as f64)
    };
    ($val:literal $unit:ident) => {
        compile_error!(concat!("Unsupported angle unit: ", stringify!($unit)))
    };
}

#[cfg(test)]
mod tests {
    use crate::angle;

    #[test]
    fn add() {
        assert_eq!(angle!(2 rad) + angle!(3 rad), angle!(5 rad));
    }

    #[test]
    fn subtract() {
        assert_eq!(angle!(3 rad) - angle!(2 rad), angle!(1 rad));
    }

    #[test]
    fn multiply_with_f64() {
        assert_eq!(angle!(5 rad) * 4., angle!(20 rad));
        assert_eq!(4. * angle!(5 rad), angle!(20 rad));
    }

    #[test]
    fn divide_with_f64() {
        assert_eq!(angle!(6 rad) / 2., angle!(3 rad));
    }
}
