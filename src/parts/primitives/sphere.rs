use crate::{Length, Part, quantities::is_zero};
use opencascade_sys::ffi;

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
    /// use anvil::{Sphere, IntoLength, Point3D, Part};
    ///
    /// let part = Sphere::from_radius(1.m());
    /// assert_eq!(part.center(), Ok(Point3D::origin()));
    /// assert!((part.volume() - 4.18879).abs() < 1e-5);
    /// ```
    pub fn from_radius(radius: Length) -> Part {
        if is_zero(&[radius]) {
            return Part::empty();
        }
        let mut make_sphere = ffi::BRepPrimAPI_MakeSphere_ctor(radius.m());
        Part::from_occt(make_sphere.pin_mut().Shape())
    }
    /// Construct a centered spherical `Part` from a given diameter.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Sphere, IntoLength, Point3D, Part};
    ///
    /// let part = Sphere::from_diameter(1.m());
    /// assert_eq!(part.center(), Ok(Point3D::origin()));
    /// assert!((part.volume() - 0.523599).abs() < 1e-5);
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
        assert!(Sphere::from_radius(0.m()) == Part::empty())
    }

    #[test]
    fn from_diameter_empty() {
        assert!(Sphere::from_diameter(0.m()) == Part::empty())
    }
}
