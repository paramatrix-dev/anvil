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

    /// Return a `Dir2D` that is at a right angle to this `Dir2D`.
    ///
    /// ```rust
    /// use anvil::dir;
    ///
    /// assert_eq!(dir!(0, 1).orthogonal(), dir!(1, 0));
    /// assert_eq!(dir!(4, 6).orthogonal(), dir!(6, -4));
    /// ```
    pub fn orthogonal(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
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
