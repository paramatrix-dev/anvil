use std::ops::Mul;

use cxx::UniquePtr;
use opencascade_sys::ffi;

use crate::Error;

use super::{Length, Point3D};

/// A direction in 3D space with a length of 1.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dir3D {
    x: f64,
    y: f64,
    z: f64,
}
impl Dir3D {
    /// Construct a `Dir3D` from the directional components.
    ///
    /// Returns an Error::ZeroVector if all of the axis components are zero.
    ///
    /// ```rust
    /// use anvil::Dir3D;
    ///
    /// let dir3 = Dir3D::try_from(1., 2., 2.).expect("");
    /// assert_eq!(dir3.x(), 1. / 3.);
    /// assert_eq!(dir3.y(), 2. / 3.);
    /// assert_eq!(dir3.z(), 2. / 3.);
    /// ```
    pub fn try_from(x: f64, y: f64, z: f64) -> Result<Self, Error> {
        let magnitude = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        if magnitude == 0. {
            return Err(Error::ZeroVector);
        }
        Ok(Dir3D {
            x: x / magnitude,
            y: y / magnitude,
            z: z / magnitude,
        })
    }

    /// Return the x-component of this `Dir3D`.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Return the y-component of this `Dir3D`.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Return the z-component of this `Dir3D`.
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Return the dot-product of this `Dir3D` with another.
    pub fn dot(&self, other: Dir3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    /// Return the cross-product of this `Dir3D` with another.
    pub fn cross(&self, other: Dir3D) -> Dir3D {
        Dir3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub(crate) fn to_occt_dir(self) -> UniquePtr<ffi::gp_Dir> {
        ffi::gp_Dir_ctor(self.x, self.y, self.z)
    }
}

impl Mul<Length> for Dir3D {
    type Output = Point3D;
    /// Multiply this `Dir3D` with a `Length` to get a `Poin3D`.
    ///
    /// ```rust
    /// use anvil::{Dir3D, length, point};
    ///
    /// let dir3 = Dir3D::try_from(1., 0., 0.).unwrap();
    /// assert_eq!(
    ///     dir3 * length!(2 m),
    ///     point!(2 m, 0 m, 0 m)
    /// )
    /// ```
    fn mul(self, other: Length) -> Point3D {
        Point3D {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Dir3D> for Length {
    type Output = Point3D;
    /// Multiply this `Length` with a `Dir3D` to get a `Poin3D`.
    ///
    /// ```rust
    /// use anvil::{Dir3D, length, point};
    ///
    /// let dir3 = Dir3D::try_from(1., 0., 0.).unwrap();
    /// assert_eq!(
    ///     length!(2 m) * dir3,
    ///     point!(2 m, 0 m, 0 m)
    /// )
    /// ```
    fn mul(self, other: Dir3D) -> Point3D {
        other * self
    }
}

impl Mul<&Length> for Dir3D {
    type Output = Point3D;
    /// Multiply this `Dir3D` with a `&Length` to get a `Poin3D`.
    ///
    /// ```rust
    /// use anvil::{Dir3D, length, point};
    ///
    /// let dir3 = Dir3D::try_from(1., 0., 0.).unwrap();
    /// assert_eq!(
    ///     dir3 * &length!(2 m),
    ///     point!(2 m, 0 m, 0 m)
    /// )
    /// ```
    fn mul(self, other: &Length) -> Point3D {
        Point3D {
            x: self.x * *other,
            y: self.y * *other,
            z: self.z * *other,
        }
    }
}

impl Mul<Dir3D> for &Length {
    type Output = Point3D;
    /// Multiply this `&Length` with a `Dir3D` to get a `Poin3D`.
    ///
    /// ```rust
    /// use anvil::{Dir3D, length, point};
    ///
    /// let dir3 = Dir3D::try_from(1., 0., 0.).unwrap();
    /// assert_eq!(
    ///     &length!(2 m) * dir3,
    ///     point!(2 m, 0 m, 0 m)
    /// )
    /// ```
    fn mul(self, other: Dir3D) -> Point3D {
        other * self
    }
}
