use std::ops::{Add, Div, Mul, Sub};

use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::{Error, IntoLength, Length};

use super::Dir3D;

/// A location in three-dimensional space.
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Point3D {
    /// Distance of the `Point2D` to the origin on the x-axis.
    pub x: Length,

    /// Distance of the `Point2D` to the origin on the y-axis.
    pub y: Length,

    /// Distance of the `Point2D` to the origin on the z-axis.
    pub z: Length,
}
impl Point3D {
    /// The origin point at the position x=0, y=0, z=0.
    pub fn origin() -> Self {
        Self::new(0.m(), 0.m(), 0.m())
    }

    /// Construct a `Point3D` from its component lengths.
    pub fn new(x: Length, y: Length, z: Length) -> Self {
        Point3D { x, y, z }
    }

    /// Return the absolute distance between this `Point3D` and the origin point.
    ///
    /// # Example
    /// ```rust
    /// use core::f64;
    /// use anvil::{IntoLength, point};
    ///
    /// let point = point!(0.m(), 1.m(), 1.m());
    /// assert_eq!(point.distance_to_origin(), f64::sqrt(2.).m())
    /// ```
    pub fn distance_to_origin(&self) -> Length {
        Length::from_m(f64::sqrt(
            f64::powi(self.x.m(), 2) + f64::powi(self.y.m(), 2) + f64::powi(self.z.m(), 2),
        ))
    }

    /// Return the direction this point lies in with respect to another point.
    ///
    /// ```rust
    /// use anvil::{Dir3D, Error, IntoLength, point};
    ///
    /// let p = point!(1.m(), 1.m(), 1.m());
    /// assert_eq!(p.direction_from(point!(0, 0, 0)), Dir3D::try_from(1., 1., 1.));
    /// assert_eq!(p.direction_from(p), Err(Error::ZeroVector));
    /// ```
    pub fn direction_from(&self, other: Point3D) -> Result<Dir3D, Error> {
        Dir3D::try_from(
            (self.x - other.x).m(),
            (self.y - other.y).m(),
            (self.z - other.z).m(),
        )
    }

    pub(crate) fn to_occt_point(self) -> UniquePtr<ffi::gp_Pnt> {
        ffi::new_point(self.x.m(), self.y.m(), self.z.m())
    }
    pub(crate) fn to_occt_vec(self) -> UniquePtr<ffi::gp_Vec> {
        ffi::new_vec(self.x.m(), self.y.m(), self.z.m())
    }
}

impl Add<Point3D> for Point3D {
    type Output = Point3D;
    fn add(self, other: Point3D) -> Point3D {
        Point3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Point3D> for Point3D {
    type Output = Point3D;
    fn sub(self, other: Point3D) -> Point3D {
        Point3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Point3D {
    type Output = Point3D;
    fn mul(self, other: f64) -> Point3D {
        Point3D::new(self.x * other, self.y * other, self.z * other)
    }
}
impl Mul<Point3D> for f64 {
    type Output = Point3D;
    fn mul(self, other: Point3D) -> Point3D {
        other * self
    }
}

impl Div<f64> for Point3D {
    type Output = Point3D;
    fn div(self, other: f64) -> Point3D {
        Point3D::new(self.x / other, self.y / other, self.z / other)
    }
}

#[cfg(test)]
mod tests {
    use crate::{IntoLength, point};

    #[test]
    fn add() {
        let point1 = point!(1.m(), 2.m(), 3.m());
        let point2 = point!(4.m(), 5.m(), 6.m());

        assert_eq!(point1 + point2, point!(5.m(), 7.m(), 9.m()));
    }

    #[test]
    fn substract() {
        let point1 = point!(1.m(), 2.m(), 3.m());
        let point2 = point!(4.m(), 5.m(), 6.m());

        assert_eq!(point2 - point1, point!(3.m(), 3.m(), 3.m()));
    }

    #[test]
    fn multiply() {
        assert_eq!(
            point!(1.m(), 2.m(), 3.m()) * 4.,
            point!(4.m(), 8.m(), 12.m())
        );
        assert_eq!(
            4. * point!(1.m(), 2.m(), 3.m()),
            point!(4.m(), 8.m(), 12.m())
        );
    }

    #[test]
    fn divide() {
        assert_eq!(
            point!(4.m(), 8.m(), 12.m()) / 4.,
            point!(1.m(), 2.m(), 3.m())
        );
    }
}
