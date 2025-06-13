use opencascade_sys::ffi;

use crate::{Dir, Error, Face, IntoLength, Length, Point};

pub struct TexturedMesh {
    points: Vec<Point<3>>,
    indices: Vec<[usize; 3]>,
    normals: Vec<Dir<3>>,
    uvs: Vec<[f64; 2]>,
}
impl TryFrom<Face> for TexturedMesh {
    type Error = Error;
    fn try_from(value: Face) -> Result<Self, Self::Error> {
        Self::try_from((value, 0.1.mm()))
    }
}
impl TryFrom<(Face, Length)> for TexturedMesh {
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
            }
            Ok(TexturedMesh {
                points: points,
                indices: vec![],
                normals: vec![],
                uvs: vec![],
            })
        } else {
            Err(Error::Triangulation)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{IntoLength, Path, Plane, point};

    use super::*;

    #[test]
    fn triangle() {
        let face = Path::at(point!(0, 0))
            .line_to(point!(1.m(), 0.m()))
            .line_to(point!(0.m(), 1.m()))
            .close()
            .to_face(Plane::xy())
            .unwrap();
        let actual = TexturedMesh::try_from(face).unwrap();
        assert_eq!(
            actual.points,
            vec![
                point!(0, 0, 0),
                point!(1.m(), 0.m(), 0.m()),
                point!(0.m(), 1.m(), 0.m())
            ]
        );
    }
}
