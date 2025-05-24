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
        Self::from_mm(0., 0.)
    }

    /// Construct a `Point2D` from its component lengths.
    pub fn new(x: Length, y: Length) -> Self {
        Point2D { x, y }
    }
    /// Construct a `Point2D` from the millimeter length values directly.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{length, Point2D};
    ///
    /// let point = Point2D::from_mm(1., 2.);
    /// assert_eq!(point.x, length!(1 mm));
    /// assert_eq!(point.y, length!(2 mm));
    /// ```
    pub fn from_mm(x: f64, y: f64) -> Self {
        Point2D {
            x: Length::from_mm(x),
            y: Length::from_mm(y),
        }
    }
    /// Construct a `Point2D` from the meter length values directly.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{length, Point2D};
    ///
    /// let point = Point2D::from_m(1., 2.);
    /// assert_eq!(point.x, length!(1 m));
    /// assert_eq!(point.y, length!(2 m));
    /// ```
    pub fn from_m(x: f64, y: f64) -> Self {
        Point2D {
            x: Length::from_m(x),
            y: Length::from_m(y),
        }
    }

    /// Return the absolute distance between this `Point2D` and the origin point.
    ///
    /// # Example
    /// ```rust
    /// use core::f64;
    /// use anvil::{Length, Point2D};
    ///
    /// let point = Point2D::from_m(1., 1.);
    /// assert_eq!(point.distance_to_origin(), Length::from_m(f64::sqrt(2.)))
    /// ```
    pub fn distance_to_origin(&self) -> Length {
        Length::from_m(f64::sqrt(
            f64::powi(self.x.m(), 2) + f64::powi(self.y.m(), 2),
        ))
    }

    /// Return the direction this point lies in with respect to another point.
    ///
    /// ```rust
    /// use anvil::{Dir2D, Error, point, Point2D};
    ///
    /// let p = point!(1 m, 1 m);
    /// assert_eq!(p.direction_from(&Point2D::origin()), Dir2D::try_from(1., 1.));
    /// assert_eq!(p.direction_from(&p), Err(Error::ZeroVector));
    /// ```
    pub fn direction_from(&self, other: &Self) -> Result<Dir2D, Error> {
        Dir2D::try_from((self.x - other.x).m(), (self.y - other.y).m())
    }

    /// Return the global position of this `Point2D` given the `Plane` it is located on.
    pub fn to_3d(&self, plane: &Plane) -> Point3D {
        plane.origin() + plane.x_axis() * self.x + plane.y_axis() * self.y
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
/// use anvil::{length, Length, point, Point2D, Point3D};
///
/// // Construct a Point2D from two length values
/// assert_eq!(
///     point!(1 m, 2 m),
///     Point2D::new(Length::from_m(1.), Length::from_m(2.))
/// );
/// assert_eq!(
///     point!(1 cm, 2.1 mm),
///     Point2D::new(Length::from_cm(1.), Length::from_mm(2.1))
/// );
///
/// // Construct a Point2D from three length values
/// assert_eq!(
///     point!(1 m, 2 m, 3 m),
///     Point3D::new(Length::from_m(1.), Length::from_m(2.), Length::from_m(3.))
/// );
///
/// // Use explicit expressions to construct a Point2D
/// assert_eq!(
///     point!(length!(1 cm), 2.1 mm),
///     Point2D::new(Length::from_cm(1.), Length::from_mm(2.1))
/// );
/// assert_eq!(
///     point!(1 cm, length!(2.1 mm)),
///     Point2D::new(Length::from_cm(1.), Length::from_mm(2.1))
/// );
/// assert_eq!(
///     point!(length!(1 cm), length!(2.1 mm)),
///     Point2D::new(Length::from_cm(1.), Length::from_mm(2.1))
/// );
///
/// // Use explicit expressions to construct a Point2D
/// assert_eq!(
///     point!(length!(1 m), 2 m, 3 m),
///     Point3D::new(Length::from_m(1.), Length::from_m(2.), Length::from_m(3.))
/// );
/// assert_eq!(
///     point!(1 m, length!(2 m), 3 m),
///     Point3D::new(Length::from_m(1.), Length::from_m(2.), Length::from_m(3.))
/// );
/// assert_eq!(
///     point!(1 m, 2 m, length!(3 m)),
///     Point3D::new(Length::from_m(1.), Length::from_m(2.), Length::from_m(3.))
/// );
/// ```
#[macro_export]
macro_rules! point {
    ($x:literal $x_unit:ident, $y:literal $y_unit:ident) => {
        anvil::Point2D::new(anvil::length!($x $x_unit), anvil::length!($y $y_unit))
    };
    ($x:expr, $y:literal $y_unit:ident) => {
        anvil::Point2D::new($x, anvil::length!($y $y_unit))
    };
    ($x:literal $x_unit:ident, $y:expr) => {
        anvil::Point2D::new(anvil::length!($x $x_unit), $y)
    };
    ($x:expr, $y:expr) => {
        anvil::Point2D::new($x, $y)
    };

    ($x:literal $x_unit:ident, $y:literal $y_unit:ident, $z:literal $z_unit:ident) => {
        anvil::Point3D::new(anvil::length!($x $x_unit), anvil::length!($y $y_unit), anvil::length!($z $z_unit))
    };
    ($x:expr, $y:literal $y_unit:ident, $z:literal $z_unit:ident) => {
        anvil::Point3D::new($x, anvil::length!($y $y_unit), anvil::length!($z $z_unit))
    };
    ($x:literal $x_unit:ident, $y:expr, $z:literal $z_unit:ident) => {
        anvil::Point3D::new(anvil::length!($x $x_unit), $y, anvil::length!($z $z_unit))
    };
    ($x:literal $x_unit:ident, $y:literal $y_unit:ident, $z:expr) => {
        anvil::Point3D::new(anvil::length!($x $x_unit), anvil::length!($y $y_unit), $z)
    };
}

#[cfg(test)]
mod tests {
    use crate::Dir3D;

    use super::*;

    #[test]
    fn add() {
        let point1 = Point2D::from_m(1., 2.);
        let point2 = Point2D::from_m(4., 5.);

        assert_eq!(point1 + point2, Point2D::from_m(5., 7.));
    }

    #[test]
    fn substract() {
        let point1 = Point2D::from_m(1., 2.);
        let point2 = Point2D::from_m(4., 5.);

        assert_eq!(point2 - point1, Point2D::from_m(3., 3.));
    }

    #[test]
    fn multiply() {
        assert_eq!(Point2D::from_m(1., 2.) * 4., Point2D::from_m(4., 8.));
        assert_eq!(4. * Point2D::from_m(1., 2.), Point2D::from_m(4., 8.));
    }

    #[test]
    fn divide() {
        assert_eq!(Point2D::from_m(4., 8.) / 4., Point2D::from_m(1., 2.));
    }

    #[test]
    fn to_3d_origin() {
        let plane = Plane::new(
            Point3D::from_m(1., 2., 3.),
            Dir3D::try_from(1., 1., 0.).unwrap(),
            Dir3D::try_from(0., 0., 1.).unwrap(),
        )
        .unwrap();
        let point = Point2D::origin();

        assert_eq!(point.to_3d(&plane), plane.origin());
    }

    #[test]
    fn to_3d_straight_plane() {
        let plane = Plane::xy();
        let point = Point2D::from_m(1., 2.);

        assert_eq!(point.to_3d(&plane), Point3D::from_m(1., 2., 0.));
    }

    #[test]
    fn to_3d_different_point() {
        let plane = Plane::new(
            Point3D::origin(),
            Dir3D::try_from(1., 0., -1.).unwrap(),
            Dir3D::try_from(0., 1., 0.).unwrap(),
        )
        .unwrap();
        let point = Point2D::from_mm(f64::sqrt(2.), 5.);

        let right = Point3D::from_mm(1., 5., -1.);
        assert!((point.to_3d(&plane).x.m() - right.x.m()).abs() < 1e-9);
        assert!((point.to_3d(&plane).y.m() - right.y.m()).abs() < 1e-9);
        assert!((point.to_3d(&plane).z.m() - right.z.m()).abs() < 1e-9);
    }
}
