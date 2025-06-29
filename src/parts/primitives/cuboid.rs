use opencascade_sys::ffi;
use uom::si::length::meter;

use crate::{Length, Part, Point, core::is_zero, point};

/// Builder for a cuboidal `Part`.
///
/// While the `Cuboid` struct itself is not used, its constructor methods like `Cuboid::from_dim()`
/// can be used to build this primitive `Part`.
#[derive(Debug, PartialEq, Clone)]
pub struct Cuboid;
impl Cuboid {
    /// Construct a centered cuboidal `Part` from the x, y, and z dimensions.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cuboid, IntoLength, Part, point};
    /// use uom::si::volume::cubic_meter;
    /// use uom::si::f64::Volume;
    ///
    /// let part = Cuboid::from_dim(1.m(), 2.m(), 3.m());
    /// assert_eq!(part.center(), Ok(point!(0, 0, 0)));
    /// assert_eq!(part.volume(), Volume::new::<cubic_meter>(6.));
    /// ```
    pub fn from_dim(x: Length, y: Length, z: Length) -> Part {
        Self::from_corners(
            point!(x * -0.5, y * -0.5, z * -0.5),
            point!(x * 0.5, y * 0.5, z * 0.5),
        )
    }
    /// Construct a centered cuboidal `Part` from its corner locations.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cuboid, IntoLength, Part, point};
    /// use uom::si::volume::cubic_meter;
    /// use uom::si::f64::Volume;
    /// use approx::assert_relative_eq;
    ///
    /// let part = Cuboid::from_corners(point!(0, 0, 0), point!(2.m(), 2.m(), 2.m()));
    /// assert_eq!(part.center(), Ok(point!(1.m(), 1.m(), 1.m())));
    /// assert_relative_eq!(part.volume().value, Volume::new::<cubic_meter>(8.).value);
    /// ```
    pub fn from_corners(corner1: Point<3>, corner2: Point<3>) -> Part {
        let volume_is_zero = is_zero(&[
            corner1.x() - corner2.x(),
            corner1.y() - corner2.y(),
            corner1.z() - corner2.z(),
        ]);
        if volume_is_zero {
            return Part::empty();
        }

        let min_x = corner1.x().min(corner2.x()).get::<meter>();
        let min_y = corner1.y().min(corner2.y()).get::<meter>();
        let min_z = corner1.z().min(corner2.z()).get::<meter>();
        let max_x = corner1.x().max(corner2.x()).get::<meter>();
        let max_y = corner1.y().max(corner2.y()).get::<meter>();
        let max_z = corner1.z().max(corner2.z()).get::<meter>();

        let point = ffi::new_point(min_x, min_y, min_z);
        let mut cuboid =
            ffi::BRepPrimAPI_MakeBox_ctor(&point, max_x - min_x, max_y - min_y, max_z - min_z);

        Part::from_occt(cuboid.pin_mut().Shape())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IntoLength;

    #[test]
    fn from_dim_empty() {
        assert_eq!(Cuboid::from_dim(0.m(), 1.m(), 1.m()), Part::empty());
        assert_eq!(Cuboid::from_dim(1.m(), 0.m(), 1.m()), Part::empty());
        assert_eq!(Cuboid::from_dim(1.m(), 1.m(), 0.m()), Part::empty())
    }
}
