use opencascade_sys::ffi;

use crate::{Dir, Error, Face, IntoLength, Length, Point};

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
}

impl TryFrom<Face> for RenderMesh {
    type Error = Error;
    fn try_from(value: Face) -> Result<Self, Self::Error> {
        Self::try_from((value, 0.1.mm()))
    }
}
impl TryFrom<(Face, Length)> for RenderMesh {
    type Error = Error;
    fn try_from(value: (Face, Length)) -> Result<Self, Self::Error> {
        let mesh = ffi::BRepMesh_IncrementalMesh_ctor(
            ffi::cast_face_to_shape(value.0.0.as_ref().unwrap()),
            value.1.m(),
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

#[cfg(test)]
mod tests {
    use crate::{Axis, Cube, IntoAngle, IntoLength, Path, Plane, Rectangle, dir, point};

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
    fn rotated_cube_has_correct_normals() {
        let cube = Cube::from_size(1.m())
            .rotate_around(Axis::<3>::x(), 45.deg())
            .rotate_around(Axis::<3>::z(), 45.deg());
        let mesh =
            RenderMesh::try_from(cube.faces().collect::<Vec<Face>>().first().unwrap().clone())
                .unwrap();
        for normal in mesh.normals {
            assert!(normal.approx_eq(dir!(-1, -1, 0)))
        }
    }
}
