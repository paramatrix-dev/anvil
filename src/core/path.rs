use crate::{Angle, Axis, Dir, Edge, Length, Point, Sketch};
use uom::si::length::meter;

/// A continuous series of edges (i.e. lines, arcs, ...).
#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    cursor: Point<2>,
    edges: Vec<Edge>,
}
impl Path {
    /// Construct an empty `Path` at a given starting point.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Path, point};
    ///
    /// let path = Path::at(point!(1.m(), 2.m()));
    /// assert_eq!(path.start(), point!(1.m(), 2.m()))
    /// ```
    pub fn at(start: Point<2>) -> Self {
        Self {
            cursor: start,
            edges: vec![],
        }
    }

    /// Add a line to the end of this `Path` that ends at a specified point.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Path, point};
    ///
    /// let path = Path::at(point!(1.m(), 2.m())).line_to(point!(3.m(), 4.m()));
    /// assert_eq!(path.end(), point!(3.m(), 4.m()))
    /// ```
    pub fn line_to(&self, point: Point<2>) -> Self {
        self.add_edge(Edge::Line(self.cursor, point))
    }

    /// Add a line to the end of this `Path` that extends by a specified amount in x and y direction.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Path, point};
    ///
    /// let path = Path::at(point!(1.m(), 2.m())).line_by(3.m(), 4.m());
    /// assert_eq!(path.end(), point!(4.m(), 6.m()))
    /// ```
    pub fn line_by(&self, dx: Length, dy: Length) -> Self {
        self.add_edge(Edge::Line(
            self.cursor,
            self.cursor + Point::<2>::new([dx, dy]),
        ))
    }

    /// Append a circle section to this `Path` that curves the Path by a certain angle.
    ///
    /// A positive radius curves the path to the left and a negative radius to the right. A positive
    /// angle ensures the path is smooth while a negative angle creates a sharp corner in the other
    /// direction.
    ///
    /// If the path was empty before, the arc starts in positive x-direction.
    ///
    /// ```rust
    /// use anvil::{Circle, IntoAngle, IntoLength, Edge, Path, point, Rectangle, Plane};
    ///
    /// let sketch = Path::at(point!(1.m(), 1.m()))
    ///     .arc_by(-1.m(), 180.deg())
    ///     .line_by(-2.m(), 0.m())
    ///     .arc_by(-1.m(), 30.deg())
    ///     .arc_by(-1.m(), 150.deg()) // arcs can be split into sections
    ///     .close();
    ///
    /// assert_eq!(
    ///     sketch,
    ///     Rectangle::from_dim(2.m(), 2.m())
    ///         .add(&Circle::from_radius(1.m()).move_to(point!(1.m(), 0.m())))
    ///         .add(&Circle::from_radius(1.m()).move_to(point!(-1.m(), 0.m())))
    /// )
    /// ```
    pub fn arc_by(&self, radius: Length, angle: Angle) -> Self {
        if radius == Length::new::<meter>(0.) || angle == Angle::zero() {
            return self.clone();
        }
        let center = self.cursor + self.end_direction().rotate(Angle::from_deg(90.)) * radius;
        let center_cursor_axis =
            Axis::<2>::between(center, self.cursor).expect("zero radius already checked");
        let direction_factor: f64 = (radius / radius.abs()).into();

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
    /// ```rust
    /// use anvil::{Edge, IntoLength, Path, point};
    ///
    /// let path = Path::at(point!(0, 0)).arc_points(point!(1.m(), 1.m()), point!(0.m(), 2.m()));
    /// assert_eq!(path.cursor(), point!(0.m(), 2.m()));
    /// assert_eq!(path.edges(), vec![Edge::Arc(point!(0, 0), point!(1.m(), 1.m()), point!(0.m(), 2.m()))]);
    /// ```
    pub fn arc_points(&self, mid: Point<2>, end: Point<2>) -> Self {
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
    /// ```rust
    /// use anvil::{IntoLength, Path, point};
    ///
    /// let path = Path::at(point!(1.m(), 2.m())).line_to(point!(3.m(), 4.m()));
    /// assert_eq!(path.start(), point!(1.m(), 2.m()));
    ///
    /// let empty_path = Path::at(point!(5.m(), 6.m()));
    /// assert_eq!(empty_path.start(), point!(5.m(), 6.m()));
    /// ```
    pub fn start(&self) -> Point<2> {
        match self.edges.first() {
            Some(edge) => edge.start(),
            None => self.cursor,
        }
    }

    /// Return the ending point of the `Path`.
    ///
    /// If the path does not have any edges, the cursor is returned.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Path, point};
    ///
    /// let path = Path::at(point!(1.m(), 2.m())).line_to(point!(3.m(), 4.m()));
    /// assert_eq!(path.end(), point!(3.m(), 4.m()));
    ///
    /// let empty_path = Path::at(point!(5.m(), 6.m()));
    /// assert_eq!(empty_path.end(), point!(5.m(), 6.m()));
    /// ```
    pub fn end(&self) -> Point<2> {
        match self.edges.iter().last() {
            Some(edge) => edge.end(),
            None => self.cursor,
        }
    }

    /// Return the direction the last element of this `Path` is pointing to.
    ///
    /// If the path is empty, a `Dir` parallel to the positive x-direction is returned.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Path, dir, point};
    ///
    /// assert_eq!(
    ///     Path::at(point!(0, 0)).line_to(point!(0.m(), 2.m())).end_direction(),
    ///     dir!(0, 1)
    /// );
    /// assert_eq!(
    ///     Path::at(point!(0, 0)).end_direction(),
    ///     dir!(1, 0)
    /// )
    /// ```
    pub fn end_direction(&self) -> Dir<2> {
        match self.edges.last() {
            Some(last_edge) => last_edge
                .end_direction()
                .expect("edge has already been checked for zero length"),
            None => Dir::from(Angle::zero()),
        }
    }

    /// Return the edges in this `Path`.
    pub fn edges(&self) -> Vec<Edge> {
        self.edges.clone()
    }

    /// Return the current cursor position of this `Path`.
    pub fn cursor(&self) -> Point<2> {
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
    use super::*;
    use crate::{IntoAngle, IntoLength, dir, point};
    use approx::assert_relative_eq;

    #[test]
    fn end_arc_positive_radius_angle() {
        let path = Path::at(point!(0, 0)).arc_by(1.m(), 90.deg());
        assert_relative_eq!(path.end(), point!(1.m(), 1.m()))
    }

    #[test]
    fn end_arc_positive_radius_negative_angle() {
        let path = Path::at(point!(0, 0)).arc_by(1.m(), -90.deg());
        assert_relative_eq!(path.end(), point!(-1.m(), 1.m()))
    }

    #[test]
    fn end_arc_negative_radius_positive_angle() {
        let path = Path::at(point!(0, 0)).arc_by(-1.m(), 90.deg());
        assert_relative_eq!(path.end(), point!(1.m(), -1.m()))
    }

    #[test]
    fn end_arc_negative_radius_angle() {
        let path = Path::at(point!(0, 0)).arc_by(-1.m(), -90.deg());
        assert_relative_eq!(path.end(), point!(-1.m(), -1.m()))
    }

    #[test]
    fn end_arc_negative_radius_positive_angle_45deg() {
        let path = Path::at(point!(0.m(), 1.m())).arc_by(-1.m(), 45.deg());
        assert_relative_eq!(
            path.end(),
            point!(1.m() / f64::sqrt(2.), 1.m() / f64::sqrt(2.)),
        )
    }

    #[test]
    fn end_direction_empty_path() {
        let path = Path::at(point!(0, 0));
        assert_relative_eq!(path.end_direction(), dir!(1, 0))
    }

    #[test]
    fn end_direction_line() {
        let path = Path::at(point!(0, 0)).line_to(point!(1.m(), 1.m()));
        assert_relative_eq!(path.end_direction(), dir!(1, 1))
    }

    #[test]
    fn end_direction_arc_positive_radius_angle() {
        let path = Path::at(point!(0, 0)).arc_by(1.m(), 45.deg());
        assert_relative_eq!(path.end_direction(), dir!(1, 1), epsilon = 1e-9)
    }

    #[test]
    fn end_direction_arc_positive_radius_negative_angle() {
        let path = Path::at(point!(0, 0)).arc_by(1.m(), -45.deg());
        assert_relative_eq!(path.end_direction(), dir!(-1, 1), epsilon = 1e-9)
    }

    #[test]
    fn end_direction_arc_negative_radius_positive_angle() {
        let path = Path::at(point!(0, 0)).arc_by(-1.m(), 45.deg());
        assert_relative_eq!(path.end_direction(), dir!(1, -1))
    }

    #[test]
    fn end_direction_arc_negative_radius_angle() {
        let path = Path::at(point!(0, 0)).arc_by(-1.m(), -45.deg());
        assert_relative_eq!(path.end_direction(), dir!(-1, -1))
    }
}
