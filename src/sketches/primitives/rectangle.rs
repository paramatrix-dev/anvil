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
    ///
    /// let rect = Rectangle::from_dim(1.m(), 1.m());
    /// assert_eq!(rect.area(), 1.);
    /// assert_eq!(rect.center(), Ok(point!(0, 0)));
    /// ```
    pub fn from_dim(x: Length, y: Length) -> Sketch {
        Self::from_corners(point!(x * -0.5, y * -0.5), point!(x * 0.5, y * 0.5))
    }

    /// Construct a rectangular `Sketch` from its corner locations.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{IntoLength, Rectangle, point};
    ///
    /// let rect = Rectangle::from_corners(point!(0, 0), point!(2.m(), 2.m()));
    /// assert_eq!(rect.area(), 4.);
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

    /// Construct a centered rectangular `Sketch` directly from the x and y meter values.
    ///
    /// This function is primarily intended to simplify tests and should not be exptected in
    /// similar structs.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{IntoLength, Rectangle};
    ///
    /// assert_eq!(
    ///     Rectangle::from_m(1., 2.),
    ///     Rectangle::from_dim(1.m(), 2.m())
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
    /// use anvil::{IntoLength, Rectangle};
    ///
    /// assert_eq!(
    ///     Rectangle::from_mm(1., 2.),
    ///     Rectangle::from_dim(1.mm(), 2.mm())
    /// )
    /// ```
    pub fn from_mm(x: f64, y: f64) -> Sketch {
        Self::from_dim(Length::from_mm(x), Length::from_mm(y))
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
