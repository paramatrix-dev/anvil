use opencascade_sys::ffi;

use crate::Part;

impl Part {
    /// Return a copy of this `Part` with the intersection of another removed.
    ///
    /// ```rust
    /// use anvil::{Cuboid, IntoLength, point};
    ///
    /// let cuboid1 = Cuboid::from_corners(point!(0, 0, 0), point!(1.m(), 1.m(), 2.m()));
    /// let cuboid2 = Cuboid::from_corners(point!(0.m(), 0.m(), 1.m()), point!(1.m(), 1.m(), 2.m()));
    /// assert_eq!(
    ///     cuboid1.subtract(&cuboid2),
    ///     Cuboid::from_corners(point!(0, 0, 0), point!(1.m(), 1.m(), 1.m()))
    /// );
    /// ```
    pub fn subtract(&self, other: &Self) -> Self {
        match (&self.inner, &other.inner) {
            (Some(self_inner), Some(other_inner)) => {
                let mut fuse_operation = ffi::BRepAlgoAPI_Cut_ctor(self_inner, other_inner);
                Self::from_occt(fuse_operation.pin_mut().Shape())
            }
            (Some(_), None) => self.clone(),
            (None, _) => Part { inner: None },
        }
    }
}
