#![doc = "A CAD engine."]
#![allow(clippy::approx_constant)]
#![warn(missing_docs)]

mod errors;
mod parts;
mod quantities;
mod sketches;

pub use errors::Error;
pub use parts::{
    Part,
    primitives::{Cuboid, Cylinder, Sphere},
};
pub use quantities::{Axis, Length, Plane, Point2D, Point3D};
pub use sketches::{
    Edge, Path, Sketch,
    primitives::{Circle, Rectangle},
};
