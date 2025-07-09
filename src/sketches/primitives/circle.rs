use uom::si::length::meter;

use crate::{Length, Path, Point, Sketch};

/// Builder for a circular `Sketch`.
///
/// While the `Circle` struct itself is not used, its constructor methods like
/// `Circle::from_radius()` can be used to build this primitive `Sketch`.
#[derive(Debug, PartialEq, Clone)]
pub struct Circle;
impl Circle {
    /// Construct a centered circular `Sketch` from a given radius.
    ///
    /// # Example
    /// ```rust
    /// use core::f64;
    /// use anvil::{Circle, IntoLength, Point};
    /// use approx::assert_relative_eq;
    /// use uom::si::area::square_meter;
    /// use uom::si::f64::Area;
    ///
    /// let circle = Circle::from_radius(1.m());
    /// assert_eq!(circle.center(), Ok(Point::<2>::origin()));
    /// assert_relative_eq!(
    ///     circle.area().value,
    ///     Area::new::<square_meter>(f64::consts::PI).value
    /// );
    /// ```
    pub fn from_radius(radius: Length) -> Sketch {
        Path::at(Point::<2>::new([radius * -1., Length::new::<meter>(0.)]))
            .arc_points(
                Point::<2>::new([Length::new::<meter>(0.), radius]),
                Point::<2>::new([radius, Length::new::<meter>(0.)]),
            )
            .arc_points(
                Point::<2>::new([Length::new::<meter>(0.), radius * -1.]),
                Point::<2>::new([radius * -1., Length::new::<meter>(0.)]),
            )
            .close()
    }

    /// Construct a centered circular `Sketch` from a given diameter.
    ///
    /// # Example
    /// ```rust
    /// use core::f64;
    /// use anvil::{Circle, IntoLength, Point};
    /// use approx::assert_relative_eq;
    /// use uom::si::area::square_meter;
    /// use uom::si::f64::Area;
    ///
    /// let circle = Circle::from_diameter(2.m());
    /// assert_eq!(circle.center(), Ok(Point::<2>::origin()));
    /// assert_relative_eq!(
    ///     circle.area().value,
    ///     Area::new::<square_meter>(f64::consts::PI).value
    /// );
    /// ```
    pub fn from_diameter(diameter: Length) -> Sketch {
        Self::from_radius(diameter / 2.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IntoLength;

    #[test]
    fn from_radius_empty() {
        assert_eq!(Circle::from_radius(0.m()), Sketch::empty())
    }

    #[test]
    fn from_diameter_empty() {
        assert_eq!(Circle::from_diameter(0.m()), Sketch::empty())
    }
}
