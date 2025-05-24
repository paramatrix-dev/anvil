#![doc = "A CAD engine."]
#![allow(clippy::approx_constant)]
#![warn(missing_docs)]

mod errors;
mod parts;
mod quantities;
mod sketches;

pub use errors::Error;
pub use parts::{
    primitives::{Cuboid, Cylinder, Sphere},
    Part,
};
pub use quantities::{Angle, Axis2D, Axis3D, Dir2D, Dir3D, Length, Plane, Point2D, Point3D};
pub use sketches::{
    primitives::{Circle, Rectangle},
    Edge, Path, Sketch,
};
