#![doc = "A CAD engine."]
#![allow(clippy::approx_constant)]
#![warn(missing_docs)]

mod core;
mod errors;
mod parts;
mod sketches;

pub use core::{
    Angle, Axis, Dir, Edge, IntoAngle, IntoF64, IntoLength, Length, Path, Plane, Point,
};
pub use errors::Error;
pub use parts::{
    Part,
    primitives::{Cube, Cuboid, Cylinder, Sphere},
};
pub use sketches::{
    Sketch,
    primitives::{Circle, Rectangle, Square},
};
