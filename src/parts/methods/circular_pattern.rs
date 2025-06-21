use crate::{Axis, IntoAngle, Part};

impl Part {
    /// Create multiple instances of the `Part` spaced evenly around a point.
    ///
    /// ```rust
    /// use anvil::{Axis, Cuboid, IntoAngle, IntoLength, point};
    ///
    /// let cuboid = Cuboid::from_corners(point!(1.m(), 1.m(), 0.m()), point!(2.m(), 2.m(), 1.m()));
    /// assert_eq!(
    ///     cuboid.circular_pattern(Axis::<3>::z(), 4),
    ///     cuboid
    ///         .add(&cuboid.rotate_around(Axis::<3>::z(), 90.deg()))
    ///         .add(&cuboid.rotate_around(Axis::<3>::z(), 180.deg()))
    ///         .add(&cuboid.rotate_around(Axis::<3>::z(), 270.deg()))
    /// )
    /// ```
    pub fn circular_pattern(&self, around: Axis<3>, instances: u8) -> Self {
        let angle_step = 360.deg() / instances as f64;
        let mut new_shape = self.clone();
        let mut angle = 0.rad();
        for _ in 0..instances {
            new_shape = new_shape.add(&self.rotate_around(around, angle));
            angle = angle + angle_step;
        }
        new_shape
    }
}
