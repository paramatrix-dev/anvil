use crate::Error;

use super::{Dir2D, Length, Point2D};

/// An axis in 2D space.
#[derive(Debug, PartialEq, Clone)]
pub struct Axis2D {
    /// A point contained in the axis.
    pub origin: Point2D,
    /// The directional vector of the axis.
    pub direction: Dir2D,
}
impl Axis2D {
    /// Construct an `Axis2D`.
    pub fn new(origin: Point2D, direction: Dir2D) -> Self {
        Self { origin, direction }
    }

    /// Construct an `Axis2D` that lies between two points.
    ///
    /// This constructor can return an error if the two points are at the same location.
    ///
    /// ```rust
    /// use anvil::{Axis2D, point, Dir2D};
    ///
    /// assert_eq!(
    ///     Axis2D::between(point!(1 m, 1 m), point!(2 m, 1 m)),
    ///     Ok(Axis2D {
    ///         origin: point!(1 m, 1 m),
    ///         direction: Dir2D::try_from(1., 0.).expect("")
    ///     })
    /// );
    /// assert!(Axis2D::between(point!(1 m, 1 m), point!(1 m, 1 m)).is_err())
    /// ```
    pub fn between(origin: Point2D, other: Point2D) -> Result<Self, Error> {
        let direction = other.direction_from(&origin)?;
        Ok(Self { origin, direction })
    }

    /// Return the axis identical to the x-axis at the origin.
    pub fn x() -> Self {
        Axis2D::new(Point2D::origin(), Dir2D::try_from(1., 0.).expect(""))
    }
    /// Return the axis identical to the y-axis at the origin.
    pub fn y() -> Self {
        Axis2D::new(Point2D::origin(), Dir2D::try_from(0., 1.).expect(""))
    }
    /// Return the axis identical to the x-axis at the origin in reverse direction.
    pub fn neg_x() -> Self {
        Axis2D::new(Point2D::origin(), Dir2D::try_from(-1., 0.).expect(""))
    }
    /// Return the axis identical to the y-axis at the origin in reverse direction.
    pub fn neg_y() -> Self {
        Axis2D::new(Point2D::origin(), Dir2D::try_from(0., -1.).expect(""))
    }

    /// Return a point on the `Axis2D` at a specified distance from the `Axis2D` origin.
    ///
    /// ```rust
    /// use anvil::{Axis2D, length, point};
    ///
    /// let axis = Axis2D::x();
    /// assert_eq!(
    ///     axis.point_at(&length!(5 m)),
    ///     point!(5 m, 0 m),
    /// )
    /// ```
    pub fn point_at(&self, distance: &Length) -> Point2D {
        self.origin + self.direction * distance
    }
}
