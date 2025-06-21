use crate::Part;

impl Part {
    /// Construct an empty `Part` which can be used for merging with other parts.
    ///
    /// ```rust
    /// use anvil::Part;
    ///
    /// let part = Part::empty();
    /// assert_eq!(part.volume(), 0.);
    /// ```
    pub fn empty() -> Self {
        Self { inner: None }
    }

    /// Return true if this `Part` is empty.
    ///
    /// ```rust
    /// use anvil::{Cube, IntoLength, Part};
    ///
    /// let cube = Cube::from_size(1.m());
    /// assert!(!cube.is_empty());
    /// assert!(cube.subtract(&cube).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.volume() < 1e-9
    }
}
