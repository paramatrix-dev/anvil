use std::fmt::Debug;

use crate::Part;

impl Debug for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Shape")
            .field("stl", &self.stl().expect(""))
            .finish()
    }
}
