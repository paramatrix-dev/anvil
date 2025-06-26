use std::ops::{Add, Div, Mul, Sub};

use approx::{AbsDiffEq, RelativeEq};
use cxx::UniquePtr;
use iter_fixed::IntoIteratorFixed;
use opencascade_sys::ffi;
use uom::si::length::meter;

use crate::{Dir, Error, Length, Plane};

/// A location in space.
///
/// `Point`s can be two- or three-dimensional.
/// ```rust
/// use anvil::{IntoLength, Point};
///
/// let two_dimensional_point = Point::<2>::new([1.m(), 2.m()]);
/// let three_dimensional_point = Point::<3>::new([1.m(), 2.m(), 3.m()]);
/// ```
///
/// The `point!` macro can be used to simplify point construction.
/// ```rust
/// use anvil::{IntoLength, Point, point};
///
/// assert_eq!(
///     point!(1.m(), 2.m()),
///     Point::<2>::new([1.m(), 2.m()])
/// );
/// assert_eq!(
///     point!(1.m(), 2.m(), 3.m()),
///     Point::<3>::new([1.m(), 2.m(), 3.m()])
/// );
/// ```
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Point<const DIM: usize>([Length; DIM]);
impl<const DIM: usize> Point<DIM> {
    /// Construct a `Point` from its coordinate `Length`s.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Point};
    ///
    /// // for 2D
    /// let point2d = Point::<2>::new([1.m(), 2.m()]);
    /// assert_eq!(point2d.x(), 1.m());
    /// assert_eq!(point2d.y(), 2.m());
    ///
    /// // for 3D
    /// let point3d = Point::<3>::new([1.m(), 2.m(), 3.m()]);
    /// assert_eq!(point3d.x(), 1.m());
    /// assert_eq!(point3d.y(), 2.m());
    /// assert_eq!(point3d.z(), 3.m());
    /// ```
    pub fn new(coordinates: [Length; DIM]) -> Self {
        Self(coordinates)
    }

    /// The origin point with all coordinates equal to zero.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Point};
    ///
    /// // for 2D
    /// let point2d = Point::<2>::origin();
    /// assert_eq!(point2d.x(), 0.m());
    /// assert_eq!(point2d.y(), 0.m());
    ///
    /// // for 3D
    /// let point3d = Point::<3>::origin();
    /// assert_eq!(point3d.x(), 0.m());
    /// assert_eq!(point3d.y(), 0.m());
    /// assert_eq!(point3d.z(), 0.m());
    /// ```
    pub fn origin() -> Self {
        Self([Length::new::<meter>(0.); DIM])
    }

    /// Return the absolute distance between this `Point` to another.
    ///
    /// ```rust
    /// use core::f64;
    /// use anvil::{IntoLength, point};
    ///
    /// // for 2D
    /// let point2 = point!(3.m(), 4.m());
    /// assert_eq!(point2.distance_to(point!(0, 0)), 5.m());
    ///
    /// // for 3D
    /// let point3 = point!(3.m(), 0.m(), 4.m());
    /// assert_eq!(point3.distance_to(point!(0, 0, 0)), 5.m());
    /// ```
    pub fn distance_to(&self, other: Self) -> Length {
        Length::new::<meter>(f64::sqrt(
            (*self - other)
                .0
                .iter()
                .map(|n| n.get::<meter>().powi(2))
                .sum(),
        ))
    }

    /// Return the direction this `Point` lies in with respect to another point.
    pub fn direction_from(&self, other: Self) -> Result<Dir<DIM>, Error> {
        Dir::<DIM>::try_from(
            (*self - other)
                .0
                .into_iter_fixed()
                .map(|n| n.get::<meter>())
                .collect(),
        )
    }
}

impl Point<2> {
    /// Return the distance of the `Point<2>` to the origin on the x-axis.
    pub fn x(&self) -> Length {
        self.0[0]
    }
    /// Return the distance of the `Point<2>` to the origin on the y-axis.
    pub fn y(&self) -> Length {
        self.0[1]
    }

    /// Return the global position of this `Point<2>` given the `Plane` it is located on.
    pub fn to_3d(&self, plane: Plane) -> Point<3> {
        plane.origin() + plane.x() * self.x() + plane.y() * self.y()
    }
}

impl Point<3> {
    /// Return the distance of the `Point<3>` to the origin on the x-axis.
    pub fn x(&self) -> Length {
        self.0[0]
    }
    /// Return the distance of the `Point<3>` to the origin on the y-axis.
    pub fn y(&self) -> Length {
        self.0[1]
    }
    /// Return the distance of the `Point<3>` to the origin on the z-axis.
    pub fn z(&self) -> Length {
        self.0[2]
    }

    pub(crate) fn to_occt_point(self) -> UniquePtr<ffi::gp_Pnt> {
        ffi::new_point(
            self.x().get::<meter>(),
            self.y().get::<meter>(),
            self.z().get::<meter>(),
        )
    }
    pub(crate) fn to_occt_vec(self) -> UniquePtr<ffi::gp_Vec> {
        ffi::new_vec(
            self.x().get::<meter>(),
            self.y().get::<meter>(),
            self.z().get::<meter>(),
        )
    }
}

impl<const DIM: usize> Add<Self> for Point<DIM> {
    type Output = Self;
    /// Add another `Point` of the same dimension to this one.
    ///
    /// ```rust
    /// use anvil::{IntoLength, point};
    ///
    /// // for 2d
    /// assert_eq!(
    ///     point!(1.m(), 2.m()) + point!(4.m(), 5.m()),
    ///     point!(5.m(), 7.m())
    /// );
    ///
    /// // for 3d
    /// assert_eq!(
    ///     point!(1.m(), 2.m(), 3.m()) + point!(4.m(), 5.m(), 6.m()),
    ///     point!(5.m(), 7.m(), 9.m())
    /// );
    /// ```
    fn add(self, other: Self) -> Self {
        Self(
            self.0
                .into_iter_fixed()
                .zip(other.0)
                .map(|(a, b)| a + b)
                .collect(),
        )
    }
}

impl<const DIM: usize> Sub<Self> for Point<DIM> {
    type Output = Self;
    /// Subtract another `Point` of the same dimension from this one.
    ///
    /// ```rust
    /// use anvil::{IntoLength, point};
    ///
    /// // for 2d
    /// assert_eq!(
    ///     point!(4.m(), 5.m()) - point!(1.m(), 2.m()),
    ///     point!(3.m(), 3.m())
    /// );
    ///
    /// // for 3d
    /// assert_eq!(
    ///     point!(4.m(), 5.m(), 6.m()) - point!(1.m(), 2.m(), 3.m()),
    ///     point!(3.m(), 3.m(), 3.m())
    /// );
    /// ```
    fn sub(self, other: Self) -> Self {
        Self(
            self.0
                .into_iter_fixed()
                .zip(other.0)
                .map(|(a, b)| a - b)
                .collect(),
        )
    }
}

impl<const DIM: usize> Mul<f64> for Point<DIM> {
    type Output = Self;
    /// Multiply this `Point` with a scalar.
    ///
    /// ```rust
    /// use anvil::{IntoLength, point};
    ///
    /// // for 2d
    /// assert_eq!(
    ///     point!(1.m(), 2.m()) * 3.,
    ///     point!(3.m(), 6.m())
    /// );
    ///
    /// // for 3d
    /// assert_eq!(
    ///     point!(1.m(), 2.m(), 3.m()) * 4.,
    ///     point!(4.m(), 8.m(), 12.m())
    /// );
    /// ```
    fn mul(self, other: f64) -> Self {
        Self(self.0.into_iter_fixed().map(|n| n * other).collect())
    }
}

impl<const DIM: usize> Mul<Point<DIM>> for f64 {
    type Output = Point<DIM>;
    /// Multiply this scalar with a `Point`.
    ///
    /// ```rust
    /// use anvil::{IntoLength, point};
    ///
    /// // for 2d
    /// assert_eq!(
    ///     3. * point!(1.m(), 2.m()),
    ///     point!(3.m(), 6.m())
    /// );
    ///
    /// // for 3d
    /// assert_eq!(
    ///     4. * point!(1.m(), 2.m(), 3.m()),
    ///     point!(4.m(), 8.m(), 12.m())
    /// );
    /// ```
    fn mul(self, other: Point<DIM>) -> Point<DIM> {
        other * self
    }
}

impl<const DIM: usize> Div<f64> for Point<DIM> {
    type Output = Self;
    /// Divide this `Point` by a scalar.
    ///
    /// ```rust
    /// use anvil::{IntoLength, point};
    ///
    /// // for 2d
    /// assert_eq!(
    ///     point!(3.m(), 6.m()) / 3.,
    ///     point!(1.m(), 2.m())
    /// );
    ///
    /// // for 3d
    /// assert_eq!(
    ///     point!(4.m(), 8.m(), 12.m()) / 4.,
    ///     point!(1.m(), 2.m(), 3.m())
    /// );
    /// ```
    fn div(self, other: f64) -> Self {
        Self(self.0.into_iter_fixed().map(|n| n / other).collect())
    }
}

impl<const DIM: usize> AbsDiffEq for Point<DIM> {
    type Epsilon = f64;
    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(a, b)| a.get::<meter>().abs_diff_eq(&b.get::<meter>(), epsilon))
    }
}

impl<const DIM: usize> RelativeEq for Point<DIM> {
    fn default_max_relative() -> Self::Epsilon {
        f64::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(a, b)| {
            a.get::<meter>()
                .relative_eq(&b.get::<meter>(), epsilon, max_relative)
        })
    }
}

/// Macro for simplifying `Point` construction for static values.
///
/// # Examples
/// ```rust
/// use anvil::{IntoLength, point, Point};
///
/// // construct a Point<2> from two `Length` values
/// assert_eq!(
///     point!(3.m(), 4.cm()),
///     Point::<2>::new([3.m(), 4.cm()])
/// );
/// assert_eq!(point!(0, 0), Point::<2>::origin());
///
/// // construct a Point<3> from three `Length` values
/// assert_eq!(
///     point!(3.m(), 4.cm(), 5.yd()),
///     Point::<3>::new([3.m(), 4.cm(), 5.yd()])
/// );
/// assert_eq!(point!(0, 0, 0), Point::<3>::origin());
/// ```
#[macro_export]
macro_rules! point {
    (0, 0) => {
        $crate::Point::<2>::origin()
    };
    ($x:expr, $y:expr) => {
        $crate::Point::<2>::new([$x, $y])
    };

    (0, 0, 0) => {
        $crate::Point::<3>::origin()
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::Point::<3>::new([$x, $y, $z])
    };
}
