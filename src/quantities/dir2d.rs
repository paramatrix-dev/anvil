use std::ops::{Add, Mul};

use crate::Error;

use super::{Angle, Length, Point2D};

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
    /// let dir2 = Dir2D::try_from(3., 4.).unwrap();
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

    /// Return the `Angle` this `Dir2D` points to in relation to the unit circle.
    ///
    /// ```rust
    /// use anvil::{dir, IntoAngle};
    ///
    /// assert!((dir!(1, 0).angle() - 0.deg()).rad().abs() < 1e-9);
    /// assert!((dir!(1, 1).angle() - 45.deg()).rad().abs() < 1e-9);
    /// assert!((dir!(0, 1).angle() - 90.deg()).rad().abs() < 1e-9);
    /// assert!((dir!(-1, 1).angle() - 135.deg()).rad().abs() < 1e-9);
    /// assert!((dir!(-1, 0).angle() - 180.deg()).rad().abs() < 1e-9);
    /// assert!((dir!(-1, -1).angle() - 225.deg()).rad().abs() < 1e-9);
    /// assert!((dir!(0, -1).angle() - 270.deg()).rad().abs() < 1e-9);
    /// assert!((dir!(1, -1).angle() - 315.deg()).rad().abs() < 1e-9);
    /// ```
    pub fn angle(&self) -> Angle {
        let angle = Angle::from_rad(self.y.atan2(self.x));
        if angle.rad() < 0. {
            Angle::from_rad(angle.rad() + std::f64::consts::TAU)
        } else {
            angle
        }
    }

    /// Return the dot-product of this `Dir2D` with another.
    pub fn dot(&self, other: Dir2D) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Return a `Dir2D` rotated by a specified amount counter clockwise.
    pub fn rotate(&self, angle: Angle) -> Self {
        Self::from(self.angle() + angle)
    }
}

impl From<Angle> for Dir2D {
    /// Construct a `Dir2D` from an `Angle`.
    ///
    /// An angle of 0 points in the positive x-direction and positive angles rotate counter
    /// clockwise.
    fn from(value: Angle) -> Self {
        Self {
            x: f64::cos(value.rad()),
            y: f64::sin(value.rad()),
        }
    }
}

impl Add<Dir2D> for Dir2D {
    type Output = Result<Self, Error>;
    /// Add another `Dir2D` to this one.
    ///
    /// ```rust
    /// use anvil::{dir, Error};
    ///
    /// assert_eq!(
    ///     dir!(0, 1) + dir!(1, 0),
    ///     Ok(dir!(1, 1))
    /// );
    /// assert_eq!(
    ///     dir!(1, 1) + dir!(-1, -1),
    ///     Err(Error::ZeroVector)
    /// )
    /// ```
    fn add(self, other: Self) -> Result<Self, Error> {
        Self::try_from(self.x + other.x, self.y + other.y)
    }
}

impl Mul<Length> for Dir2D {
    type Output = Point2D;
    /// Multiply this `Dir2D` with a `Length` to get a `Point2D`.
    ///
    /// ```rust
    /// use anvil::{Dir2D, IntoLength, point};
    ///
    /// let dir2 = Dir2D::try_from(1., 0.).unwrap();
    /// assert_eq!(
    ///     dir2 * 2.m(),
    ///     point!(2.m(), 0.m())
    /// )
    /// ```
    fn mul(self, other: Length) -> Point2D {
        Point2D {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

/// Macro for simplifying `Dir2D` and `Dir3D` construction for static values.
///
/// ```rust
/// use anvil::{dir, Dir2D, Dir3D};
///
/// // For Dir2D
/// assert_eq!(dir!(3, 4), Dir2D::try_from(3., 4.).unwrap());
/// assert_eq!(dir!(3., 4.), Dir2D::try_from(3., 4.).unwrap());
/// // dir!(0, 0); <- this raises a compile error
///
/// // For Dir3D
/// assert_eq!(dir!(3, 4, 5), Dir3D::try_from(3., 4., 5.).unwrap());
/// assert_eq!(dir!(3., 4., 5.), Dir3D::try_from(3., 4., 5.).unwrap());
/// // dir!(0, 0, 0); // <- this raises a compile error
/// ```
#[macro_export]
macro_rules! dir {
    ( 0., 0. ) => {
        compile_error!("At least one value of the Dir needs to be non-zero.")
    };
    ( 0, 0. ) => {
        compile_error!("At least one value of the Dir needs to be non-zero.")
    };
    ( 0., 0 ) => {
        compile_error!("At least one value of the Dir needs to be non-zero.")
    };
    ( 0, 0 ) => {
        compile_error!("At least one value of the Dir needs to be non-zero.")
    };
    ( $x:literal, $y:literal ) => {
        $crate::Dir2D::try_from($x as f64, $y as f64)
            .expect("macro already checked for zero values")
    };

    ( 0., 0., 0. ) => {
        compile_error!("At least one value of the Dir needs to be non-zero.")
    };
    ( 0, 0., 0. ) => {
        compile_error!("At least one value of the Dir needs to be non-zero.")
    };
    ( 0, 0, 0. ) => {
        compile_error!("At least one value of the Dir needs to be non-zero.")
    };
    ( 0, 0., 0 ) => {
        compile_error!("At least one value of the Dir needs to be non-zero.")
    };
    ( 0., 0, 0. ) => {
        compile_error!("At least one value of the Dir needs to be non-zero.")
    };
    ( 0., 0, 0 ) => {
        compile_error!("At least one value of the Dir needs to be non-zero.")
    };
    ( 0., 0., 0 ) => {
        compile_error!("At least one value of the Dir needs to be non-zero.")
    };
    ( 0, 0, 0 ) => {
        compile_error!("At least one value of the Dir needs to be non-zero.")
    };
    ( $x:literal, $y:literal, $z:literal ) => {
        $crate::Dir3D::try_from($x as f64, $y as f64, $z as f64)
            .expect("macro already checked for zero values")
    };
}
