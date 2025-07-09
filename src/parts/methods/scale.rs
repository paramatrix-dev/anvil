use opencascade_sys::ffi;

use crate::Part;

impl Part {
    /// Return a clone of this `Part` with the size scaled by a factor.
    ///
    /// ```rust
    /// use anvil::{Cuboid, IntoLength};
    ///
    /// let cuboid = Cuboid::from_dim(1.m(), 1.m(), 1.m());
    /// assert_eq!(
    ///     cuboid.scale(2.),
    ///     Cuboid::from_dim(2.m(), 2.m(), 2.m())
    /// )
    /// ```
    pub fn scale(&self, factor: f64) -> Self {
        match &self.inner {
            Some(inner) => {
                let mut transform = ffi::new_transform();
                transform.pin_mut().SetScale(
                    &self.center().expect("shape is not empty").to_occt_point(),
                    factor,
                );
                let mut operation = ffi::BRepBuilderAPI_Transform_ctor(inner, &transform, false);
                Self::from_occt(operation.pin_mut().Shape())
            }
            None => Self { inner: None },
        }
    }
}
