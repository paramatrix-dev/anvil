use opencascade_sys::ffi;

use crate::{Error, Length, Part, Point, point};

impl Part {
    /// Return the center of mass of the `Part`.
    ///
    /// If the `Part` is empty, an `Err(Error::EmptyPart)` is returned.
    ///
    /// ```rust
    /// use anvil::{Cuboid, IntoLength, point};
    ///
    /// let centered_cuboid = Cuboid::from_dim(1.m(), 1.m(), 1.m());
    /// assert_eq!(centered_cuboid.center(), Ok(point!(0, 0, 0)));
    ///
    /// let non_centered_cuboid = Cuboid::from_corners(
    ///     point!(0, 0, 0),
    ///     point!(2.m(), 2.m(), 2.m())
    /// );
    /// assert_eq!(non_centered_cuboid.center(), Ok(point!(1.m(), 1.m(), 1.m())));
    /// ```
    pub fn center(&self) -> Result<Point<3>, Error> {
        match &self.inner {
            Some(inner) => {
                let mut gprops = ffi::GProp_GProps_ctor();
                ffi::BRepGProp_VolumeProperties(inner, gprops.pin_mut());
                let centre_of_mass = ffi::GProp_GProps_CentreOfMass(&gprops);

                Ok(point!(
                    Length::from_m(round(centre_of_mass.X(), 9)),
                    Length::from_m(round(centre_of_mass.Y(), 9)),
                    Length::from_m(round(centre_of_mass.Z(), 9))
                ))
            }
            None => Err(Error::EmptyPart),
        }
    }
}

fn round(x: f64, n_digits: u8) -> f64 {
    (x * f64::from(10 ^ n_digits)).round() / f64::from(10 ^ n_digits)
}
#[cfg(test)]
mod tests {
    use crate::{Cuboid, IntoLength, point};

    #[test]
    fn centre_at_origin() {
        let cuboid = Cuboid::from_m(1., 1., 1.);
        assert_eq!(cuboid.center(), Ok(point!(0, 0, 0)))
    }

    #[test]
    fn centre_not_at_origin() {
        let cuboid = Cuboid::from_corners(point!(0, 0, 0), point!(2.m(), 2.m(), 2.m()));
        assert_eq!(cuboid.center(), Ok(point!(1.m(), 1.m(), 1.m())))
    }
}
