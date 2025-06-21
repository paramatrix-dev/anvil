use opencascade_sys::ffi;

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
    /// use approx::assert_relative_eq;
    ///
    /// let part = Cuboid::from_dim(1.m(), 2.m(), 3.m());
    /// assert_eq!(part.center(), Ok(point!(0, 0, 0)));
    /// assert_relative_eq!(part.volume(), 6.);
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
    /// use approx::assert_relative_eq;
    ///
    /// let part = Cuboid::from_corners(point!(0, 0, 0), point!(2.m(), 2.m(), 2.m()));
    /// assert_eq!(part.center(), Ok(point!(1.m(), 1.m(), 1.m())));
    /// assert_relative_eq!(part.volume(), 8.);
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

        let min_x = corner1.x().min(&corner2.x()).m();
        let min_y = corner1.y().min(&corner2.y()).m();
        let min_z = corner1.z().min(&corner2.z()).m();
        let max_x = corner1.x().max(&corner2.x()).m();
        let max_y = corner1.y().max(&corner2.y()).m();
        let max_z = corner1.z().max(&corner2.z()).m();

        let point = ffi::new_point(min_x, min_y, min_z);
        let mut cuboid =
            ffi::BRepPrimAPI_MakeBox_ctor(&point, max_x - min_x, max_y - min_y, max_z - min_z);

        Part::from_occt(cuboid.pin_mut().Shape())
    }
    /// Construct a centered cuboidal `Part` directly from the x, y, and z meter values.
    ///
    /// This function is primarily intended to simplify tests and should not be exptected in
    /// similar structs.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cuboid, IntoLength, Part};
    ///
    /// assert_eq!(
    ///     Cuboid::from_m(1., 2., 3.),
    ///     Cuboid::from_dim(1.m(), 2.m(), 3.m())
    /// )
    /// ```
    pub fn from_m(x: f64, y: f64, z: f64) -> Part {
        Self::from_dim(Length::from_m(x), Length::from_m(y), Length::from_m(z))
    }
    /// Construct a centered cuboidal `Part` directly from the x, y, and z millimeter values.
    ///
    /// This function is primarily intended to simplify tests and should not be exptected in
    /// similar structs.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cuboid, IntoLength, Part};
    ///
    /// assert_eq!(
    ///     Cuboid::from_mm(1., 2., 3.),
    ///     Cuboid::from_dim(1.mm(), 2.mm(), 3.mm())
    /// )
    /// ```
    pub fn from_mm(x: f64, y: f64, z: f64) -> Part {
        Self::from_dim(Length::from_mm(x), Length::from_mm(y), Length::from_mm(z))
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
