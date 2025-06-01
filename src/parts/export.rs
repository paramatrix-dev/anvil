use opencascade_sys::ffi;

use super::Part;

/// An indexed triangle mesh
#[derive(Debug, Clone)]
pub struct IndexedMesh {
    /// Array of vertices, not necessarily each unique
    pub vertices: Vec<[f64; 3]>,
    /// Array of triangle defined by index
    pub indices: Vec<[usize; 3]>,
    /// Count of faces that couldn't be triangulated (hopefully zero)
    pub missing_faces: usize,
}

impl Part {
    /// Extract an indexed mesh from a Part
    ///
    /// Note that this does not fully deduplicate vertices. Each shape is
    /// individually translated to vertices, and then those vertices are
    /// indexed by triangles on that face.
    pub fn triangulate(&self) -> IndexedMesh {
        self.triangulate_with_tolerance(0.0001)
    }

    /// Extract an indexed mesh from a Part
    ///
    /// Note that this does not fully deduplicate vertices. Each shape is
    /// individually translated to vertices, and then those vertices are
    /// indexed by triangles on that face.
    ///
    /// Smaller tolerances lead to higher precision in rounded shapes, but also
    /// larger file size.
    pub fn triangulate_with_tolerance(&self, tolerance: f64) -> IndexedMesh {
        let Some(original_shape) = self.inner.as_ref().and_then(|ptr| ptr.as_ref()) else {
            return IndexedMesh {
                vertices: Vec::new(),
                indices: Vec::new(),
                missing_faces: 0,
            };
        };

        let mesh = ffi::BRepMesh_IncrementalMesh_ctor(original_shape, tolerance);
        let mesh = mesh.as_ref().unwrap();

        let mut face_explorer =
            ffi::TopExp_Explorer_ctor(mesh.Shape(), ffi::TopAbs_ShapeEnum::TopAbs_FACE);

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut missing_faces = 0;

        while face_explorer.More() {
            let face_shape = face_explorer.Current();
            let mut location = ffi::TopLoc_Location_ctor();
            let face_topo = ffi::TopoDS_cast_to_face(face_shape);
            let orientation = face_shape.Orientation();

            let triangulation = ffi::BRep_Tool_Triangulation(face_topo, location.pin_mut());

            if let Ok(triangulation) = ffi::HandlePoly_Triangulation_Get(&triangulation) {
                let transformation = ffi::TopLoc_Location_Transformation(&location);
                let vertex_offset = vertices.len();

                for node_index in 1..=triangulation.NbNodes() {
                    let mut point = ffi::Poly_Triangulation_Node(triangulation, node_index);
                    point.pin_mut().Transform(&transformation);
                    vertices.push([point.X(), point.Y(), point.Z()]);
                }

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
            // All indices should be valid
            for &index in triangle_indices {
                assert!(index < mesh.vertices.len());
            }
        }

        // We should have 4 vertices for each face.
        assert_eq!(mesh.vertices.len(), 4 * 6);
    }
}
