use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::Part;

use super::face::Face;

/// Iterator over the `Face`s of a `Part`.
///
/// ```rust
/// use anvil::{Cube, Face, FaceIterator, IntoLength};
///
/// let face_iterator: FaceIterator = Cube::from_size(1.m()).faces();
/// assert_eq!(face_iterator.clone().len(), 6);
/// for face in face_iterator {
///     // ...
/// }
/// ```
pub enum FaceIterator {
    /// A FaceIterator that is not empty.
    NotEmpty(Part, UniquePtr<ffi::TopExp_Explorer>),
    /// A FaceIterator from an empty shape.
    Empty,
}

impl Iterator for FaceIterator {
    type Item = Face;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::NotEmpty(_, explorer) => {
                if explorer.More() {
                    let face = ffi::TopoDS_cast_to_face(explorer.Current());
                    let face = Face::from_occt(face);
                    explorer.pin_mut().Next();
                    Some(face)
                } else {
                    None
                }
            }
            Self::Empty => None,
        }
    }
}
impl FaceIterator {
    /// Return `true` if this `FaceIterator` has a length of 0.
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
    /// Return the number of `Face`s in this `FaceIterator`.
    pub fn len(self) -> usize {
        match self {
            Self::NotEmpty(_, _) => {
                let self_clone = self.clone();
                let mut len = 0;
                for _ in self_clone {
                    len += 1;
                }
                len
            }
            Self::Empty => 0,
        }
    }
}
impl Clone for FaceIterator {
    /// Return a clone of this `FaceIterator`.
    ///
    /// WARNING: Iterator position will not be cloned.
    fn clone(&self) -> Self {
        match self {
            Self::NotEmpty(part, _) => part.faces(),
            Self::Empty => Self::Empty,
        }
    }
}
impl From<&Part> for FaceIterator {
    fn from(value: &Part) -> Self {
        match &value.inner {
            Some(inner) => {
                let explorer = ffi::TopExp_Explorer_ctor(inner, ffi::TopAbs_ShapeEnum::TopAbs_FACE);
                Self::NotEmpty(value.clone(), explorer)
            }
            None => Self::Empty,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert!(Part::empty().faces().is_empty())
    }
}
