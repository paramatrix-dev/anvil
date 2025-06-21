use opencascade_sys::ffi;

use crate::{Angle, Axis, Part};

impl Part {
    /// Return a clone of this `Part` rotated around an `Axis::<3>`.
    ///
    /// For positive angles, the right-hand-rule applies for the direction of rotation.
    ///
    /// ```rust
    /// use anvil::{Axis, Cuboid, IntoAngle, IntoLength, point};
    ///
    /// let cuboid = Cuboid::from_corners(point!(0, 0, 0), point!(1.m(), 1.m(), 1.m()));
    /// assert_eq!(
    ///     cuboid.rotate_around(Axis::<3>::x(), 90.deg()),
    ///     Cuboid::from_corners(point!(0, 0, 0), point!(1.m(), -1.m(), 1.m()))
    /// )
    /// ```
    pub fn rotate_around(&self, axis: Axis<3>, angle: Angle) -> Self {
        match &self.inner {
            Some(inner) => {
                let mut transform = ffi::new_transform();
                transform
                    .pin_mut()
                    .SetRotation(&axis.to_occt_ax1(), angle.rad());
                let mut operation = ffi::BRepBuilderAPI_Transform_ctor(inner, &transform, false);
                Self::from_occt(operation.pin_mut().Shape())
            }
            None => Self { inner: None },
        }
    }
}
