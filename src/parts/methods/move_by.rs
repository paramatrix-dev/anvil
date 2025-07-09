use crate::{Length, Part, point};

impl Part {
    /// Return a clone of this `Part` moved by a specified amount in each axis.
    ///
    /// ```rust
    /// use anvil::{Cuboid, IntoLength, point};
    ///
    /// let cuboid = Cuboid::from_dim(1.m(), 1.m(), 1.m());
    /// let moved_cuboid = cuboid
    ///     .move_by(1.m(), 0.m(), 3.m())
    ///     .move_by(0.m(), 1.m(), 0.m());
    /// assert_eq!(
    ///     moved_cuboid.center(),
    ///     Ok(point!(1.m(), 1.m(), 3.m()))
    /// )
    /// ```
    pub fn move_by(&self, dx: Length, dy: Length, dz: Length) -> Self {
        let center = match self.center() {
            Ok(c) => c,
            Err(_) => return self.clone(),
        };
        self.move_to(center + point!(dx, dy, dz))
    }
}
