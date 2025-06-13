use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::Part;

use super::face::Face;

pub struct FaceIterator(Part, UniquePtr<ffi::TopExp_Explorer>);
impl Iterator for FaceIterator {
    type Item = Face;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1.More() {
            let face = ffi::TopoDS_cast_to_face(self.1.Current());
            let face = Face::from_occt(face);
            self.1.pin_mut().Next();
            Some(face)
        } else {
            None
        }
    }
}
impl FaceIterator {
    /// Return the number of `Face`s in this `FaceIterator`.
    pub fn len(self) -> usize {
        let mut self_clone = self.clone();
        let mut len = 0;
        while let Some(_) = self_clone.next() {
            len += 1;
        }
        len
    }
}
impl Clone for FaceIterator {
    /// Return a clone of this `FaceIterator`.
    ///
    /// WARNING: Iterator position will not be cloned.
    fn clone(&self) -> Self {
        self.0.faces()
    }
}
impl From<&Part> for FaceIterator {
    fn from(value: &Part) -> Self {
        match &value.inner {
            Some(inner) => {
                let explorer =
                    ffi::TopExp_Explorer_ctor(&inner, ffi::TopAbs_ShapeEnum::TopAbs_FACE);
                Self(value.clone(), explorer)
            }
            None => todo!(),
        }
    }
}
