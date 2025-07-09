use opencascade_sys::ffi;

use crate::Part;

impl Part {
    /// Merge this `Part` with another.
    ///
    /// ```rust
    /// use anvil::{Cuboid, point, IntoLength};
    ///
    /// let cuboid1 = Cuboid::from_corners(point!(0, 0, 0), point!(1.m(), 1.m(), 1.m()));
    /// let cuboid2 = Cuboid::from_corners(point!(0.m(), 0.m(), 1.m()), point!(1.m(), 1.m(), 2.m()));
    ///
    /// assert_eq!(
    ///     cuboid1.add(&cuboid2),
    ///     Cuboid::from_corners(point!(0, 0, 0), point!(1.m(), 1.m(), 2.m()))
    /// )
    /// ```
    pub fn add(&self, other: &Self) -> Self {
        match (&self.inner, &other.inner) {
            (Some(self_inner), Some(other_inner)) => {
                let mut fuse_operation = ffi::BRepAlgoAPI_Fuse_ctor(self_inner, other_inner);
                Self::from_occt(fuse_operation.pin_mut().Shape())
            }
            (Some(_), None) => self.clone(),
            (None, Some(_)) => other.clone(),
            (None, None) => self.clone(),
        }
    }
}
