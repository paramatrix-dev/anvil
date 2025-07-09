use uom::si::volume::cubic_meter;

use crate::Part;

impl Part {
    /// Construct an empty `Part` which can be used for merging with other parts.
    ///
    /// ```rust
    /// use anvil::Part;
    /// use uom::si::volume::cubic_meter;
    /// use uom::si::f64::Volume;
    ///
    /// let part = Part::empty();
    /// assert_eq!(part.volume(), Volume::new::<cubic_meter>(0.));
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
        self.volume().get::<cubic_meter>() < 1e-9
    }
}
