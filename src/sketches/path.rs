use crate::Point2D;

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
