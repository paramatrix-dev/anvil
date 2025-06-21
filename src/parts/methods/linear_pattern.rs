use crate::{Axis, Length, Part, Point};

impl Part {
    /// Create multiple instances of the `Part` spaced evenly until a point.
    ///
    /// ```rust
    /// use anvil::{Cuboid, IntoLength, point};
    ///
    /// let cuboid = Cuboid::from_dim(1.m(), 1.m(), 1.m());
    /// assert_eq!(
    ///     cuboid.linear_pattern(point!(4.m(), 0.m(), 0.m()), 5),
    ///     cuboid
    ///         .add(&cuboid.move_to(point!(1.m(), 0.m(), 0.m())))
    ///         .add(&cuboid.move_to(point!(2.m(), 0.m(), 0.m())))
    ///         .add(&cuboid.move_to(point!(3.m(), 0.m(), 0.m())))
    ///         .add(&cuboid.move_to(point!(4.m(), 0.m(), 0.m())))
    /// )
    /// ```
    pub fn linear_pattern(&self, until: Point<3>, instances: u8) -> Self {
        let start = match self.center() {
            Ok(p) => p,
            Err(_) => return self.clone(),
        };
        let axis = match Axis::<3>::between(start, until) {
            Ok(axis) => axis,
            Err(_) => return self.clone(),
        };

        let len_step = (start - until).distance_to(Point::<3>::origin()) / instances as f64;
        let mut new_part = self.clone();
        let mut pos = Length::zero();
        for _ in 0..instances {
            pos = pos + len_step;
            new_part = new_part.add(&self.move_to(axis.point_at(pos)));
        }
        new_part
    }
}
