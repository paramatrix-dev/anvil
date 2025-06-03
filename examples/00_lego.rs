use anvil::{Axis, Cuboid, Cylinder, IntoLength, Part, point};

fn construct() -> Part {
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

    block.add(&studs).subtract(&inner_block).add(&inner_tube)
}

fn main() {
    let part = construct();
    part.write_stl("examples/00_lego.stl")
        .expect("could not write part to .STL");
}
