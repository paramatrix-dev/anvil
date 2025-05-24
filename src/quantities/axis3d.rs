use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::Error;

use super::{Dir3D, Length, Point3D};

/// An axis in 3D space.
#[derive(Debug, PartialEq, Clone)]
pub struct Axis3D {
    /// A point contained in the axis.
    pub origin: Point3D,
    /// The directional vector of the axis.
    pub direction: Dir3D,
}
impl Axis3D {
    /// Construct an `Axis3D`.
    pub fn new(origin: Point3D, direction: Dir3D) -> Self {
        Self { origin, direction }
    }

    /// Construct an `Axis3D` that lies between two points.
    ///
    /// This constructor can return an error if the two points are at the same location.
    ///
    /// ```rust
    /// use anvil::{Axis3D, point, Dir3D};
    ///
    /// assert_eq!(
    ///     Axis3D::between(point!(1 m, 1 m, 1 m), point!(2 m, 1 m, 1 m)),
    ///     Ok(Axis3D {
    ///         origin: point!(1 m, 1 m, 1 m),
    ///         direction: Dir3D::try_from(1., 0., 0.).expect("")
    ///     })
    /// );
    /// assert!(Axis3D::between(point!(1 m, 1 m, 1 m), point!(1 m, 1 m, 1 m)).is_err())
    /// ```
    pub fn between(origin: Point3D, other: Point3D) -> Result<Self, Error> {
        let direction = other.direction_from(&origin)?;
        Ok(Self { origin, direction })
    }

    /// Return the axis identical to the x-axis at the origin.
    pub fn x() -> Self {
        Axis3D::new(Point3D::origin(), Dir3D::try_from(1., 0., 0.).expect(""))
    }
    /// Return the axis identical to the y-axis at the origin.
    pub fn y() -> Self {
        Axis3D::new(Point3D::origin(), Dir3D::try_from(0., 1., 0.).expect(""))
    }
    /// Return the axis identical to the z-axis at the origin.
    pub fn z() -> Self {
        Axis3D::new(Point3D::origin(), Dir3D::try_from(0., 0., 1.).expect(""))
    }
    /// Return the axis identical to the x-axis at the origin in reverse direction.
    pub fn neg_x() -> Self {
        Axis3D::new(Point3D::origin(), Dir3D::try_from(-1., 0., 0.).expect(""))
    }
    /// Return the axis identical to the y-axis at the origin in reverse direction.
    pub fn neg_y() -> Self {
        Axis3D::new(Point3D::origin(), Dir3D::try_from(0., -1., 0.).expect(""))
    }
    /// Return the axis identical to the z-axis at the origin in reverse direction.
    pub fn neg_z() -> Self {
        Axis3D::new(Point3D::origin(), Dir3D::try_from(0., 0., -1.).expect(""))
    }

    /// Return a point on the `Axis3D` at a specified distance from the `Axis3D` origin.
    ///
    /// ```rust
    /// use anvil::{Axis3D, length, point};
    ///
    /// let axis = Axis3D::x();
    /// assert_eq!(
    ///     axis.point_at(&length!(5 m)),
    ///     point!(5 m, 0 m, 0 m),
    /// )
    /// ```
    pub fn point_at(&self, distance: &Length) -> Point3D {
        self.origin + self.direction * distance
    }

    pub(crate) fn to_occt_ax1(&self) -> UniquePtr<ffi::gp_Ax1> {
        ffi::gp_Ax1_ctor(&self.origin.to_occt_point(), &self.direction.to_occt_dir())
    }
    pub(crate) fn to_occt_ax2(&self) -> UniquePtr<ffi::gp_Ax2> {
        ffi::gp_Ax2_ctor(&self.origin.to_occt_point(), &self.direction.to_occt_dir())
    }
}
