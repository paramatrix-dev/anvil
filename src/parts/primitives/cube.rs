use crate::{Cuboid, Length, Part};

/// Builder for a cubic `Part`.
///
/// While the `Cube` struct itself is not used, its constructor methods like `Cube::from_size()`
/// can be used to build this primitive `Part`.
#[derive(Debug, PartialEq, Clone)]
pub struct Cube;
impl Cube {
    /// Construct a centered cubic `Part` from the length on every side.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Cube, IntoLength, Part, point};
    /// use uom::si::volume::cubic_meter;
    /// use uom::si::f64::Volume;
    /// use approx::assert_relative_eq;
    ///
    /// let part = Cube::from_size(1.m());
    /// assert_eq!(part.center(), Ok(point!(0, 0, 0)));
    /// assert_relative_eq!(part.volume().value, Volume::new::<cubic_meter>(1.).value);
    /// ```
    pub fn from_size(size: Length) -> Part {
        Cuboid::from_dim(size, size, size)
    }
}
