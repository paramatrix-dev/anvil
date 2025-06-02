use std::ops::{Add, Div, Mul, Sub};

use iter_fixed::IntoIteratorFixed;

use crate::{Length, Plane};

/// A location in space.
///
/// `Point`s can be two- or three-dimensional.
/// ```rust
/// use anvil::{IntoLength, Point};
///
/// let two_dimensional_point = Point::<2>::new(1.m(), 2.m());
/// let three_dimensional_point = Point::<3>::new(1.m(), 2.m(), 3.m());
/// ```
///
/// The point! macro can be used to simplify point construction.
/// ```rust
/// use anvil::{IntoLength, Point, pointRENAME};
///
/// assert_eq!(
///     pointRENAME!(1.m(), 2.m()),
///     Point::<2>::new(1.m(), 2.m())
/// );
/// assert_eq!(
///     pointRENAME!(1.m(), 2.m(), 3.m()),
///     Point::<3>::new(1.m(), 2.m(), 3.m())
/// );
/// ```
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Point<const DIM: usize>([Length; DIM]);
impl<const DIM: usize> Point<DIM> {
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
    /// assert_eq!(point3d.y(), 0.m());
    /// ```
    pub fn origin() -> Self {
        Self([Length::zero(); DIM])
    }

    /// Return the absolute distance between this `Point` to another.
    ///
    /// ```rust
    /// use core::f64;
    /// use anvil::{IntoLength, pointRENAME};
    ///
    /// // for 2D
    /// let point2 = pointRENAME!(3.m(), 4.m());
    /// assert_eq!(point2.distance_to(pointRENAME!(0, 0)), 5.m());
    ///
    /// // for 3D
    /// let point3 = pointRENAME!(3.m(), 0.m(), 4.m());
    /// assert_eq!(point3.distance_to(pointRENAME!(0, 0, 0)), 5.m());
    /// ```
    pub fn distance_to(&self, other: Self) -> Length {
        Length::from_m(f64::sqrt(
            (*self - other).0.iter().map(|n| n.m().powi(2)).sum(),
        ))
    }

    /// Return the direction this `Point` lies in with respect to another point.
    pub fn direction_from(&self, other: Self) {
        todo!()
    }
}

impl Point<2> {
    /// Construct a `Point<2>` from its component lengths.
    pub fn new(x: Length, y: Length) -> Self {
        Point::<2>([x, y])
    }

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
        todo!()
    }
}

impl Point<3> {
    /// Construct a `Point<3>` from its component lengths.
    pub fn new(x: Length, y: Length, z: Length) -> Self {
        Point::<3>([x, y, z])
    }

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
}

impl<const DIM: usize> Add<Self> for Point<DIM> {
    type Output = Self;
    /// Add another `Point` of the same dimension to this one.
    ///
    /// ```rust
    /// use anvil::{IntoLength, pointRENAME};
    ///
    /// // for 2d
    /// assert_eq!(
    ///     pointRENAME!(1.m(), 2.m()) + pointRENAME!(4.m(), 5.m()),
    ///     pointRENAME!(5.m(), 7.m())
    /// );
    ///
    /// // for 3d
    /// assert_eq!(
    ///     pointRENAME!(1.m(), 2.m(), 3.m()) + pointRENAME!(4.m(), 5.m(), 6.m()),
    ///     pointRENAME!(5.m(), 7.m(), 9.m())
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
    /// use anvil::{IntoLength, pointRENAME};
    ///
    /// // for 2d
    /// assert_eq!(
    ///     pointRENAME!(4.m(), 5.m()) - pointRENAME!(1.m(), 2.m()),
    ///     pointRENAME!(3.m(), 3.m())
    /// );
    ///
    /// // for 3d
    /// assert_eq!(
    ///     pointRENAME!(4.m(), 5.m(), 6.m()) - pointRENAME!(1.m(), 2.m(), 3.m()),
    ///     pointRENAME!(3.m(), 3.m(), 3.m())
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
    /// use anvil::{IntoLength, pointRENAME};
    ///
    /// // for 2d
    /// assert_eq!(
    ///     pointRENAME!(1.m(), 2.m()) * 3.,
    ///     pointRENAME!(3.m(), 6.m())
    /// );
    ///
    /// // for 3d
    /// assert_eq!(
    ///     pointRENAME!(1.m(), 2.m(), 3.m()) * 4.,
    ///     pointRENAME!(4.m(), 8.m(), 12.m())
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
    /// use anvil::{IntoLength, pointRENAME};
    ///
    /// // for 2d
    /// assert_eq!(
    ///     3. * pointRENAME!(1.m(), 2.m()),
    ///     pointRENAME!(3.m(), 6.m())
    /// );
    ///
    /// // for 3d
    /// assert_eq!(
    ///     4. * pointRENAME!(1.m(), 2.m(), 3.m()),
    ///     pointRENAME!(4.m(), 8.m(), 12.m())
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
    /// use anvil::{IntoLength, pointRENAME};
    ///
    /// // for 2d
    /// assert_eq!(
    ///     pointRENAME!(3.m(), 6.m()) / 3.,
    ///     pointRENAME!(1.m(), 2.m())
    /// );
    ///
    /// // for 3d
    /// assert_eq!(
    ///     pointRENAME!(4.m(), 8.m(), 12.m()) / 4.,
    ///     pointRENAME!(1.m(), 2.m(), 3.m())
    /// );
    /// ```
    fn div(self, other: f64) -> Self {
        Self(self.0.into_iter_fixed().map(|n| n / other).collect())
    }
}

/// Macro for simplifying `Point` construction for static values.
///
/// # Examples
/// ```rust
/// use anvil::{IntoLength, pointRENAME, Point};
///
/// // construct a Point<2> from two `Length` values
/// assert_eq!(
///     pointRENAME!(3.m(), 4.cm()),
///     Point::<2>::new(3.m(), 4.cm())
/// );
/// assert_eq!(pointRENAME!(0, 0), Point::<2>::origin());
///
/// // construct a Point<3> from three `Length` values
/// assert_eq!(
///     pointRENAME!(3.m(), 4.cm(), 5.yd()),
///     Point::<3>::new(3.m(), 4.cm(), 5.yd())
/// );
/// assert_eq!(pointRENAME!(0, 0, 0), Point::<3>::origin());
/// ```
#[macro_export]
macro_rules! pointRENAME {
    (0, 0) => {
        $crate::Point::<2>::origin()
    };
    ($x:expr, $y:expr) => {
        $crate::Point::<2>::new($x, $y)
    };

    (0, 0, 0) => {
        $crate::Point::<3>::origin()
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::Point::<3>::new($x, $y, $z)
    };
}
