use cxx::UniquePtr;
use opencascade_sys::ffi;

pub struct Face(pub(crate) UniquePtr<ffi::TopoDS_Face>);
impl Face {
    pub(crate) fn from_occt(occt: &ffi::TopoDS_Face) -> Self {
        Self(ffi::TopoDS_Face_to_owned(occt))
    }
}

impl Clone for Face {
    fn clone(&self) -> Self {
        Self::from_occt(&self.0)
    }
}
