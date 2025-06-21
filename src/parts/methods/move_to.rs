use opencascade_sys::ffi;

use crate::{Part, Point};

impl Part {
    /// Return a clone of this `Part` with the center moved to a specified point.
    ///
    /// ```rust
    /// use anvil::{Cuboid, IntoLength, point};
    ///
    /// let cuboid = Cuboid::from_dim(1.m(), 1.m(), 1.m());
    /// let moved_cuboid = cuboid.move_to(point!(2.m(), 2.m(), 2.m()));
    /// assert_eq!(cuboid.center(), Ok(point!(0, 0, 0)));
    /// assert_eq!(moved_cuboid.center(), Ok(point!(2.m(), 2.m(), 2.m())));
    /// ```
    pub fn move_to(&self, loc: Point<3>) -> Self {
        match &self.inner {
            Some(inner) => {
                let move_vec = (loc - self.center().unwrap()).to_occt_vec();
                let mut transform = ffi::new_transform();
                transform.pin_mut().set_translation_vec(&move_vec);
                let mut operation = ffi::BRepBuilderAPI_Transform_ctor(inner, &transform, false);
                Self::from_occt(operation.pin_mut().Shape())
            }
            None => Self { inner: None },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Axis, Cuboid, IntoAngle, IntoLength, point};

    #[test]
    fn move_to_deepcopied() {
        let cuboid1 = Cuboid::from_m(1., 1., 1.);
        let loc = point!(2.m(), 2.m(), 2.m());
        let cuboid2 = cuboid1.move_to(loc);

        assert_eq!(cuboid1.center(), Ok(Point::<3>::origin()));
        assert_eq!(cuboid2.center(), Ok(loc));
    }

    #[test]
    fn part_move_to_twice() {
        let part = Cuboid::from_m(1., 1., 1.);
        assert_eq!(
            part.move_to(point!(1.m(), 1.m(), 1.m()))
                .move_to(point!(-1.m(), -1.m(), -1.m())),
            Cuboid::from_m(1., 1., 1.).move_to(point!(-1.m(), -1.m(), -1.m())),
        )
    }

    #[test]
    fn move_after_rotate_should_not_reset_rotate() {
        let part = Cuboid::from_m(1., 1., 2.);
        assert_eq!(
            part.rotate_around(Axis::<3>::y(), 90.deg())
                .move_to(Point::<3>::origin()),
            Cuboid::from_m(2., 1., 1.)
        )
    }
}
