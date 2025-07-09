use opencascade_sys::ffi;

use crate::Part;

impl Part {
    /// Return the `Part` that is created from the overlapping volume between this one and another.
    ///
    /// ```rust
    /// use anvil::{Cuboid, IntoLength};
    ///
    /// let cuboid1 = Cuboid::from_dim(5.m(), 5.m(), 1.m());
    /// let cuboid2 = Cuboid::from_dim(1.m(), 1.m(), 5.m());
    /// assert_eq!(
    ///     cuboid1.intersect(&cuboid2),
    ///     Cuboid::from_dim(1.m(), 1.m(), 1.m())
    /// )
    /// ```
    pub fn intersect(&self, other: &Self) -> Self {
        match (&self.inner, &other.inner) {
            (Some(self_inner), Some(other_inner)) => {
                let mut fuse_operation = ffi::BRepAlgoAPI_Common_ctor(self_inner, other_inner);
                Self::from_occt(fuse_operation.pin_mut().Shape())
            }
            _ => Part { inner: None },
        }
    }
}
