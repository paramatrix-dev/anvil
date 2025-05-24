use crate::{Length, Path, Point2D, Sketch};

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
    /// use anvil::{length, Point2D, Rectangle};
    ///
    /// let rect = Rectangle::from_dim(length!(1 m), length!(1 m));
    /// assert_eq!(rect.area(), 1.);
    /// assert_eq!(rect.center(), Ok(Point2D::origin()));
    /// ```
    pub fn from_dim(x: Length, y: Length) -> Sketch {
        let corner1 = Point2D {
            x: x * -0.5,
            y: y * -0.5,
        };
        let corner2 = Point2D {
            x: x * 0.5,
            y: y * 0.5,
        };
        Self::from_corners(corner1, corner2)
    }

    /// Construct a rectangular `Sketch` from its corner locations.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Point2D, Rectangle};
    ///
    /// let rect = Rectangle::from_corners(Point2D::origin(), Point2D::from_m(2., 2.));
    /// assert_eq!(rect.area(), 4.);
    /// ```
    pub fn from_corners(corner1: Point2D, corner2: Point2D) -> Sketch {
        if corner1.x == corner2.x || corner1.y == corner2.y {
            return Sketch::empty();
        }
        Path::at(corner1)
            .line_to(Point2D::new(corner2.x, corner1.y))
            .line_to(corner2)
            .line_to(Point2D::new(corner1.x, corner2.y))
            .close()
    }

    /// Construct a centered rectangular `Sketch` directly from the x and y meter values.
    ///
    /// This function is primarily intended to simplify tests and should not be exptected in
    /// similar structs.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{length, Rectangle};
    ///
    /// assert_eq!(
    ///     Rectangle::from_m(1., 2.),
    ///     Rectangle::from_dim(length!(1 m), length!(2 m))
    /// )
    /// ```
    pub fn from_m(x: f64, y: f64) -> Sketch {
        Self::from_dim(Length::from_m(x), Length::from_m(y))
    }

    /// Construct a centered rectangular `Sketch` directly from the x and y millimeter values.
    ///
    /// This function is primarily intended to simplify tests and should not be exptected in
    /// similar structs.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{length, Rectangle};
    ///
    /// assert_eq!(
    ///     Rectangle::from_mm(1., 2.),
    ///     Rectangle::from_dim(length!(1 mm), length!(2 mm))
    /// )
    /// ```
    pub fn from_mm(x: f64, y: f64) -> Sketch {
        Self::from_dim(Length::from_mm(x), Length::from_mm(y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::length;

    #[test]
    fn from_dim_empty() {
        assert_eq!(
            Rectangle::from_dim(length!(0), length!(1 m)),
            Sketch::empty()
        );
        assert_eq!(
            Rectangle::from_dim(length!(1 m), length!(0)),
            Sketch::empty()
        );
    }

    #[test]
    fn from_corners_empty() {
        assert_eq!(
            Rectangle::from_corners(Point2D::from_m(1., 2.), Point2D::from_m(1., 4.)),
            Sketch::empty()
        );
        assert_eq!(
            Rectangle::from_corners(Point2D::from_m(1., 2.), Point2D::from_m(3., 2.)),
            Sketch::empty()
        );
    }
}
