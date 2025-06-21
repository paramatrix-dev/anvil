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
    ///
    /// let circle = Circle::from_radius(1.m());
    /// assert_relative_eq!(circle.area(), f64::consts::PI);
    /// assert_eq!(circle.center(), Ok(Point::<2>::origin()));
    /// ```
    pub fn from_radius(radius: Length) -> Sketch {
        Path::at(Point::<2>::new([radius * -1., Length::zero()]))
            .arc_points(
                Point::<2>::new([Length::zero(), radius]),
                Point::<2>::new([radius, Length::zero()]),
            )
            .arc_points(
                Point::<2>::new([Length::zero(), radius * -1.]),
                Point::<2>::new([radius * -1., Length::zero()]),
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
    ///
    /// let circle = Circle::from_diameter(2.m());
    /// assert_relative_eq!(circle.area(), f64::consts::PI);
    /// assert_eq!(circle.center(), Ok(Point::<2>::origin()));
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
