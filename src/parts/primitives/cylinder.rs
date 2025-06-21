use crate::{Length, Part, core::is_zero};
use opencascade_sys::ffi;

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
    /// use approx::assert_relative_eq;
    ///
    /// let part = Cylinder::from_radius(1.m(), 2.m());
    /// assert_eq!(part.center(), Ok(Point::<3>::origin()));
    /// assert_relative_eq!(part.volume(), 6.283185307179587);
    /// ```
    pub fn from_radius(radius: Length, height: Length) -> Part {
        if is_zero(&[radius, height]) {
            return Part::empty();
        }
        let axis = ffi::gp_Ax2_ctor(
            &ffi::new_point(0., 0., -height.m() / 2.),
            &ffi::gp_Dir_ctor(0., 0., 1.),
        );
        let mut make = ffi::BRepPrimAPI_MakeCylinder_ctor(&axis, radius.m(), height.m());
        Part::from_occt(make.pin_mut().Shape())
    }

    /// Construct a centered cylindrical `Part` from a given diameter.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cylinder, IntoLength, Point, Part};
    /// use approx::assert_relative_eq;
    ///
    /// let part = Cylinder::from_diameter(1.m(), 2.m());
    /// assert_eq!(part.center(), Ok(Point::<3>::origin()));
    /// assert_relative_eq!(part.volume(), 1.5707963267948968);
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
