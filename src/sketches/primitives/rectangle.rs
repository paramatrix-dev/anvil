use crate::{Length, Path, Point, Sketch, point};

/// Builder for a rectangular `Sketch`.
///
/// While the `Rectangle` struct itself is not used, its constructor methods like
/// `Rectangle::from_dim()` can be used to build this primitive `Sketch`.
#[derive(Debug, PartialEq, Clone)]
pub struct Rectangle;
impl Rectangle {
    /// Construct a centered rectangular `Sketch` from the x and y dimensions.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{IntoLength, Rectangle, point};
    /// use approx::assert_relative_eq;
    /// use uom::si::area::square_meter;
    /// use uom::si::f64::Area;
    ///
    /// let rect = Rectangle::from_dim(1.m(), 1.m());
    /// assert_eq!(rect.center(), Ok(point!(0, 0)));
    /// assert_relative_eq!(
    ///     rect.area().value,
    ///     Area::new::<square_meter>(1.).value
    /// );
    /// ```
    pub fn from_dim(x: Length, y: Length) -> Sketch {
        Self::from_corners(point!(x * -0.5, y * -0.5), point!(x * 0.5, y * 0.5))
    }

    /// Construct a rectangular `Sketch` from its corner locations.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{IntoLength, Rectangle, point};
    /// use approx::assert_relative_eq;
    /// use uom::si::area::square_meter;
    /// use uom::si::f64::Area;
    ///
    /// let rect = Rectangle::from_corners(point!(0, 0), point!(2.m(), 2.m()));
    /// assert_relative_eq!(
    ///     rect.area().value,
    ///     Area::new::<square_meter>(4.).value
    /// );
    /// ```
    pub fn from_corners(corner1: Point<2>, corner2: Point<2>) -> Sketch {
        if corner1.x() == corner2.x() || corner1.y() == corner2.y() {
            return Sketch::empty();
        }
        Path::at(corner1)
            .line_to(Point::<2>::new([corner2.x(), corner1.y()]))
            .line_to(corner2)
            .line_to(Point::<2>::new([corner1.x(), corner2.y()]))
            .close()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntoLength, point};

    #[test]
    fn from_dim_empty() {
        assert_eq!(Rectangle::from_dim(0.m(), 1.m()), Sketch::empty());
        assert_eq!(Rectangle::from_dim(1.m(), 0.m()), Sketch::empty());
    }

    #[test]
    fn from_corners_empty() {
        assert_eq!(
            Rectangle::from_corners(point!(1.m(), 2.m()), point!(1.m(), 4.m())),
            Sketch::empty()
        );
        assert_eq!(
            Rectangle::from_corners(point!(1.m(), 2.m()), point!(3.m(), 2.m())),
            Sketch::empty()
        );
    }
}
