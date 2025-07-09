use crate::{FaceIterator, Part};

impl Part {
    /// Return the faces spanned by this `Part`.
    ///
    /// ```rust
    /// use anvil::{Cube, Cylinder, IntoLength, Sphere};
    ///
    /// assert_eq!(Cube::from_size(1.m()).faces().len(), 6);
    /// assert_eq!(Cylinder::from_radius(1.m(), 1.m()).faces().len(), 3);
    /// assert_eq!(Sphere::from_radius(1.m()).faces().len(), 1);
    /// ```
    pub fn faces(&self) -> FaceIterator {
        self.into()
    }
}
