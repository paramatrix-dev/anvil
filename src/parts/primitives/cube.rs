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
    /// use approx::assert_relative_eq;
    ///
    /// let part = Cube::from_size(1.m());
    /// assert_eq!(part.center(), Ok(point!(0, 0, 0)));
    /// assert_relative_eq!(part.volume(), 1.);
    /// ```
    pub fn from_size(size: Length) -> Part {
        Cuboid::from_dim(size, size, size)
    }
}
