use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::{Dir, Error, Length, Point, dirRENAME, pointRENAME};

/// An axis in space.
///
/// Axes can be two- or three-dimensional.
/// ```rust
/// use anvil::{Axis, IntoLength, dirRENAME, pointRENAME};
///
/// let two_dimensional_axis = Axis::<2>::new(pointRENAME!(1.m(), 2.m()), dirRENAME!(3, 4));
/// let three_dimensional_axis = Axis::<3>::new(pointRENAME!(1.m(), 2.m(), 3.m()), dirRENAME!(4, 5, 6));
/// ```
///
/// Axes can also be constructed from tuples containing a `Point` and a `Dir`, simplifying dimensionality.
/// ```rust
/// use anvil::{Axis, IntoLength, dirRENAME, pointRENAME};
///
/// assert_eq!(
///     Axis::<2>::new(pointRENAME!(1.m(), 2.m()), dirRENAME!(3, 4)),
///     (pointRENAME!(1.m(), 2.m()), dirRENAME!(3, 4)).into(),
/// );
/// assert_eq!(
///     Axis::<3>::new(pointRENAME!(1.m(), 2.m(), 3.m()), dirRENAME!(4, 5, 6)),
///     (pointRENAME!(1.m(), 2.m(), 3.m()), dirRENAME!(4, 5, 6)).into(),
/// );
/// ```
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Axis<const DIM: usize> {
    /// A `Point` contained in the `Axis`.
    pub origin: Point<DIM>,
    /// The `Dir` this `Axis` points to.
    pub direction: Dir<DIM>,
}
impl<const DIM: usize> Axis<DIM> {
    /// Construct an `Axis`.
    pub fn new(origin: Point<DIM>, direction: Dir<DIM>) -> Self {
        Self { origin, direction }
    }

    /// Construct an `Axis` that lies between two `Point`s.
    ///
    /// The first point is taken as the axis origin.
    ///
    /// ```rust
    /// use anvil::{Axis, Error, IntoLength, dirRENAME, pointRENAME};
    ///
    /// // for 2d
    /// assert_eq!(
    ///     Axis::<2>::between(pointRENAME!(1.m(), 1.m()), pointRENAME!(2.m(), 1.m())),
    ///     Ok(Axis::<2>::new(pointRENAME!(1.m(), 1.m()), dirRENAME!(1, 0)))
    /// );
    /// assert_eq!(
    ///     Axis::<2>::between(pointRENAME!(1.m(), 1.m()), pointRENAME!(1.m(), 1.m())),
    ///     Err(Error::ZeroVector)
    /// );
    ///
    /// // for 2d
    /// assert_eq!(
    ///     Axis::<3>::between(pointRENAME!(1.m(), 1.m(), 1.m()), pointRENAME!(2.m(), 1.m(), 1.m())),
    ///     Ok(Axis::<3>::new(pointRENAME!(1.m(), 1.m(), 1.m()), dirRENAME!(1, 0, 0)))
    /// );
    /// assert_eq!(
    ///     Axis::<3>::between(pointRENAME!(1.m(), 1.m(), 1.m()), pointRENAME!(1.m(), 1.m(), 1.m())),
    ///     Err(Error::ZeroVector)
    /// );
    /// ```
    pub fn between(origin: Point<DIM>, other: Point<DIM>) -> Result<Self, Error> {
        let direction = other.direction_from(origin)?;
        Ok(Self { origin, direction })
    }

    /// Return a `Point` on the `Axis` at a specified distance its origin.
    ///
    /// ```rust
    /// use anvil::{Axis, IntoLength, dirRENAME, pointRENAME};
    ///
    /// // for 2d
    /// let axis = Axis::<2>::new(pointRENAME!(1.m(), 2.m()), dirRENAME!(1, 0));
    /// assert_eq!(
    ///     axis.point_at(5.m()),
    ///     pointRENAME!(6.m(), 2.m()),
    /// );
    ///
    /// // for 3d
    /// let axis = Axis::<3>::new(pointRENAME!(1.m(), 2.m(), 3.m()), dirRENAME!(1, 0, 0));
    /// assert_eq!(
    ///     axis.point_at(5.m()),
    ///     pointRENAME!(6.m(), 2.m(), 3.m()),
    /// );
    /// ```
    pub fn point_at(&self, distance: Length) -> Point<DIM> {
        self.origin + self.direction * distance
    }
}
impl<const DIM: usize> From<(Point<DIM>, Dir<DIM>)> for Axis<DIM> {
    fn from((origin, direction): (Point<DIM>, Dir<DIM>)) -> Self {
        Axis::new(origin, direction)
    }
}
impl<const DIM: usize> From<(Dir<DIM>, Point<DIM>)> for Axis<DIM> {
    fn from((direction, origin): (Dir<DIM>, Point<DIM>)) -> Self {
        Axis::new(origin, direction)
    }
}

impl Axis<2> {
    /// Return the `Axis<2>` identical to the x-axis at the origin.
    pub fn x() -> Self {
        Self::new(pointRENAME!(0, 0), dirRENAME!(1, 0))
    }
    /// Return the `Axis<2>` identical to the y-axis at the origin.
    pub fn y() -> Self {
        Self::new(pointRENAME!(0, 0), dirRENAME!(0, 1))
    }
    /// Return the `Axis<2>` identical to the x-axis at the origin in reverse direction.
    pub fn neg_x() -> Self {
        Self::new(pointRENAME!(0, 0), dirRENAME!(-1, 0))
    }
    /// Return the `Axis<2>` identical to the y-axis at the origin in reverse direction.
    pub fn neg_y() -> Self {
        Self::new(pointRENAME!(0, 0), dirRENAME!(0, -1))
    }

    /// Return the intersection `Point` of this `Axis<2>` with another.
    ///
    /// If the two axes are parallel, None is returned.
    ///
    /// ```rust
    /// use anvil::{Axis, IntoLength, dirRENAME, pointRENAME};
    ///
    /// let axis1 = Axis::<2>::new(pointRENAME!(0, 0), dirRENAME!(1, 1));
    /// let axis2 = Axis::<2>::new(pointRENAME!(1.m(), 5.m()), dirRENAME!(0, 1));
    /// assert_eq!(axis1.intersect(axis2), Some(pointRENAME!(1.m(), 1.m())));
    /// assert_eq!(axis1.intersect(axis1), None);
    /// ```
    pub fn intersect(&self, other: Self) -> Option<Point<2>> {
        // TODO: generalize for all dimensions
        let determinant =
            self.direction.x() * other.direction.y() - self.direction.y() * other.direction.x();

        let lines_are_parallel = determinant.abs() < 1e-9;
        if lines_are_parallel {
            return None;
        }

        let diff = other.origin - self.origin;
        let offset =
            (diff.x() * other.direction.y() - diff.y() * other.direction.x()) / determinant;

        Some(self.origin + offset * self.direction)
    }
}

impl Axis<3> {
    /// Return the `Axis<3>` identical to the x-axis at the origin.
    pub fn x() -> Self {
        Self::new(pointRENAME!(0, 0, 0), dirRENAME!(1, 0, 0))
    }
    /// Return the `Axis<3>` identical to the y-axis at the origin.
    pub fn y() -> Self {
        Self::new(pointRENAME!(0, 0, 0), dirRENAME!(0, 1, 0))
    }
    /// Return the `Axis<3>` identical to the z-axis at the origin.
    pub fn z() -> Self {
        Self::new(pointRENAME!(0, 0, 0), dirRENAME!(0, 0, 1))
    }
    /// Return the `Axis<3>` identical to the x-axis at the origin in reverse direction.
    pub fn neg_x() -> Self {
        Self::new(pointRENAME!(0, 0, 0), dirRENAME!(-1, 0, 0))
    }
    /// Return the `Axis<3>` identical to the y-axis at the origin in reverse direction.
    pub fn neg_y() -> Self {
        Self::new(pointRENAME!(0, 0, 0), dirRENAME!(0, -1, 0))
    }
    /// Return the `Axis<3>` identical to the z-axis at the origin in reverse direction.
    pub fn neg_z() -> Self {
        Self::new(pointRENAME!(0, 0, 0), dirRENAME!(0, 0, -1))
    }

    pub(crate) fn to_occt_ax1(self) -> UniquePtr<ffi::gp_Ax1> {
        ffi::gp_Ax1_ctor(&self.origin.to_occt_point(), &self.direction.to_occt_dir())
    }
}
