use opencascade_sys::ffi;
use uom::si::length::meter;

use crate::{Length, Part, core::is_zero};

/// Builder for a cylindrical `Part`.
///
/// While the `Cylinder` struct itself is not used, its constructor methods like
/// `Cylinder::from_radius()` can be used to build this primitive `Part`.
#[derive(Debug, PartialEq, Clone)]
pub struct Cylinder;
impl Cylinder {
    /// Construct a centered cylindrical `Part` from a given radius.
    ///
    /// ```rust
    /// use anvil::{Cylinder, IntoLength, Point, Part};
    /// use uom::si::volume::cubic_meter;
    /// use uom::si::f64::Volume;
    ///
    /// let part = Cylinder::from_radius(1.m(), 2.m());
    /// assert_eq!(part.center(), Ok(Point::<3>::origin()));
    /// assert_eq!(part.volume(), Volume::new::<cubic_meter>(6.283185307179587));
    /// ```
    pub fn from_radius(radius: Length, height: Length) -> Part {
        if is_zero(&[radius, height]) {
            return Part::empty();
        }
        let axis = ffi::gp_Ax2_ctor(
            &ffi::new_point(0., 0., -height.get::<meter>() / 2.),
            &ffi::gp_Dir_ctor(0., 0., 1.),
        );
        let mut make =
            ffi::BRepPrimAPI_MakeCylinder_ctor(&axis, radius.get::<meter>(), height.get::<meter>());
        Part::from_occt(make.pin_mut().Shape())
    }

    /// Construct a centered cylindrical `Part` from a given diameter.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cylinder, IntoLength, Point, Part};
    /// use uom::si::volume::cubic_meter;
    /// use uom::si::f64::Volume;
    /// use approx::assert_relative_eq;
    ///
    /// let part = Cylinder::from_diameter(1.m(), 2.m());
    /// assert_eq!(part.center(), Ok(Point::<3>::origin()));
    /// assert_eq!(part.volume().value, Volume::new::<cubic_meter>(1.5707963267948968).value);
    /// ```
    pub fn from_diameter(diameter: Length, height: Length) -> Part {
        Self::from_radius(diameter / 2., height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IntoLength;

    #[test]
    fn from_radius_empty() {
        assert_eq!(Cylinder::from_radius(0.m(), 1.m()), Part::empty());
        assert_eq!(Cylinder::from_radius(1.m(), 0.m()), Part::empty());
    }

    #[test]
    fn from_diameter_empty() {
        assert_eq!(Cylinder::from_diameter(0.m(), 1.m()), Part::empty());
        assert_eq!(Cylinder::from_diameter(1.m(), 0.m()), Part::empty());
    }
}
