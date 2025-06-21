use cxx::UniquePtr;
use opencascade_sys::ffi;

/// A 3D object in space.
pub struct Part {
    pub(crate) inner: Option<UniquePtr<ffi::TopoDS_Shape>>,
}
impl Part {
    pub(crate) fn from_occt(part: &ffi::TopoDS_Shape) -> Self {
        let inner = ffi::TopoDS_Shape_to_owned(part);
        Self { inner: Some(inner) }
    }
}
