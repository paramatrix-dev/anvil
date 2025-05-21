use std::{
    fmt::Debug,
    fs,
    io::{self, BufRead},
    path::Path,
};

use cxx::UniquePtr;
use opencascade_sys::ffi;
use tempfile::NamedTempFile;

use crate::{Angle, Axis, Error, Length, Point3D, angle};

/// A 3D object in space.
pub struct Part {
    pub(crate) inner: Option<UniquePtr<ffi::TopoDS_Shape>>,
}
impl Part {
    /// Construct an empty `Part` which can be used for merging with other parts.
    ///
    /// # Example
    /// ```rust
    /// use anvil::Part;
    ///
    /// let part = Part::empty();
    /// assert_eq!(part.volume(), 0.);
    /// ```
    pub fn empty() -> Self {
        Self { inner: None }
    }

    /// Merge this `Part` with another.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cuboid, Point3D};
    ///
    /// let cuboid1 = Cuboid::from_corners(
    ///     Point3D::origin(),
    ///     Point3D::from_m(1., 1., 1.)
    /// );
    /// let cuboid2 = Cuboid::from_corners(
    ///     Point3D::from_m(0., 0., 1.),
    ///     Point3D::from_m(1., 1., 2.)
    /// );
    /// assert!(cuboid1.add(&cuboid2) == Cuboid::from_corners(Point3D::origin(), Point3D::from_m(1., 1., 2.)));
    /// ```
    pub fn add(&self, other: &Self) -> Self {
        match (&self.inner, &other.inner) {
            (Some(self_inner), Some(other_inner)) => {
                let mut fuse_operation = ffi::BRepAlgoAPI_Fuse_ctor(self_inner, other_inner);
                Self::from_occt(fuse_operation.pin_mut().Shape())
            }
            (Some(_), None) => self.clone(),
            (None, Some(_)) => other.clone(),
            (None, None) => self.clone(),
        }
    }

    /// Create multiple instances of the `Sketch` spaced evenly around a point.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{angle, Axis, Cuboid, point};
    ///
    /// let cuboid = Cuboid::from_corners(point!(1 m, 1 m, 0 m), point!(2 m, 2 m, 1 m));
    /// assert_eq!(
    ///     cuboid.circular_pattern(Axis::z(), 4),
    ///     cuboid
    ///         .add(&cuboid.rotate_around(Axis::z(), angle!(90 deg)))
    ///         .add(&cuboid.rotate_around(Axis::z(), angle!(180 deg)))
    ///         .add(&cuboid.rotate_around(Axis::z(), angle!(270 deg)))
    /// )
    /// ```
    pub fn circular_pattern(&self, around: Axis, instances: u8) -> Self {
        let angle_step = angle!(360 deg) / instances as f64;
        let mut new_shape = self.clone();
        let mut angle = angle!(0 deg);
        for _ in 0..instances {
            new_shape = new_shape.add(&self.rotate_around(around.clone(), angle));
            angle = angle + angle_step;
        }
        new_shape
    }
    /// Return the `Part` that is created from the overlapping volume between this one and another.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cuboid, length};
    ///
    /// let cuboid1 = Cuboid::from_dim(length!(5 m), length!(5 m), length!(1 m));
    /// let cuboid2 = Cuboid::from_dim(length!(1 m), length!(1 m), length!(5 m));
    /// assert!(
    ///     cuboid1.intersect(&cuboid2) == Cuboid::from_dim(length!(1 m), length!(1 m), length!(1 m))
    /// )
    /// ```
    pub fn intersect(&self, other: &Self) -> Self {
        match (&self.inner, &other.inner) {
            (Some(self_inner), Some(other_inner)) => {
                let mut fuse_operation = ffi::BRepAlgoAPI_Common_ctor(self_inner, other_inner);
                Self::from_occt(fuse_operation.pin_mut().Shape())
            }
            _ => Part { inner: None },
        }
    }
    /// Return a clone of this `Part` with the center moved to a specified point.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cuboid, length, Point3D};
    ///
    /// let cuboid = Cuboid::from_dim(length!(1 m), length!(1 m), length!(1 m));
    /// let moved_cuboid = cuboid.move_to(Point3D::from_m(2., 2., 2.));
    /// assert_eq!(cuboid.center(), Ok(Point3D::origin()));
    /// assert_eq!(moved_cuboid.center(), Ok(Point3D::from_m(2., 2., 2.)));
    /// ```
    pub fn move_to(&self, loc: Point3D) -> Self {
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
    /// Return a clone of this `Part` rotated around an `Axis`.
    ///
    /// For positive angles, the right-hand-rule applies for the direction of rotation.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{angle, Axis, Cuboid, point, Point3D};
    ///
    /// let cuboid = Cuboid::from_corners(Point3D::origin(), point!(1 m, 1 m , 1 m));
    /// assert_eq!(
    ///     cuboid.rotate_around(Axis::x(), angle!(90 deg)),
    ///     Cuboid::from_corners(Point3D::origin(), point!(1 m, -1 m , 1 m))
    /// )
    /// ```
    pub fn rotate_around(&self, axis: Axis, angle: Angle) -> Self {
        match &self.inner {
            Some(inner) => {
                let mut transform = ffi::new_transform();
                transform
                    .pin_mut()
                    .SetRotation(&axis.to_occt_ax1(), angle.rad());
                let mut operation = ffi::BRepBuilderAPI_Transform_ctor(inner, &transform, false);
                Self::from_occt(operation.pin_mut().Shape())
            }
            None => Self { inner: None },
        }
    }
    /// Return a clone of this `Part` with the size scaled by a factor.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cuboid, length};
    ///
    /// let cuboid = Cuboid::from_dim(length!(1 m), length!(1 m), length!(1 m));
    /// assert_eq!(
    ///     cuboid.scale(2.),
    ///     Cuboid::from_dim(length!(2 m), length!(2 m), length!(2 m))
    /// )
    /// ```
    pub fn scale(&self, factor: f64) -> Self {
        match &self.inner {
            Some(inner) => {
                let mut transform = ffi::new_transform();
                transform.pin_mut().SetScale(
                    &self.center().expect("shape is not empty").to_occt_point(),
                    factor,
                );
                let mut operation = ffi::BRepBuilderAPI_Transform_ctor(inner, &transform, false);
                Self::from_occt(operation.pin_mut().Shape())
            }
            None => Self { inner: None },
        }
    }
    /// Return a copy of this `Part` with the intersection of another removed.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cuboid, Point3D};
    ///
    /// let cuboid1 = Cuboid::from_corners(
    ///     Point3D::origin(),
    ///     Point3D::from_m(1., 1., 2.)
    /// );
    /// let cuboid2 = Cuboid::from_corners(
    ///     Point3D::from_m(0., 0., 1.),
    ///     Point3D::from_m(1., 1., 2.)
    /// );
    /// assert!(cuboid1.subtract(&cuboid2) == Cuboid::from_corners(Point3D::origin(), Point3D::from_m(1., 1., 1.)));
    /// ```
    pub fn subtract(&self, other: &Self) -> Self {
        match (&self.inner, &other.inner) {
            (Some(self_inner), Some(other_inner)) => {
                let mut fuse_operation = ffi::BRepAlgoAPI_Cut_ctor(self_inner, other_inner);
                Self::from_occt(fuse_operation.pin_mut().Shape())
            }
            (Some(_), None) => self.clone(),
            (None, _) => Part { inner: None },
        }
    }

    /// Return the volume occupied by this `Part` in cubic meters.
    ///
    /// Warning: the volume is susceptibility to floating point errors.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cuboid, length};
    ///
    /// let cuboid = Cuboid::from_dim(length!(1 m), length!(1 m), length!(1 m));
    /// assert!((cuboid.volume() - 1.).abs() < 1e-9)
    /// ```
    pub fn volume(&self) -> f64 {
        match &self.inner {
            Some(inner) => {
                let mut gprops = ffi::GProp_GProps_ctor();
                ffi::BRepGProp_VolumeProperties(inner, gprops.pin_mut());
                gprops.Mass()
            }
            None => 0.,
        }
    }
    /// Return the center of mass of the `Part`.
    ///
    /// If the `Part` is empty, an `Err(Error::EmptyPart)` is returned.
    ///
    /// # Examples
    /// ```rust
    /// use anvil::{Cuboid, length, Point3D};
    ///
    /// let centered_cuboid = Cuboid::from_dim(length!(1 m), length!(1 m), length!(1 m));
    /// assert_eq!(centered_cuboid.center(), Ok(Point3D::origin()));
    ///
    /// let non_centered_cuboid = Cuboid::from_corners(
    ///     Point3D::from_m(0., 0., 0.),
    ///     Point3D::from_m(2., 2., 2.)
    /// );
    /// assert_eq!(non_centered_cuboid.center(), Ok(Point3D::from_m(1., 1., 1.)));
    /// ```
    pub fn center(&self) -> Result<Point3D, Error> {
        match &self.inner {
            Some(inner) => {
                let mut gprops = ffi::GProp_GProps_ctor();
                ffi::BRepGProp_VolumeProperties(inner, gprops.pin_mut());
                let centre_of_mass = ffi::GProp_GProps_CentreOfMass(&gprops);

                Ok(Point3D {
                    x: Length::from_m(round(centre_of_mass.X(), 9)),
                    y: Length::from_m(round(centre_of_mass.Y(), 9)),
                    z: Length::from_m(round(centre_of_mass.Z(), 9)),
                })
            }
            None => Err(Error::EmptyPart),
        }
    }

    /// Write the `Part` to a file in the STEP format.
    pub fn write_step(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        match &self.scale(1000.).inner {
            Some(inner) => {
                let mut writer = ffi::STEPControl_Writer_ctor();
                let status = ffi::transfer_shape(writer.pin_mut(), inner);
                if status != ffi::IFSelect_ReturnStatus::IFSelect_RetDone {
                    return Err(Error::StepWrite(path.as_ref().to_path_buf()));
                }
                let status = ffi::write_step(
                    writer.pin_mut(),
                    path.as_ref().to_string_lossy().to_string(),
                );
                if status != ffi::IFSelect_ReturnStatus::IFSelect_RetDone {
                    return Err(Error::StepWrite(path.as_ref().to_path_buf()));
                }
            }
            None => return Err(Error::EmptyPart),
        }
        Ok(())
    }

    /// Write the `Part` to a file in the STL format.
    pub fn write_stl(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        match &self.inner {
            Some(inner) => {
                let mut writer = ffi::StlAPI_Writer_ctor();
                let mesh = ffi::BRepMesh_IncrementalMesh_ctor(inner, 0.0001);
                let success = ffi::write_stl(
                    writer.pin_mut(),
                    mesh.Shape(),
                    path.as_ref().to_string_lossy().to_string(),
                );
                if success {
                    Ok(())
                } else {
                    Err(Error::StlWrite(path.as_ref().to_path_buf()))
                }
            }
            None => Err(Error::EmptyPart),
        }
    }
    /// Return the STL lines that describe this `Part`.
    pub fn stl(&self) -> Result<Vec<String>, Error> {
        match &self.inner {
            Some(_) => {
                let temp_file = NamedTempFile::new().expect("could not create tempfile");
                let path = temp_file.path();

                self.write_stl(path)?;

                let file = fs::File::open(path).map_err(|_| Error::StlWrite(path.into()))?;
                let lines = io::BufReader::new(file)
                    .lines()
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| Error::StlWrite(path.into()))?;

                Ok(lines)
            }
            None => Err(Error::EmptyPart),
        }
    }

    pub(crate) fn from_occt(part: &ffi::TopoDS_Shape) -> Self {
        let inner = ffi::TopoDS_Shape_to_owned(part);
        Self { inner: Some(inner) }
    }
}

impl Clone for Part {
    fn clone(&self) -> Self {
        match &self.inner {
            Some(inner) => Self::from_occt(inner),
            None => Part { inner: None },
        }
    }
}

impl PartialEq for Part {
    fn eq(&self, other: &Self) -> bool {
        match (&self.inner, &other.inner) {
            (Some(_), Some(_)) => {
                let intersection = self.intersect(other);

                (intersection.volume() - self.volume()).abs() < intersection.volume() * 1e-7
                    && (intersection.volume() - other.volume()).abs() < intersection.volume() * 1e-7
            }
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }
}

impl Debug for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shape")
            .field("stl", &self.stl().expect(""))
            .finish()
    }
}

fn round(x: f64, n_digits: u8) -> f64 {
    (x * f64::from(10 ^ n_digits)).round() / f64::from(10 ^ n_digits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cuboid, Point3D, Sphere, length};

    #[test]
    fn eq_both_none() {
        assert!(Part::empty() == Part::empty())
    }

    #[test]
    fn eq_both_cuboid() {
        let cuboid1 = Cuboid::from_m(1., 1., 1.);
        let cuboid2 = Cuboid::from_m(1., 1., 1.);
        assert!(cuboid1 == cuboid2)
    }

    #[test]
    fn neq_both_cuboid() {
        let cuboid1 = Cuboid::from_m(1., 1., 1.);
        let cuboid2 = Cuboid::from_m(2., 2., 2.);
        assert!(cuboid1 != cuboid2)
    }

    #[test]
    fn eq_both_sphere() {
        let sphere1 = Sphere::from_radius(length!(2 m));
        let sphere2 = Sphere::from_radius(length!(2 m));
        assert!(sphere1 == sphere2)
    }

    #[test]
    fn neq_both_sphere() {
        let sphere1 = Sphere::from_radius(length!(1 m));
        let sphere2 = Sphere::from_radius(length!(2 m));
        assert!(sphere1 != sphere2)
    }

    #[test]
    fn move_to_deepcopied() {
        let cuboid1 = Cuboid::from_m(1., 1., 1.);
        let loc = Point3D::from_m(2., 2., 2.);
        let cuboid2 = cuboid1.move_to(loc);

        assert_eq!(cuboid1.center(), Ok(Point3D::origin()));
        assert_eq!(cuboid2.center(), Ok(loc));
    }

    #[test]
    fn volume() {
        let cuboid = Cuboid::from_m(1., 1., 1.);
        assert!((cuboid.volume() - 1.).abs() < 1e-9)
    }

    #[test]
    fn centre_of_mass_at_origin() {
        let cuboid = Cuboid::from_m(1., 1., 1.);
        assert_eq!(cuboid.center(), Ok(Point3D::from_m(0., 0., 0.)))
    }

    #[test]
    fn centre_of_mass_not_at_origin() {
        let cuboid = Cuboid::from_corners(Point3D::from_m(0., 0., 0.), Point3D::from_m(2., 2., 2.));
        assert_eq!(cuboid.center(), Ok(Point3D::from_m(1., 1., 1.)))
    }

    #[test]
    fn write_stl() {
        let cuboid = Cuboid::from_m(1., 2., 3.);
        assert!(
            cuboid
                .write_stl("/Users/tk/user/dev/anvil/local/out.stl")
                .is_ok()
        );
    }

    #[test]
    fn part_moved_twice() {
        let part = Cuboid::from_m(1., 1., 1.);
        assert_eq!(
            part.move_to(Point3D::from_m(1., 1., 1.))
                .move_to(Point3D::from_m(2., 2., 2.)),
            Cuboid::from_m(1., 1., 1.).move_to(Point3D::from_m(2., 2., 2.)),
        )
    }

    #[test]
    fn move_after_rotate_should_not_reset_rotate() {
        let part = Cuboid::from_m(1., 1., 2.);
        assert_eq!(
            part.rotate_around(Axis::y(), angle!(90 deg))
                .move_to(Point3D::origin()),
            Cuboid::from_m(2., 1., 1.)
        )
    }
}
