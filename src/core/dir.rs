use std::ops::{Add, Mul, Sub};

use cxx::UniquePtr;
use iter_fixed::IntoIteratorFixed;
use opencascade_sys::ffi;

use crate::{Angle, Error, Length, Point};

/// A direction in space with a length of 1.
///
/// `Dir`s can be two- or three-dimensional.
/// ```rust
/// use anvil::Dir;
///
/// let two_dimensional_dir = Dir::<2>::try_from([1., 2.]);
/// let three_dimensional_dir = Dir::<3>::try_from([1., 2., 3.]);
/// ```
///
/// The `dir!` macro can be used to simplify point construction.
/// ```rust
/// use anvil::{Dir, dir};
///
/// assert_eq!(dir!(3, 4), Dir::try_from([3., 4.]).unwrap());
/// assert_eq!(dir!(3, 4, 5), Dir::try_from([3., 4., 5.]).unwrap());
/// ```
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Dir<const DIM: usize>([f64; DIM]);
impl<const DIM: usize> Dir<DIM> {
    /// Construct a `Dir` from the directional components.
    ///
    /// Returns an Error::ZeroVector if all of the axis values are zero.
    ///
    /// ```rust
    /// use anvil::{Dir, Error};
    ///
    /// // for 2d
    /// let dir2 = Dir::<2>::try_from([3., 4.]).unwrap();
    /// assert_eq!(dir2.x(), 3. / 5.);
    /// assert_eq!(dir2.y(), 4. / 5.);
    /// assert_eq!(Dir::<2>::try_from([0., 0.]), Err(Error::ZeroVector));
    ///
    /// // for 3d
    /// let dir3 = Dir::<3>::try_from([3., 0., 4.]).unwrap();
    /// assert_eq!(dir3.x(), 3. / 5.);
    /// assert_eq!(dir3.y(), 0. / 5.);
    /// assert_eq!(dir3.z(), 4. / 5.);
    /// assert_eq!(Dir::<3>::try_from([0., 0., 0.]), Err(Error::ZeroVector));
    /// ```
    pub fn try_from(components: [f64; DIM]) -> Result<Self, Error> {
        let magnitude = f64::sqrt(components.iter().map(|n| n.powi(2)).sum());
        match magnitude {
            0. => Err(Error::ZeroVector),
            _ => Ok(Self(
                components
                    .into_iter_fixed()
                    .map(|n| n / magnitude)
                    .collect(),
            )),
        }
    }

    /// Return the dot-product of this `Dir` with another of the same dimension.
    pub fn dot(&self, other: Self) -> f64 {
        self.0.into_iter().zip(other.0).map(|(a, b)| a * b).sum()
    }

    /// Return true if this `Dir` has less than a 0.000001% difference to another.
    ///
    /// ```rust
    /// use anvil::dir;
    ///
    /// assert!(dir!(1, 1).approx_eq(dir!(1.00000001, 1)));
    /// assert!(!dir!(1, 1).approx_eq(dir!(0.5, 1)));
    /// ```
    pub fn approx_eq(&self, other: Dir<DIM>) -> bool {
        for (s, o) in self.0.iter().zip(other.0) {
            if (s / o - 1.).abs() > 0.0000001 {
                return false;
            }
        }
        true
    }
}

impl Dir<2> {
    /// Return the x-component of this `Dir<2>`.
    pub fn x(&self) -> f64 {
        self.0[0]
    }
    /// Return the y-component of this `Dir<2>`.
    pub fn y(&self) -> f64 {
        self.0[1]
    }

    /// Return the `Angle` this `Dir<2>` points to in relation to the unit circle.
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
        let angle = Angle::from_rad(self.y().atan2(self.x()));
        if angle.rad() < 0. {
            Angle::from_rad(angle.rad() + std::f64::consts::TAU)
        } else {
            angle
        }
    }

    /// Return a `Dir<2>` rotated by a specified amount counter clockwise.
    pub fn rotate(&self, angle: Angle) -> Self {
        Self::from(self.angle() + angle)
    }
}

impl From<Angle> for Dir<2> {
    /// Construct a `Dir<2>` from an `Angle`.
    ///
    /// An angle of 0 points in the positive x-direction and positive angles rotate counter
    /// clockwise.
    fn from(value: Angle) -> Self {
        Self([f64::cos(value.rad()), f64::sin(value.rad())])
    }
}

impl Dir<3> {
    /// Return the x-component of this `Dir<3>`.
    pub fn x(&self) -> f64 {
        self.0[0]
    }
    /// Return the y-component of this `Dir<3>`.
    pub fn y(&self) -> f64 {
        self.0[1]
    }
    /// Return the z-component of this `Dir<3>`.
    pub fn z(&self) -> f64 {
        self.0[2]
    }

    /// Return the cross-product of this `Dir<3>` with another.
    pub fn cross(&self, other: Self) -> Self {
        Self([
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        ])
    }

    pub(crate) fn to_occt_dir(self) -> UniquePtr<ffi::gp_Dir> {
        ffi::gp_Dir_ctor(self.x(), self.y(), self.z())
    }
}

impl<const DIM: usize> Add<Self> for Dir<DIM> {
    type Output = Result<Self, Error>;
    /// Add another `Dir` to this one.
    ///
    /// ```rust
    /// use anvil::{dir, Error};
    ///
    /// // for 2d
    /// assert_eq!(
    ///     dir!(0, 1) + dir!(1, 0),
    ///     Ok(dir!(1, 1))
    /// );
    /// assert_eq!(
    ///     dir!(1, 1) + dir!(-1, -1),
    ///     Err(Error::ZeroVector)
    /// );
    ///
    /// // for 3d
    /// assert_eq!(
    ///     dir!(0, 1, 0) + dir!(1, 0, 0),
    ///     Ok(dir!(1, 1, 0))
    /// );
    /// assert_eq!(
    ///     dir!(1, 1, 1) + dir!(-1, -1, -1),
    ///     Err(Error::ZeroVector)
    /// );
    /// ```
    fn add(self, other: Self) -> Result<Self, Error> {
        Self::try_from(
            self.0
                .into_iter_fixed()
                .zip(other.0)
                .map(|(a, b)| a + b)
                .collect(),
        )
    }
}

impl<const DIM: usize> Sub<Self> for Dir<DIM> {
    type Output = Result<Self, Error>;
    /// Subtract another `Dir` from this one.
    ///
    /// ```rust
    /// use anvil::{dir, Error};
    ///
    /// // for 2d
    /// assert_eq!(
    ///     dir!(0, 1) - dir!(1, 0),
    ///     Ok(dir!(-1, 1))
    /// );
    /// assert_eq!(
    ///     dir!(1, 1) - dir!(1, 1),
    ///     Err(Error::ZeroVector)
    /// );
    ///
    /// // for 3d
    /// assert_eq!(
    ///     dir!(0, 1, 0) - dir!(1, 0, 0),
    ///     Ok(dir!(-1, 1, 0))
    /// );
    /// assert_eq!(
    ///     dir!(1, 1, 1) - dir!(1, 1, 1),
    ///     Err(Error::ZeroVector)
    /// );
    /// ```
    fn sub(self, other: Self) -> Result<Self, Error> {
        Self::try_from(
            self.0
                .into_iter_fixed()
                .zip(other.0)
                .map(|(a, b)| a - b)
                .collect(),
        )
    }
}

impl<const DIM: usize> Mul<Length> for Dir<DIM> {
    type Output = Point<DIM>;
    /// Multiply this `Dir` with a `Length` to get a `Point`.
    ///
    /// ```rust
    /// use anvil::{IntoLength, dir, point};
    ///
    /// // for 2d
    /// assert_eq!(
    ///     dir!(1, 0) * 2.m(),
    ///     point!(2.m(), 0.m())
    /// );
    ///
    /// // for 3d
    /// assert_eq!(
    ///     dir!(1, 0, 0) * 2.m(),
    ///     point!(2.m(), 0.m(), 0.m())
    /// );
    /// ```
    fn mul(self, other: Length) -> Point<DIM> {
        Point::new(self.0.into_iter_fixed().map(|n| n * other).collect())
    }
}

/// Macro for simplifying `Dir` construction for static values.
///
/// ```rust
/// use anvil::{dir, Dir};
///
/// // for 2d
/// assert_eq!(dir!(3, 4), Dir::try_from([3., 4.]).unwrap());
/// assert_eq!(dir!(3., 4.), Dir::try_from([3., 4.]).unwrap());
/// // dir!(0, 0); <- this raises a compile error
///
/// // for 3d
/// assert_eq!(dir!(3, 4, 5), Dir::try_from([3., 4., 5.]).unwrap());
/// assert_eq!(dir!(3., 4., 5.), Dir::try_from([3., 4., 5.]).unwrap());
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
        $crate::Dir::try_from([$x as f64, $y as f64])
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
        $crate::Dir::try_from([$x as f64, $y as f64, $z as f64])
            .expect("macro already checked for zero values")
    };
}
