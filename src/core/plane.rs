use crate::{Axis, Dir, Error, Point, dir, point};

/// A 2D plane in 3D space.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Plane(Point<3>, Dir<3>, Dir<3>);
impl Plane {
    /// Construct the `Plane` spaned by the x and y axes.
    pub fn xy() -> Self {
        Self(point!(0, 0, 0), dir!(1, 0, 0), dir!(0, 1, 0))
    }
    /// Construct the `Plane` spaned by the x and z axes.
    pub fn xz() -> Self {
        Self(point!(0, 0, 0), dir!(1, 0, 0), dir!(0, 0, 1))
    }
    /// Construct the `Plane` spaned by the y and z axes.
    pub fn yz() -> Self {
        Self(point!(0, 0, 0), dir!(0, 1, 0), dir!(0, 0, 1))
    }

    /// Construct a `Plane` from a point and two orthogonal vectors.
    ///
    /// `x_axis` defines the direction of the x-axis inside the plane. `y_axis` defines the
    /// direction of the y-axis inside the plane. Both are used to project from the local 2D
    /// coordinate system to the global coordinate system. If the two axes are not orthogonal,
    /// an `Err(Error::VectorsNotOrthogonal)` is returned.
    pub fn new(origin: Point<3>, x_dir: Dir<3>, y_dir: Dir<3>) -> Result<Self, Error> {
        let axes_are_orthogonal = x_dir.dot(y_dir) < 1e-9;
        if !axes_are_orthogonal {
            return Err(Error::VectorsNotOrthogonal(x_dir, y_dir));
        }
        Ok(Self(origin, x_dir, y_dir))
    }

    /// Return the origin point of this `Plane`.
    pub fn origin(&self) -> Point<3> {
        self.0
    }
    /// Return the direction of the x-axis of this `Plane`.
    pub fn x(&self) -> Dir<3> {
        self.1
    }
    /// Return the x-axis of this `Plane`.
    pub fn x_axis(&self) -> Axis<3> {
        (self.origin(), self.x()).into()
    }
    /// Return the direction of the y-axis of this `Plane`.
    pub fn y(&self) -> Dir<3> {
        self.2
    }
    /// Return the y-axis of this `Plane`.
    pub fn y_axis(&self) -> Axis<3> {
        (self.origin(), self.y()).into()
    }
    /// Return the `Dir3D` that is orthogonal to this plane.
    pub fn normal(&self) -> Dir<3> {
        self.x().cross(self.y())
    }
    /// Return the `Axis::<3>` that is orthogonal to this plane and crosses its origin.
    pub fn normal_axis(&self) -> Axis<3> {
        (self.origin(), self.normal()).into()
    }
}
