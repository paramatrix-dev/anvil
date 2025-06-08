use opencascade_sys::ffi;

use super::Part;

/// An indexed triangle mesh with vertices, UVs, and normals
///
/// Vertices are not guaranteed to be unique.
#[derive(Debug, Clone)]
pub struct IndexedMesh {
    /// Array of vertices, not necessarily each unique
    pub vertices: Vec<[f64; 3]>,
    /// Array of UV coordinates, normalized per-face to [0,1]²
    pub uvs: Vec<[f64; 2]>,
    /// Array of unit normals
    pub normals: Vec<[f64; 3]>,
    /// Array of triangles defined by indices
    pub indices: Vec<[usize; 3]>,
    /// Count of faces that couldn't be triangulated (hopefully zero)
    pub missing_faces: usize,
}

impl Part {
    /// Extract an indexed mesh from a Part with default tolerance
    ///
    /// Vertices are currently deduplicated per "face" of the Part, but each face will
    /// have its own copy of shared vertices.
    ///
    /// See details [here](https://dev.opencascade.org/doc/overview/html/occt_user_guides__mesh.html).
    pub fn triangulate(&self) -> IndexedMesh {
        self.triangulate_with_tolerance(0.0001)
    }

    /// Extract an indexed mesh from a Part with default tolerance
    ///
    /// Vertices are currently deduplicated per "face" of the Part, but each face will
    /// have its own copy of shared vertices.
    ///
    /// See details [here](https://dev.opencascade.org/doc/overview/html/occt_user_guides__mesh.html).
    pub fn triangulate_with_tolerance(&self, tolerance: f64) -> IndexedMesh {
        let Some(original_shape) = self.inner.as_ref().and_then(|ptr| ptr.as_ref()) else {
            return IndexedMesh {
                vertices: Vec::new(),
                uvs: Vec::new(),
                normals: Vec::new(),
                indices: Vec::new(),
                missing_faces: 0,
            };
        };

        let mesh = ffi::BRepMesh_IncrementalMesh_ctor(original_shape, tolerance);
        let mesh = mesh.as_ref().unwrap();

        let mut face_explorer =
            ffi::TopExp_Explorer_ctor(mesh.Shape(), ffi::TopAbs_ShapeEnum::TopAbs_FACE);

        let mut vertices = Vec::new();
        let mut uvs = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();
        let mut missing_faces = 0;

        while face_explorer.More() {
            let face_shape = face_explorer.Current();
            let mut location = ffi::TopLoc_Location_ctor();
            let face_topo = ffi::TopoDS_cast_to_face(face_shape);
            let orientation = face_shape.Orientation();

            let triangulation_handle = ffi::BRep_Tool_Triangulation(face_topo, location.pin_mut());
            let transformation = ffi::TopLoc_Location_Transformation(&location);

            if let Ok(triangulation) = ffi::HandlePoly_Triangulation_Get(&triangulation_handle) {
                // let transformation = ffi::TopLoc_Location_Transformation(&location);
                let vertex_offset = vertices.len();
                let face_point_count = triangulation.NbNodes();
                ffi::compute_normals(face_topo, &triangulation_handle);

                // Extract vertices
                for node_index in 1..=face_point_count {
                    let mut point = ffi::Poly_Triangulation_Node(triangulation, node_index);
                    point.pin_mut().Transform(&transformation);
                    vertices.push([point.X(), point.Y(), point.Z()]);

                    let uv = ffi::Poly_Triangulation_UV(triangulation, node_index);
                    uvs.push([uv.X(), uv.Y()]);

                    let mut normal = ffi::Poly_Triangulation_Normal(triangulation, node_index);
                    normal.pin_mut().Transform(&transformation);
                    let m = if orientation == ffi::TopAbs_Orientation::TopAbs_REVERSED {
                        -1.
                    } else {
                        1.
                    };
                    normals.push([normal.X() * m, normal.Y() * m, normal.Z() * m]);
                }

                // Normalize uvs (each face has a seperate [0, 1]^2 set of uv's, but it seems that for some
                // forms of geometry opencascade instead returns [0, width]x[0, height]).
                let mut u_min = f64::INFINITY;
                let mut v_min = f64::INFINITY;
                let mut u_max = f64::NEG_INFINITY;
                let mut v_max = f64::NEG_INFINITY;

                for &[u, v] in &uvs[vertex_offset..] {
                    u_min = u_min.min(u);
                    v_min = v_min.min(v);
                    u_max = u_max.max(u);
                    v_max = v_max.max(v);
                }

                for [u, v] in &mut uvs[vertex_offset..] {
                    *u = (*u - u_min) / (u_max - u_min);
                    *v = (*v - v_min) / (v_max - v_min);

                    if orientation == ffi::TopAbs_Orientation::TopAbs_REVERSED {
                        *u = 1.0 - *u;
                    }
                }

                // Extract triangle indices
                for triangle_index in 1..=triangulation.NbTriangles() {
                    let triangle = triangulation.Triangle(triangle_index);
                    let mut node_ids = [triangle.Value(1), triangle.Value(2), triangle.Value(3)]
                        .map(|id| id as usize + vertex_offset - 1);

                    if orientation == ffi::TopAbs_Orientation::TopAbs_REVERSED {
                        // Reverse triangle winding
                        node_ids.swap(1, 2);
                    }

                    indices.push(node_ids);
                }
            } else {
                missing_faces += 1;
            }

            face_explorer.pin_mut().Next();
        }

        IndexedMesh {
            vertices,
            uvs,
            normals,
            indices,
            missing_faces,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cuboid, IntoLength as _};

    use super::*;

    #[test]
    fn test_triangulate_empty() {
        let mesh = Part::empty().triangulate();
        assert!(mesh.vertices.is_empty());
        assert!(mesh.uvs.is_empty());
        assert!(mesh.normals.is_empty());
        assert!(mesh.indices.is_empty());
        assert_eq!(mesh.missing_faces, 0);
    }

    #[test]
    fn test_triangulate_box_indexed() {
        let mesh = Cuboid::from_dim(1.0.mm(), 2.0.mm(), 3.0.mm()).triangulate();

        // A cube should have 12 triangles
        assert_eq!(mesh.indices.len(), 12);
        assert_eq!(mesh.missing_faces, 0);

        for triangle_indices in &mesh.indices {
            // Check for off by one errors...
            for &index in triangle_indices {
                assert!(index < mesh.vertices.len());
            }
        }

        // Faces should have 4 vertices, for 24 total.
        assert_eq!(mesh.vertices.len(), 4 * 6);
        assert_eq!(mesh.uvs.len(), mesh.vertices.len());
        assert_eq!(mesh.normals.len(), mesh.vertices.len());
    }

    #[test]
    fn test_triangulate_uvs_and_normals() {
        let mesh = Cuboid::from_dim(1.0.mm(), 1.0.mm(), 2.0.mm())
            .scale(2.)
            .triangulate();

        // Verify we have the expected data structure integrity
        assert_eq!(mesh.uvs.len(), mesh.vertices.len());
        assert_eq!(mesh.normals.len(), mesh.vertices.len());
        assert_eq!(mesh.missing_faces, 0);

        // Check that UVs are normalized
        for uv in &mesh.uvs {
            assert!(
                uv[0] >= 0.0 && uv[0] <= 1.0,
                "UV u-coordinate {} not in [0,1]",
                uv[0]
            );
            assert!(
                uv[1] >= 0.0 && uv[1] <= 1.0,
                "UV v-coordinate {} not in [0,1]",
                uv[1]
            );
        }

        // Check that normals are (approximately) unit vectors
        for normal in &mesh.normals {
            let magnitude = (normal[0].powi(2) + normal[1].powi(2) + normal[2].powi(2)).sqrt();
            assert!(
                (magnitude - 1.0).abs() < 0.1,
                "Normal magnitude {magnitude} is not approximately 1.0"
            );
        }

        // For a unit cube, we expect normals in both positive and negative primary directions
        let mut found_directions = [false; 6]; // +X, -X, +Y, -Y, +Z, -Z

        for normal in &mesh.normals {
            println!("normal: {normal:?}");
            for (axis, &value) in normal.iter().enumerate() {
                let other_axes_small = normal
                    .iter()
                    .enumerate()
                    .all(|(i, &v)| i == axis || v.abs() < 0.1);

                if value > 0.9 && other_axes_small {
                    found_directions[axis * 2] = true; // positive direction
                } else if value < -0.9 && other_axes_small {
                    found_directions[axis * 2 + 1] = true; // negative direction
                }
            }
        }

        assert!(
            found_directions.iter().all(|&x| x),
            "Should have found normals pointing in all cardinal directions. Found normals for +X, -X, +Y, -Y, +Z, -Z respectively: {found_directions:?}"
        );
    }
}
