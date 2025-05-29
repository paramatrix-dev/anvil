use crate::{Angle, Axis2D, Dir2D, Length, Point2D};

use super::{Edge, Sketch};

/// A continuous series of edges (i.e. lines, arcs, ...).
#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    cursor: Point2D,
    edges: Vec<Edge>,
}
impl Path {
    /// Construct an empty `Path` at a given starting point.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Path, Point2D};
    ///
    /// let path = Path::at(Point2D::from_m(1., 2.));
    /// assert_eq!(path.start(), Point2D::from_m(1., 2.))
    /// ```
    pub fn at(start: Point2D) -> Self {
        Self {
            cursor: start,
            edges: vec![],
        }
    }

    /// Add a line to the end of this `Path` that ends at a specified point.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Path, Point2D};
    ///
    /// let path = Path::at(Point2D::from_m(1., 2.)).line_to(Point2D::from_m(3., 4.));
    /// assert_eq!(path.end(), Point2D::from_m(3., 4.))
    /// ```
    pub fn line_to(&self, point: Point2D) -> Self {
        self.add_edge(Edge::Line(self.cursor, point))
    }

    /// Add a line to the end of this `Path` that extends by a specified amount in x and y direction.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{length, Path, point};
    ///
    /// let path = Path::at(point!(1 m, 2 m)).line_by(length!(3 m), length!(4 m));
    /// assert_eq!(path.end(), point!(4 m, 6 m))
    /// ```
    pub fn line_by(&self, dx: Length, dy: Length) -> Self {
        self.add_edge(Edge::Line(self.cursor, self.cursor + Point2D::new(dx, dy)))
    }

    /// Append a circle section to this `Path` that curves the Path by a certain angle.
    ///
    /// A positive radius curves the path to the left and a negative radius to the right. A positive
    /// angle ensures the path is smooth while a negative angle creates a sharp corner in the other
    /// direction.
    ///
    /// If the path was empty before, the arc starts in positive x-direction.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{angle, Circle, Edge, length, Path, point, Rectangle, Plane};
    ///
    /// let sketch = Path::at(point!(1 m, 1 m))
    ///     .arc_by(length!(-1 m), angle!(180 deg))
    ///     .line_by(length!(-2 m), length!(0))
    ///     .arc_by(length!(-1 m), angle!(30 deg))
    ///     .arc_by(length!(-1 m), angle!(150 deg)) // arcs can be split into sections
    ///     .close();
    ///
    /// assert_eq!(
    ///     sketch,
    ///     Rectangle::from_dim(length!(2 m), length!(2 m))
    ///         .add(&Circle::from_radius(length!(1 m)).move_to(point!(1 m, 0 m)))
    ///         .add(&Circle::from_radius(length!(1 m)).move_to(point!(-1 m, 0 m)))
    /// )
    /// ```
    pub fn arc_by(&self, radius: Length, angle: Angle) -> Self {
        if radius == Length::zero() || angle == Angle::zero() {
            return self.clone();
        }
        let center = self.cursor + self.end_direction().rotate(Angle::from_deg(90.)) * radius;
        let center_cursor_axis =
            Axis2D::between(center, self.cursor).expect("zero radius already checked");
        let direction_factor = radius / radius.abs();

        let interim_point = center
            + center_cursor_axis
                .direction
                .rotate(angle / 2. * direction_factor)
                * radius.abs();
        let end_point = center
            + center_cursor_axis
                .direction
                .rotate(angle * direction_factor)
                * radius.abs();

        self.add_edge(Edge::Arc(self.cursor, interim_point, end_point))
    }

    /// Add a circle section to the end of this `Path` two points.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Edge, Path, point};
    ///
    /// let path = Path::at(point!(0 m, 0 m)).arc_points(point!(1 m, 1 m), point!(0 m, 2 m));
    /// assert_eq!(path.cursor(), point!(0 m, 2 m));
    /// assert_eq!(path.edges(), vec![Edge::Arc(point!(0 m, 0 m), point!(1 m, 1 m), point!(0 m, 2 m))]);
    /// ```
    pub fn arc_points(&self, mid: Point2D, end: Point2D) -> Self {
        self.add_edge(Edge::Arc(self.cursor, mid, end))
    }

    /// Connect the end of this `Path` to its start with a straight line and return the resulting `Sketch`.
    pub fn close(self) -> Sketch {
        if self.start() == self.end() {
            Sketch::from_edges(self.edges)
        } else {
            Sketch::from_edges(self.line_to(self.start()).edges)
        }
    }

    /// Return the starting point of the `Path`.
    ///
    /// If the path does not have any edges, the cursor is returned.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Path, Point2D};
    ///
    /// let path = Path::at(Point2D::from_m(1., 2.)).line_to(Point2D::origin()).line_to(Point2D::from_m(3., 4.));
    /// assert_eq!(path.start(), Point2D::from_m(1., 2.));
    ///
    /// let empty_path = Path::at(Point2D::from_m(5., 6.));
    /// assert_eq!(empty_path.start(), Point2D::from_m(5., 6.));
    /// ```
    pub fn start(&self) -> Point2D {
        match self.edges.first() {
            Some(edge) => edge.start(),
            None => self.cursor,
        }
    }

    /// Return the ending point of the `Path`.
    ///
    /// If the path does not have any edges, the cursor is returned.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Path, Point2D};
    ///
    /// let path = Path::at(Point2D::from_m(1., 2.)).line_to(Point2D::origin()).line_to(Point2D::from_m(3., 4.));
    /// assert_eq!(path.end(), Point2D::from_m(3., 4.));
    ///
    /// let empty_path = Path::at(Point2D::from_m(5., 6.));
    /// assert_eq!(empty_path.end(), Point2D::from_m(5., 6.));
    /// ```
    pub fn end(&self) -> Point2D {
        match self.edges.iter().last() {
            Some(edge) => edge.end(),
            None => self.cursor,
        }
    }

    /// Return the direction the last element of this `Path` is pointing to.
    ///
    /// If the path is empty, a `Dir2D` parallel to the positive x-direction is returned.
    ///
    /// ```rust
    /// use anvil::{Path, dir, point};
    ///
    /// assert_eq!(
    ///     Path::at(point!(0 m, 0 m)).line_to(point!(0 m, 2 m)).end_direction(),
    ///     dir!(0, 1)
    /// );
    /// assert_eq!(
    ///     Path::at(point!(0 m, 0 m)).end_direction(),
    ///     dir!(1, 0)
    /// )
    /// ```
    pub fn end_direction(&self) -> Dir2D {
        match self.edges.last() {
            Some(last_edge) => last_edge
                .end_direction()
                .expect("edge has already been checked for zero length"),
            None => Dir2D::from(Angle::zero()),
        }
    }

    /// Return the edges in this `Path`.
    pub fn edges(&self) -> Vec<Edge> {
        self.edges.clone()
    }

    /// Return the current cursor position of this `Path`.
    pub fn cursor(&self) -> Point2D {
        self.cursor
    }

    fn add_edge(&self, edge: Edge) -> Self {
        if edge.start() != self.end() {
            panic!("path is not continuous");
        }

        let new_cursor = edge.end();
        let mut new_edges = self.edges.clone();
        new_edges.push(edge);

        Self {
            cursor: new_cursor,
            edges: new_edges,
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_relative_eq;

    use super::*;
    use crate::{angle, dir, length, point};

    fn assert_dir_eq(dir1: Dir2D, dir2: Dir2D) {
        assert_float_relative_eq!(dir1.x(), dir2.x());
        assert_float_relative_eq!(dir1.y(), dir2.y());
    }

    fn assert_point_eq(point1: Point2D, point2: Point2D) {
        assert_float_relative_eq!(point1.x.m(), point2.x.m());
        assert_float_relative_eq!(point1.y.m(), point2.y.m());
    }

    #[test]
    fn end_arc_positive_radius_angle() {
        let path = Path::at(Point2D::origin()).arc_by(length!(1 m), angle!(90 deg));
        assert_point_eq(path.end(), point!(1 m, 1 m))
    }

    #[test]
    fn end_arc_positive_radius_negative_angle() {
        let path = Path::at(Point2D::origin()).arc_by(length!(1 m), angle!(-90 deg));
        assert_point_eq(path.end(), point!(-1 m, 1 m))
    }

    #[test]
    fn end_arc_negative_radius_positive_angle() {
        let path = Path::at(Point2D::origin()).arc_by(length!(-1 m), angle!(90 deg));
        assert_point_eq(path.end(), point!(1 m, -1 m))
    }

    #[test]
    fn end_arc_negative_radius_angle() {
        let path = Path::at(Point2D::origin()).arc_by(length!(-1 m), angle!(-90 deg));
        assert_point_eq(path.end(), point!(-1 m, -1 m))
    }

    #[test]
    fn end_arc_negative_radius_positive_angle_45deg() {
        let path = Path::at(point!(0 m, 1 m)).arc_by(length!(-1 m), angle!(45 deg));
        assert_point_eq(
            path.end(),
            Point2D::from_m(1. / f64::sqrt(2.), 1. / f64::sqrt(2.)),
        )
    }

    #[test]
    fn end_direction_empty_path() {
        let path = Path::at(Point2D::origin());
        assert_dir_eq(path.end_direction(), dir!(1, 0))
    }

    #[test]
    fn end_direction_line() {
        let path = Path::at(Point2D::origin()).line_to(point!(1 m, 1 m));
        assert_dir_eq(path.end_direction(), dir!(1, 1))
    }

    #[test]
    fn end_direction_arc_positive_radius_angle() {
        let path = Path::at(Point2D::origin()).arc_by(length!(1 m), angle!(45 deg));
        assert_dir_eq(path.end_direction(), dir!(1, 1))
    }

    #[test]
    fn end_direction_arc_positive_radius_negative_angle() {
        let path = Path::at(Point2D::origin()).arc_by(length!(1 m), angle!(-45 deg));
        assert_dir_eq(path.end_direction(), dir!(-1, 1))
    }

    #[test]
    fn end_direction_arc_negative_radius_positive_angle() {
        let path = Path::at(Point2D::origin()).arc_by(length!(-1 m), angle!(45 deg));
        assert_dir_eq(path.end_direction(), dir!(1, -1))
    }

    #[test]
    fn end_direction_arc_negative_radius_angle() {
        let path = Path::at(Point2D::origin()).arc_by(length!(-1 m), angle!(-45 deg));
        assert_dir_eq(path.end_direction(), dir!(-1, -1))
    }
}
