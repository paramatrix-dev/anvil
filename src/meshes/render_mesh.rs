use opencascade_sys::ffi;

use crate::{Dir, Error, Face, IntoLength, Length, Part, Point};

const DEFAULT_TOLERANCE: Length = Length::from_m(0.000001);

/// A triangular mesh of one or more `Face`s optimized for 3D rendering.
#[derive(Clone, Debug, PartialEq)]
pub struct RenderMesh {
    points: Vec<Point<3>>,
    indices: Vec<[usize; 3]>,
    normals: Vec<Dir<3>>,
    uvs: Vec<[f64; 2]>,
}
impl RenderMesh {
    /// Return a clone of this `RenderMesh` with the individual indices sorted.
    ///
    /// Sorting of the triangle indices depends on the machine executing the tests which introduces
    /// non-deterministic behavior. This function enables comparing `RenderMesh`es across devices.
    pub fn sorted(&self) -> Self {
        Self {
            points: self.points.clone(),
            indices: {
                let mut sorted_indices = vec![];
                for triangle in self.indices.clone() {
                    let mut sorted_triangle = triangle;
                    sorted_triangle.sort();
                    sorted_indices.push(sorted_triangle);
                }
                sorted_indices
            },
            normals: self.normals.clone(),
            uvs: self.uvs.clone(),
        }
    }

    /// Return the `Point`s of this `RenderMesh`.
    pub fn points(&self) -> &Vec<Point<3>> {
        &self.points
    }
    /// Return the `Point` indices defining the triangles of this `RenderMesh`.
    pub fn indices(&self) -> &Vec<[usize; 3]> {
        &self.indices
    }
    /// Return the normal `Dir` of every `Point` in this `RenderMesh`.
    pub fn normals(&self) -> &Vec<Dir<3>> {
        &self.normals
    }
    /// Return the relative position of every `Point` on the 2D-grid of this `RenderMesh`.
    pub fn uvs(&self) -> &Vec<[f64; 2]> {
        &self.uvs
    }

    /// Return the collective area spanned by the triangles in a `RenderedMesh` in square meters.
    ///
    /// ```rust
    /// use anvil::{Cube, IntoLength, Plane, Rectangle, RenderMesh};
    /// use approx::assert_relative_eq;
    ///
    /// let rect = Rectangle::from_dim(2.m(), 3.m());
    /// let mesh = RenderMesh::try_from(rect.to_face(Plane::xy()).unwrap()).unwrap();
    /// assert_relative_eq!(mesh.area(), 6.);
    ///
    /// let cube = Cube::from_size(2.m());
    /// let mesh = RenderMesh::try_from(cube).unwrap();
    /// assert_relative_eq!(mesh.area(), 24.);
    /// ```
    pub fn area(&self) -> f64 {
        let mut total_area = 0.;
        for triangle in &self.indices {
            let point1 = *self
                .points
                .get(triangle[0])
                .expect("index should be a valid point");
            let point2 = *self
                .points
                .get(triangle[1])
                .expect("index should be a valid point");
            let point3 = *self
                .points
                .get(triangle[2])
                .expect("index should be a valid point");
            let edge_1 = point2 - point1;
            let edge_2 = point3 - point1;

            let cross = (
                edge_1.y().m() * edge_2.z().m() - edge_1.z().m() * edge_2.y().m(),
                edge_1.z().m() * edge_2.x().m() - edge_1.x().m() * edge_2.z().m(),
                edge_1.x().m() * edge_2.y().m() - edge_1.y().m() * edge_2.x().m(),
            );

            total_area += 0.5 * f64::sqrt(cross.0.powi(2) + cross.1.powi(2) + cross.2.powi(2));
        }
        total_area
    }
    /// Return the center point of the `RenderMesh`, i.e. the average of all mesh points.
    ///
    /// ```rust
    /// use anvil::{IntoLength, Plane, Rectangle, RenderMesh, point};
    /// use approx::assert_relative_eq;
    ///
    /// let rect = Rectangle::from_dim(1.m(), 1.m()).move_to(point!(2.m(), 3.m()));
    /// let mesh = RenderMesh::try_from(rect.to_face(Plane::xy()).unwrap()).unwrap();
    /// let mesh_center = mesh.center();
    /// assert_relative_eq!(
    ///     mesh_center,
    ///     point!(2.m(), 3.m(), 0.m())
    /// );
    /// ```
    pub fn center(&self) -> Point<3> {
        let mut sum_of_points = Point::<3>::origin();
        for point in &self.points {
            sum_of_points = sum_of_points + *point;
        }
        sum_of_points / self.points.len() as f64
    }

    fn empty() -> Self {
        Self {
            points: vec![],
            indices: vec![],
            normals: vec![],
            uvs: vec![],
        }
    }

    fn merge_with(&mut self, other: Self) {
        self.indices.extend(other.indices().iter().map(|t| {
            [
                t[0] + self.points.len(),
                t[1] + self.points.len(),
                t[2] + self.points.len(),
            ]
        }));
        self.points.extend(other.points());
        self.normals.extend(other.normals());
        self.uvs.extend(other.uvs());
    }
}

impl TryFrom<Face> for RenderMesh {
    type Error = Error;
    fn try_from(face: Face) -> Result<Self, Self::Error> {
        (face, DEFAULT_TOLERANCE).try_into()
    }
}
impl TryFrom<(Face, Length)> for RenderMesh {
    type Error = Error;
    fn try_from((face, tolerance): (Face, Length)) -> Result<Self, Self::Error> {
        let mesh = ffi::BRepMesh_IncrementalMesh_ctor(
            ffi::cast_face_to_shape(face.0.as_ref().unwrap()),
            tolerance.m(),
        );
        let face = ffi::TopoDS_cast_to_face(mesh.as_ref().unwrap().Shape());
        let mut location = ffi::TopLoc_Location_ctor();

        let triangulation_handle = ffi::BRep_Tool_Triangulation(face, location.pin_mut());
        let transformation = ffi::TopLoc_Location_Transformation(&location);

        if let Ok(triangulation) = ffi::HandlePoly_Triangulation_Get(&triangulation_handle) {
            let mut points = vec![];
            let mut indices = vec![];
            let mut normals = vec![];
            let mut uvs = vec![];

            let orientation = face.Orientation();
            let face_point_count = triangulation.NbNodes();
            ffi::compute_normals(face, &triangulation_handle);

            for node_index in 1..=face_point_count {
                let mut point = ffi::Poly_Triangulation_Node(triangulation, node_index);
                point.pin_mut().Transform(&transformation);
                points.push(Point::<3>::new([
                    point.X().m(),
                    point.Y().m(),
                    point.Z().m(),
                ]));

                let uv = ffi::Poly_Triangulation_UV(triangulation, node_index);
                uvs.push([uv.X(), uv.Y()]);

                let mut normal = ffi::Poly_Triangulation_Normal(triangulation, node_index);
                normal.pin_mut().Transform(&transformation);
                let m = if orientation == ffi::TopAbs_Orientation::TopAbs_REVERSED {
                    -1.
                } else {
                    1.
                };
                normals.push(
                    Dir::try_from([normal.X() * m, normal.Y() * m, normal.Z() * m])
                        .expect("normals should not be zero"),
                );
            }

            let mut u_min = f64::INFINITY;
            let mut v_min = f64::INFINITY;
            let mut u_max = f64::NEG_INFINITY;
            let mut v_max = f64::NEG_INFINITY;

            for &[u, v] in &uvs {
                u_min = u_min.min(u);
                v_min = v_min.min(v);
                u_max = u_max.max(u);
                v_max = v_max.max(v);
            }

            for [u, v] in &mut uvs {
                *u = (*u - u_min) / (u_max - u_min);
                *v = (*v - v_min) / (v_max - v_min);

                if orientation == ffi::TopAbs_Orientation::TopAbs_REVERSED {
                    *u = 1.0 - *u;
                }
            }

            for triangle_index in 1..=triangulation.NbTriangles() {
                let triangle = triangulation.Triangle(triangle_index);
                let mut node_ids = [triangle.Value(1), triangle.Value(2), triangle.Value(3)]
                    .map(|id| id as usize - 1);

                if orientation == ffi::TopAbs_Orientation::TopAbs_REVERSED {
                    node_ids.swap(1, 2);
                }
                indices.push(node_ids);
            }

            Ok(RenderMesh {
                points,
                indices,
                normals,
                uvs,
            })
        } else {
            Err(Error::Triangulation)
        }
    }
}
impl TryFrom<Part> for RenderMesh {
    type Error = Error;
    fn try_from(part: Part) -> Result<Self, Self::Error> {
        (part, DEFAULT_TOLERANCE).try_into()
    }
}
impl TryFrom<(Part, Length)> for RenderMesh {
    type Error = Error;
    fn try_from((part, tolerance): (Part, Length)) -> Result<Self, Self::Error> {
        let meshes = part
            .faces()
            .map(|face| RenderMesh::try_from((face, tolerance)))
            .collect::<Result<Vec<RenderMesh>, Error>>()?;
        Ok(merge(meshes))
    }
}

fn merge(meshes: Vec<RenderMesh>) -> RenderMesh {
    let mut merged_mesh = RenderMesh::empty();
    for mesh in meshes {
        merged_mesh.merge_with(mesh);
    }
    merged_mesh
}

#[cfg(test)]
mod tests {
    use core::f64;

    use approx::{assert_abs_diff_eq, assert_relative_eq};

    use crate::{Axis, Circle, Cube, IntoAngle, IntoLength, Path, Plane, Rectangle, dir, point};

    use super::*;

    #[test]
    fn triangle() {
        let face = Path::at(point!(0, 0))
            .line_to(point!(1.m(), 0.m()))
            .line_to(point!(0.m(), 1.m()))
            .close()
            .to_face(Plane::xy())
            .unwrap();

        assert_eq!(
            RenderMesh::try_from(face).unwrap().sorted(),
            RenderMesh {
                points: vec![
                    point!(0, 0, 0),
                    point!(1.m(), 0.m(), 0.m()),
                    point!(0.m(), 1.m(), 0.m())
                ],
                indices: vec![[0, 1, 2]],
                normals: vec![dir!(0, 0, 1), dir!(0, 0, 1), dir!(0, 0, 1)],
                uvs: vec![[0., 0.], [1., 0.], [0., 1.]]
            }
        )
    }

    #[test]
    fn rectangle() {
        let face = Rectangle::from_corners(point!(0, 0), point!(1.m(), 1.m()))
            .to_face(Plane::xy())
            .unwrap();

        assert_eq!(
            RenderMesh::try_from(face).unwrap().sorted(),
            RenderMesh {
                points: vec![
                    point!(0, 0, 0),
                    point!(1.m(), 0.m(), 0.m()),
                    point!(1.m(), 1.m(), 0.m()),
                    point!(0.m(), 1.m(), 0.m()),
                ],
                indices: vec![[0, 1, 2], [0, 2, 3]],
                normals: vec![dir!(0, 0, 1), dir!(0, 0, 1), dir!(0, 0, 1), dir!(0, 0, 1)],
                uvs: vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]]
            }
        )
    }

    #[test]
    fn circle() {
        let mesh =
            RenderMesh::try_from(Circle::from_radius(1.m()).to_face(Plane::xy()).unwrap()).unwrap();
        assert_abs_diff_eq!(mesh.center(), point!(0, 0, 0), epsilon = 1e-6);
        assert_abs_diff_eq!(mesh.area(), f64::consts::PI, epsilon = 1e-4);
        assert_eq!(mesh.normals(), &vec![dir!(0, 0, -1); mesh.normals().len()]);
    }

    #[test]
    fn rotated_cube_has_correct_normals() {
        let cube = Cube::from_size(1.m())
            .rotate_around(Axis::<3>::x(), 45.deg())
            .rotate_around(Axis::<3>::z(), 45.deg());
        let mesh =
            RenderMesh::try_from(cube.faces().collect::<Vec<Face>>().first().unwrap().clone())
                .unwrap();
        for normal in mesh.normals {
            assert_relative_eq!(normal, dir!(-1, -1, 0))
        }
    }

    #[test]
    fn cube() {
        let cube_mesh = RenderMesh::try_from(Cube::from_size(2.m()))
            .unwrap()
            .sorted();
        assert_eq!(
            cube_mesh.points(),
            &vec![
                // -x face
                point!(-1.m(), -1.m(), -1.m()),
                point!(-1.m(), -1.m(), 1.m()),
                point!(-1.m(), 1.m(), -1.m()),
                point!(-1.m(), 1.m(), 1.m()),
                // +x face
                point!(1.m(), -1.m(), -1.m()),
                point!(1.m(), -1.m(), 1.m()),
                point!(1.m(), 1.m(), -1.m()),
                point!(1.m(), 1.m(), 1.m()),
                // -y face
                point!(-1.m(), -1.m(), -1.m()),
                point!(1.m(), -1.m(), -1.m()),
                point!(-1.m(), -1.m(), 1.m()),
                point!(1.m(), -1.m(), 1.m()),
                // +y face
                point!(-1.m(), 1.m(), -1.m()),
                point!(1.m(), 1.m(), -1.m()),
                point!(-1.m(), 1.m(), 1.m()),
                point!(1.m(), 1.m(), 1.m()),
                // -z face
                point!(-1.m(), -1.m(), -1.m()),
                point!(-1.m(), 1.m(), -1.m()),
                point!(1.m(), -1.m(), -1.m()),
                point!(1.m(), 1.m(), -1.m()),
                // +z face
                point!(-1.m(), -1.m(), 1.m()),
                point!(-1.m(), 1.m(), 1.m()),
                point!(1.m(), -1.m(), 1.m()),
                point!(1.m(), 1.m(), 1.m()),
            ]
        );
        assert_eq!(
            cube_mesh.indices(),
            &vec![
                // -x face
                [0, 1, 2],
                [1, 2, 3],
                // +x face
                [4, 5, 6],
                [5, 6, 7],
                // -y face
                [8, 9, 11],
                [8, 10, 11],
                // +y face
                [12, 13, 15],
                [12, 14, 15],
                // -z face
                [16, 17, 19],
                [16, 18, 19],
                // +z face
                [20, 21, 23],
                [20, 22, 23],
            ]
        );
        assert_eq!(
            cube_mesh.normals(),
            &vec![
                // -x face
                dir!(-1, 0, 0),
                dir!(-1, 0, 0),
                dir!(-1, 0, 0),
                dir!(-1, 0, 0),
                // +x face
                dir!(1, 0, 0),
                dir!(1, 0, 0),
                dir!(1, 0, 0),
                dir!(1, 0, 0),
                // -y face
                dir!(0, -1, 0),
                dir!(0, -1, 0),
                dir!(0, -1, 0),
                dir!(0, -1, 0),
                // +y face
                dir!(0, 1, 0),
                dir!(0, 1, 0),
                dir!(0, 1, 0),
                dir!(0, 1, 0),
                // -z face
                dir!(0, 0, -1),
                dir!(0, 0, -1),
                dir!(0, 0, -1),
                dir!(0, 0, -1),
                // +z face
                dir!(0, 0, 1),
                dir!(0, 0, 1),
                dir!(0, 0, 1),
                dir!(0, 0, 1),
            ]
        )
    }
}
