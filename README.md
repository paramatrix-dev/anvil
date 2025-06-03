# What is Anvil

Anvil is an intuitive, easy-to-use Rust crate for building 3D CAD models. It is built on the principles of:
- **consistent and predictable APIs**: APIs are consistent between 2D and 3D modelling and between different shapes
- **mandatory unit support**: Anvil forces you to specify units for all lengths and angles, giving you certainty about the true dimensions of your models
- **reliability by design**: unit and integration tests exist for almost all public APIs communicating functionality and ensuring reliability

# Installation

Anvil is not yet on crates.io, so to install it you need to run
```bash
cargo add --git "https://github.com/paramatrix-dev/anvil.git"
```
or add
```toml
anvil = { git = "https://github.com/paramatrix-dev/anvil.git", branch = "main" }
```
to your Cargo.toml `[dependencies]` section.

# Usage

The two main structs in Anvil are `anvil::Part` for 3D and `anvil::Sketch` for 2D models. Both have primitive constructor-structs like `anvil::Cuboid` for `Part` or `anvil::Rectangle` for `Sketch` which can be further designed with operations like `add`, `subtract`, and `interface`. This is how you would create a 2x2 Lego brick in Anvil:
```rust
use anvil::{Axis, Cuboid, Cylinder, IntoLength, Part, point};

let block_width = 16.mm();
let block_height = 9.6.mm();
let stud_height = 11.2.mm() - block_height;
let stud_distance = 8.mm();
let stud_diameter = 4.8.mm();
let thickness = 1.2.mm();
let tube_diameter = 6.5.mm();

let hollow_block_width = block_width - thickness;

let block = Cuboid::from_dim(block_width, block_width, block_height);
let studs = Cylinder::from_diameter(stud_diameter, stud_height)
    .move_to(point!(
        stud_distance / 2.,
        stud_distance / 2.,
        (block_height + stud_height) / 2.
    ))
    .circular_pattern(Axis::<3>::z(), 4);
let inner_block = Cuboid::from_dim(hollow_block_width, hollow_block_width, block_height)
    .move_to(point!(0.m(), 0.m(), thickness * -0.5));
let inner_tube = Cylinder::from_diameter(tube_diameter, block_height - thickness).subtract(
    &Cylinder::from_diameter(tube_diameter - thickness / 2., block_height - thickness),
);

let part = block.add(&studs).subtract(&inner_block).add(&inner_tube);
```
![](/examples/00_lego.png)

For more examples, have a look at the `/examples` directory.

# Inspiration

Anvil was originally inspired by the [opencascade](https://crates.io/crates/opencascade) crate. `opencascade-rs` was first used as the basis for another project, but we quickly realized a few drawbacks of the crate:
- basic functionality like `Clone` and `PartialEq` is missing
- hardly any function, struct, or enum are documented or tested
- the APIs are inconsistent and feel unintuitive

However, the OpenCascade bindings of the adjacent [opencascade-sys](https://crates.io/crates/opencascade-sys) crate were very useful and used as the backend of Anvil. In the future, we might try to eliminate OpenCascade entirely and implement a custom kernel in Rust.
