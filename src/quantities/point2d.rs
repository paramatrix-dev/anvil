use std::ops::{Add, Div, Mul, Sub};

use crate::{Error, Length};

use super::{Dir2D, Plane, Point3D};

/// A location in two-dimensional space.
#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Point2D {
    /// Distance of the `Point2D` to the origin on the x-axis.
    pub x: Length,

    /// Distance of the `Point2D` to the origin on the y-axis.
    pub y: Length,
}
impl Point2D {
    /// The origin point at the position x=0 and y=0.
    pub fn origin() -> Self {
        Self {
            x: Length::from_m(0.),
            y: Length::from_m(0.),
        }
    }

    /// Construct a `Point2D` from its component lengths.
    pub fn new(x: Length, y: Length) -> Self {
        Point2D { x, y }
    }

    /// Return the absolute distance between this `Point2D` and the origin point.
    ///
    /// # Example
    /// ```rust
    /// use core::f64;
    /// use anvil::{IntoLength, point};
    ///
    /// let point = point!(1.m(), 1.m());
    /// assert_eq!(point.distance_to_origin(), f64::sqrt(2.).m())
    /// ```
    pub fn distance_to_origin(&self) -> Length {
        Length::from_m(f64::sqrt(
            f64::powi(self.x.m(), 2) + f64::powi(self.y.m(), 2),
        ))
    }

    /// Return the direction this point lies in with respect to another point.
    ///
    /// ```rust
    /// use anvil::{Dir2D, Error, IntoLength, point};
    ///
    /// let p = point!(1.m(), 1.m());
    /// assert_eq!(p.direction_from(point!(0, 0)), Dir2D::try_from(1., 1.));
    /// assert_eq!(p.direction_from(p), Err(Error::ZeroVector));
    /// ```
    pub fn direction_from(&self, other: Self) -> Result<Dir2D, Error> {
        Dir2D::try_from((self.x - other.x).m(), (self.y - other.y).m())
    }

    /// Return the global position of this `Point2D` given the `Plane` it is located on.
    pub fn to_3d(&self, plane: Plane) -> Point3D {
        plane.origin() + plane.x() * self.x + plane.y() * self.y
    }
}

impl Default for Point2D {
    fn default() -> Self {
        Self::origin()
    }
}

impl Add<Point2D> for Point2D {
    type Output = Point2D;
    fn add(self, other: Point2D) -> Point2D {
        Point2D::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub<Point2D> for Point2D {
    type Output = Point2D;
    fn sub(self, other: Point2D) -> Point2D {
        Point2D::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f64> for Point2D {
    type Output = Point2D;
    fn mul(self, other: f64) -> Point2D {
        Point2D::new(self.x * other, self.y * other)
    }
}
impl Mul<Point2D> for f64 {
    type Output = Point2D;
    fn mul(self, other: Point2D) -> Point2D {
        other * self
    }
}

impl Div<f64> for Point2D {
    type Output = Point2D;
    fn div(self, other: f64) -> Point2D {
        Point2D::new(self.x / other, self.y / other)
    }
}

/// Macro for simplifying `Point2D` and `Point3D` construction for static values.
///
/// # Examples
/// ```rust
/// use anvil::{IntoLength, Length, point, Point2D, Point3D};
///
/// // construct a Point2D from two length values
/// assert_eq!(
///     point!(3.m(), 4.cm()),
///     Point2D::new(Length::from_m(3.), Length::from_cm(4.))
/// );
/// assert_eq!(point!(0, 0), Point2D::origin());
///
/// // construct a Point3D from three length values
/// assert_eq!(
///     point!(3.m(), 4.cm(), 5.yd()),
///     Point3D::new(Length::from_m(3.), Length::from_cm(4.), Length::from_yd(5.))
/// );
/// assert_eq!(point!(0, 0, 0), Point3D::origin());
/// ```
#[macro_export]
macro_rules! point {
    (0, 0) => {
        $crate::Point2D::origin()
    };
    ($x:expr, $y:expr) => {
        $crate::Point2D::new($x, $y)
    };

    (0, 0, 0) => {
        $crate::Point3D::origin()
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::Point3D::new($x, $y, $z)
    };
}

#[cfg(test)]
mod tests {
    use crate::{Dir3D, IntoLength, point};

    use super::*;

    #[test]
    fn add() {
        let point1 = point!(1.m(), 2.m());
        let point2 = point!(4.m(), 5.m());

        assert_eq!(point1 + point2, point!(5.m(), 7.m()));
    }

    #[test]
    fn substract() {
        let point1 = point!(1.m(), 2.m());
        let point2 = point!(4.m(), 5.m());

        assert_eq!(point2 - point1, point!(3.m(), 3.m()));
    }

    #[test]
    fn multiply() {
        assert_eq!(point!(1.m(), 2.m()) * 4., point!(4.m(), 8.m()));
        assert_eq!(4. * point!(1.m(), 2.m()), point!(4.m(), 8.m()));
    }

    #[test]
    fn divide() {
        assert_eq!(point!(4.m(), 8.m()) / 4., point!(1.m(), 2.m()));
    }

    #[test]
    fn to_3d_origin() {
        let plane = Plane::new(
            point!(1.m(), 2.m(), 3.m()),
            Dir3D::try_from(1., 1., 0.).unwrap(),
            Dir3D::try_from(0., 0., 1.).unwrap(),
        )
        .unwrap();
        let point = Point2D::origin();

        assert_eq!(point.to_3d(plane), plane.origin());
    }

    #[test]
    fn to_3d_straight_plane() {
        let plane = Plane::xy();
        let point = point!(1.m(), 2.m());

        assert_eq!(point.to_3d(plane), point!(1.m(), 2.m(), 0.m()));
    }

    #[test]
    fn to_3d_different_point() {
        let plane = Plane::new(
            Point3D::origin(),
            Dir3D::try_from(1., 0., -1.).unwrap(),
            Dir3D::try_from(0., 1., 0.).unwrap(),
        )
        .unwrap();
        let point = point!(f64::sqrt(2.).mm(), 5.mm());

        let right = point!(1.mm(), 5.mm(), -1.mm());
        assert!((point.to_3d(plane).x.m() - right.x.m()).abs() < 1e-9);
        assert!((point.to_3d(plane).y.m() - right.y.m()).abs() < 1e-9);
        assert!((point.to_3d(plane).z.m() - right.z.m()).abs() < 1e-9);
    }
}
