use core::f64;

use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::{Angle, Axis2D, Dir2D, Error, Length, Plane, Point2D};

/// A one-dimensional object in two-dimensional space.
#[derive(Debug, PartialEq, Clone)]
pub enum Edge {
    /// A circle section defined by the start point, a mid point and the end point.
    Arc(Point2D, Point2D, Point2D),

    /// A line between two points.
    Line(Point2D, Point2D),
}
impl Edge {
    /// Return the starting point of the edge.
    ///
    /// ```rust
    /// use anvil::{Edge, Point2D};
    ///
    /// let edge = Edge::Line(Point2D::from_m(1., 1.), Point2D::from_m(2., 2.));
    /// assert_eq!(edge.start(), Point2D::from_m(1., 1.))
    /// ```
    pub fn start(&self) -> Point2D {
        match self {
            Self::Arc(start, _, _) => *start,
            Self::Line(start, _) => *start,
        }
    }
    /// Return the ending point of the edge.
    ///
    /// ```rust
    /// use anvil::{Edge, Point2D};
    ///
    /// let edge = Edge::Line(Point2D::from_m(1., 1.), Point2D::from_m(2., 2.));
    /// assert_eq!(edge.end(), Point2D::from_m(2., 2.))
    /// ```
    pub fn end(&self) -> Point2D {
        match self {
            Self::Arc(_, _, end) => *end,
            Self::Line(_, end) => *end,
        }
    }

    /// Return the distance spanned by the `Edge`.
    ///
    /// ```rust
    /// use core::f64;
    /// use anvil::{Edge, IntoLength, point};
    ///
    /// let line = Edge::Line(point!(1.m(), 0.m()), point!(1.m(), 2.m()));
    /// assert_eq!(line.len(), 2.m());
    ///
    /// let arc = Edge::Arc(point!(-1.m(), 0.m()), point!(0.m(), 1.m()), point!(1.m(), 0.m()));
    /// assert_eq!(arc.len(), f64::consts::PI.m());
    /// ```
    pub fn len(&self) -> Length {
        match self {
            Self::Arc(start, mid, end) => {
                // Works for now but needs to be refactored in the future
                let (x1, y1) = (start.x.m(), start.y.m());
                let (x2, y2) = (mid.x.m(), mid.y.m());
                let (x3, y3) = (end.x.m(), end.y.m());

                let b = (x1.powi(2) + y1.powi(2)) * (y3 - y2)
                    + (x2.powi(2) + y2.powi(2)) * (y1 - y3)
                    + (x3.powi(2) + y3.powi(2)) * (y2 - y1);
                let c = (x1.powi(2) + y1.powi(2)) * (x2 - x3)
                    + (x2.powi(2) + y2.powi(2)) * (x3 - x1)
                    + (x3.powi(2) + y3.powi(2)) * (x1 - x2);

                let denom = 2.0 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));
                if denom.abs() < f64::EPSILON {
                    return Length::zero();
                }
                let cx = -b / denom;
                let cy = -c / denom;

                let r = f64::sqrt((x1 - cx).powi(2) + (y1 - cy).powi(2));

                let v1 = ((x1 - cx), (y1 - cy));
                let v2 = ((x3 - cx), (y3 - cy));

                let dot = v1.0 * v2.0 + v1.1 * v2.1;
                let det = v1.0 * v2.1 - v1.1 * v2.0;
                let mut angle = det.atan2(dot).abs();

                let is_mid_on_arc = {
                    let cross1 = (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1);
                    let cross2 = (cx - x1) * (y3 - y1) - (cy - y1) * (x3 - x1);
                    cross1 * cross2 >= 0.0
                };
                if !is_mid_on_arc {
                    angle = f64::consts::TAU - angle;
                }

                Length::from_m(r * angle)
            }
            Self::Line(start, end) => {
                let diff = *start - *end;
                Length::from_m(f64::sqrt(diff.x.m().powi(2) + diff.y.m().powi(2)))
            }
        }
    }

    /// Return the direction this `Edge` is pointing to at its end point.
    ///
    /// ```rust
    /// use anvil::{Dir2D, Edge, IntoLength, point};
    ///
    /// let line = Edge::Line(point!(0, 0), point!(1.m(), 2.m()));
    /// assert_eq!(line.end_direction(), Dir2D::try_from(1., 2.));
    /// ```
    pub fn end_direction(&self) -> Result<Dir2D, Error> {
        match self {
            Self::Arc(start, interior, end) => {
                let (center, _) = arc_center_radius(*start, *interior, *end)?;

                let start_angle = arc_point_angle_on_unit_circle(center, *start);
                let interior_angle = arc_point_angle_on_unit_circle(center, *interior);
                let end_angle = arc_point_angle_on_unit_circle(center, *end);

                let arc_is_clockwise = (end_angle > start_angle || start_angle > interior_angle)
                    && interior_angle > end_angle;

                if arc_is_clockwise {
                    Ok(Dir2D::from(end_angle - Angle::from_deg(90.)))
                } else {
                    Ok(Dir2D::from(end_angle + Angle::from_deg(90.)))
                }
            }
            Self::Line(start, end) => Dir2D::try_from((*end - *start).x.m(), (*end - *start).y.m()),
        }
    }

    pub(crate) fn to_occt(&self, plane: Plane) -> Option<UniquePtr<ffi::TopoDS_Edge>> {
        if self.len() == Length::zero() {
            return None;
        }
        match self {
            Self::Arc(start, mid, end) => {
                let make_arc = ffi::GC_MakeArcOfCircle_point_point_point(
                    &start.to_3d(plane).to_occt_point(),
                    &mid.to_3d(plane).to_occt_point(),
                    &end.to_3d(plane).to_occt_point(),
                );
                Some(ffi::TopoDS_Edge_to_owned(
                    ffi::BRepBuilderAPI_MakeEdge_HandleGeomCurve(
                        &ffi::new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(
                            &ffi::GC_MakeArcOfCircle_Value(&make_arc),
                        ),
                    )
                    .pin_mut()
                    .Edge(),
                ))
            }
            Self::Line(start, end) => {
                let mut constructor = ffi::BRepBuilderAPI_MakeEdge_gp_Pnt_gp_Pnt(
                    &start.to_3d(plane).to_occt_point(),
                    &end.to_3d(plane).to_occt_point(),
                );
                Some(ffi::TopoDS_Edge_to_owned(constructor.pin_mut().Edge()))
            }
        }
    }
}

fn arc_center_radius(
    start: Point2D,
    interior: Point2D,
    end: Point2D,
) -> Result<(Point2D, Length), Error> {
    if start == interior || interior == end || end == start {
        return Err(Error::ZeroVector);
    }

    let start_interior_midpoint =
        Point2D::new((start.x + interior.x) / 2., (start.y + interior.y) / 2.);
    let interior_end_midpoint = Point2D::new((end.x + interior.x) / 2., (end.y + interior.y) / 2.);

    let start_interior_direction = interior
        .direction_from(start)
        .expect("zero vector already checked above");
    let interior_end_direction = end
        .direction_from(interior)
        .expect("zero vector already checked above");

    let start_interior_axis = Axis2D::new(
        start_interior_midpoint,
        start_interior_direction.rotate(Angle::from_deg(90.)),
    );
    let interior_end_axis = Axis2D::new(
        interior_end_midpoint,
        interior_end_direction.rotate(Angle::from_deg(90.)),
    );

    let center = start_interior_axis
        .intersect(interior_end_axis)
        .expect("zero vector already checked above");

    let radius = (center - start).distance_to_origin();

    Ok((center, radius))
}

fn arc_point_angle_on_unit_circle(center: Point2D, point: Point2D) -> Angle {
    point
        .direction_from(center)
        .expect("center and point can not be the same")
        .angle()
}
