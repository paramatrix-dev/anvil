use opencascade_sys::ffi;

use crate::Part;

impl Part {
    /// Return the volume occupied by this `Part` in cubic meters.
    ///
    /// ```rust
    /// use anvil::{Cuboid, IntoLength};
    /// use approx::assert_relative_eq;
    ///
    /// let cuboid = Cuboid::from_dim(1.m(), 1.m(), 1.m());
    /// assert_relative_eq!(cuboid.volume(), 1.);
    /// ```
    pub fn volume(&self) -> f64 {
        match &self.inner {
            Some(inner) => {
                let mut gprops = ffi::GProp_GProps_ctor();
                ffi::BRepGProp_VolumeProperties(inner, gprops.pin_mut());
                gprops.Mass()
            }
            None => 0.,
        }
    }
}
