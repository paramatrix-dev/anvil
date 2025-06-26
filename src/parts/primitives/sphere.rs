use crate::{Length, Part, core::is_zero};
use opencascade_sys::ffi;
use uom::si::length::meter;

/// Builder for a spherical `Part`.
///
/// While the `Sphere` struct itself is not used, its constructor methods like `Sphere::from_radius()`
/// can be used to build this primitive `Part`.
#[derive(Debug, PartialEq, Clone)]
pub struct Sphere;
impl Sphere {
    /// Construct a centered spherical `Part` from a given radius.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Sphere, IntoLength, Point, Part};
    /// use approx::assert_relative_eq;
    ///
    /// let part = Sphere::from_radius(1.m());
    /// assert_eq!(part.center(), Ok(Point::<3>::origin()));
    /// assert_relative_eq!(part.volume(), 4.188790204786391);
    /// ```
    pub fn from_radius(radius: Length) -> Part {
        if is_zero(&[radius]) {
            return Part::empty();
        }

        let axis = ffi::gp_Ax2_ctor(&ffi::new_point(0., 0., 0.), &ffi::gp_Dir_ctor(0., 0., 1.));
        let mut make_sphere =
            ffi::BRepPrimAPI_MakeSphere_ctor(&axis, radius.get::<meter>(), std::f64::consts::TAU);
        Part::from_occt(make_sphere.pin_mut().Shape())
    }
    /// Construct a centered spherical `Part` from a given diameter.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Sphere, IntoLength, Point, Part};
    /// use approx::assert_relative_eq;
    ///
    /// let part = Sphere::from_diameter(1.m());
    /// assert_eq!(part.center(), Ok(Point::<3>::origin()));
    /// assert_relative_eq!(part.volume(), 0.5235987755982989);
    /// ```
    pub fn from_diameter(diameter: Length) -> Part {
        Self::from_radius(diameter / 2.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IntoLength;

    #[test]
    fn from_radius_empty() {
        assert_eq!(Sphere::from_radius(0.m()), Part::empty())
    }

    #[test]
    fn from_diameter_empty() {
        assert_eq!(Sphere::from_diameter(0.m()), Part::empty())
    }
}
