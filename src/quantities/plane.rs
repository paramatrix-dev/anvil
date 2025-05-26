use crate::Error;

use super::{Axis3D, Point3D, dir3d::Dir3D};

/// A 2D plane in 3D space.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Plane(Point3D, Dir3D, Dir3D);
impl Plane {
    /// Construct the `Plane` spaned by the x and y axes.
    pub fn xy() -> Self {
        Self::new(
            Point3D::origin(),
            Dir3D::try_from(1., 0., 0.).expect("zero vector defined"),
            Dir3D::try_from(0., 1., 0.).expect("zero vector defined"),
        )
        .expect("non orthogonal axes")
    }
    /// Construct the `Plane` spaned by the x and z axes.
    pub fn xz() -> Self {
        Self::new(
            Point3D::origin(),
            Dir3D::try_from(1., 0., 0.).expect("zero vector defined"),
            Dir3D::try_from(0., 0., 1.).expect("zero vector defined"),
        )
        .expect("non orthogonal axes")
    }
    /// Construct the `Plane` spaned by the y and z axes.
    pub fn yz() -> Self {
        Self::new(
            Point3D::origin(),
            Dir3D::try_from(0., 1., 0.).expect("zero vector defined"),
            Dir3D::try_from(0., 0., 1.).expect("zero vector defined"),
        )
        .expect("non orthogonal axes")
    }

    /// Construct a `Plane` from a point and two orthogonal vectors.
    ///
    /// `x_axis` defines the direction of the x-axis inside the plane. `y_axis` defines the
    /// direction of the y-axis inside the plane. Both are used to project from the local 2D
    /// coordinate system to the global coordinate system. If the two axes are not orthogonal,
    /// an `Err(Error::VectorsNotOrthogonal)` is returned.
    pub fn new(origin: Point3D, x_axis: Dir3D, y_axis: Dir3D) -> Result<Self, Error> {
        let axes_are_orthogonal = x_axis.dot(y_axis) < 1e-9;
        if !axes_are_orthogonal {
            return Err(Error::VectorsNotOrthogonal(x_axis, y_axis));
        }
        Ok(Self(origin, x_axis, y_axis))
    }

    /// Return the origin point of this `Plane`.
    pub fn origin(&self) -> Point3D {
        self.0
    }
    /// Return the direction of the x-axis of this `Plane`.
    pub fn x(&self) -> Dir3D {
        self.1
    }
    /// Return the x-axis of this `Plane`.
    pub fn x_axis(&self) -> Axis3D {
        Axis3D {
            origin: self.origin(),
            direction: self.x(),
        }
    }
    /// Return the direction of the y-axis of this `Plane`.
    pub fn y(&self) -> Dir3D {
        self.2
    }
    /// Return the y-axis of this `Plane`.
    pub fn y_axis(&self) -> Axis3D {
        Axis3D {
            origin: self.origin(),
            direction: self.y(),
        }
    }
    /// Return the `Dir3D` that is orthogonal to this plane.
    pub fn normal(&self) -> Dir3D {
        self.x().cross(self.y())
    }
    /// Return the `Axis3D` that is orthogonal to this plane and crosses its origin.
    pub fn normal_axis(&self) -> Axis3D {
        Axis3D {
            origin: self.origin(),
            direction: self.normal(),
        }
    }
}
