#![doc = "A CAD engine."]
#![allow(clippy::approx_constant)]
#![warn(missing_docs)]

mod core;
mod errors;
mod parts;
mod quantities;
mod sketches;

pub use core::{Axis, Dir, Point};
pub use errors::Error;
pub use parts::{
    Part,
    primitives::{Cuboid, Cylinder, Sphere},
};
pub use quantities::{
    Angle, Axis2D, Axis3D, Dir2D, Dir3D, IntoAngle, IntoF64, IntoLength, Length, Plane, Point2D,
    Point3D,
};
pub use sketches::{
    Edge, Path, Sketch,
    primitives::{Circle, Rectangle},
};
