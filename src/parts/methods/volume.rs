use opencascade_sys::ffi;
use uom::si::f64::Volume;
use uom::si::volume::cubic_meter;

use crate::Part;

impl Part {
    /// Return the volume occupied by this `Part` in cubic meters.
    ///
    /// ```rust
    /// use anvil::{Cuboid, IntoLength};
    /// use uom::si::volume::cubic_meter;
    /// use uom::si::f64::Volume;
    /// use approx::assert_relative_eq;
    ///
    /// let cuboid = Cuboid::from_dim(1.m(), 1.m(), 1.m());
    /// assert_relative_eq!(cuboid.volume().value, Volume::new::<cubic_meter>(1.).value);
    /// ```
    pub fn volume(&self) -> Volume {
        match &self.inner {
            Some(inner) => {
                let mut gprops = ffi::GProp_GProps_ctor();
                ffi::BRepGProp_VolumeProperties(inner, gprops.pin_mut());
                Volume::new::<cubic_meter>(gprops.Mass())
            }
            None => Volume::new::<cubic_meter>(0.),
        }
    }
}
