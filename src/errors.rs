use std::{error::Error as StdError, fmt, path::PathBuf};

use crate::quantities::Dir3D;

/// The errors that can occurr.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Occurs when a function that requires a non-empty `Part` is called on an empty one.
    EmptyPart,

    /// Occurs when a function that requires a non-empty `Sketch` is called on an empty one.
    EmptySketch,

    /// Occurs when a `Part` could not be written to a .step file at a given path.
    StepWrite(PathBuf),

    /// Occurs when a `Part` could not be written to a .stl file at a given path.
    StlWrite(PathBuf),

    /// Occurs when an operation that requires a length is performed on a `Dir3D` with a magnitude of zero.
    ZeroVector,

    /// Occurs when two vectors that are required to be orthogonal, are not.
    VectorsNotOrthogonal(Dir3D, Dir3D),
}
impl StdError for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Under construction")
    }
}
