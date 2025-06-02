use crate::{Length, Rectangle, Sketch};

/// Builder for a square `Sketch`.
///
/// While the `Square` struct itself is not used, its constructor methods like `Square::from_size()`
/// can be used to build this primitive `Sketch`.
#[derive(Debug, PartialEq, Clone)]
pub struct Square;
impl Square {
    /// Construct a centered cubic `Sketch` from the length on every side.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Square, IntoLength, Sketch, point};
    ///
    /// let Sketch = Square::from_size(1.m());
    /// assert_eq!(Sketch.center(), Ok(point!(0, 0)));
    /// assert!((Sketch.area() - 1.).abs() < 1e-5);
    /// ```
    pub fn from_size(size: Length) -> Sketch {
        Rectangle::from_dim(size, size)
    }
}
