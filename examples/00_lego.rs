use anvil::{Axis3D, Cuboid, Cylinder, Part, Point3D, length};

fn construct() -> Part {
    let block_width = length!(16 mm);
    let block_height = length!(9.6 mm);
    let stud_height = length!(11.2 mm) - block_height;
    let stud_distance = length!(8 mm);
    let stud_diameter = length!(4.8 mm);
    let thickness = length!(1.2 mm);
    let tube_diameter = length!(6.5 mm);

    let block = Cuboid::from_dim(block_width, block_width, block_height);
    let studs = Cylinder::from_diameter(stud_diameter, stud_height)
        .move_to(Point3D::new(
            stud_distance / 2.,
            stud_distance / 2.,
            (block_height + stud_height) / 2.,
        ))
        .circular_pattern(Axis3D::z(), 4);
    let inner_block = Cuboid::from_dim(
        block_width - thickness,
        block_width - thickness,
        block_height,
    )
    .move_to(Point3D::new(length!(0), length!(0), thickness * -0.5));
    let inner_tube = Cylinder::from_diameter(tube_diameter, block_height - thickness).subtract(
        &Cylinder::from_diameter(tube_diameter - thickness / 2., block_height - thickness),
    );

    block.add(&studs).subtract(&inner_block).add(&inner_tube)
}

fn main() {
    let part = construct();
    part.write_stl("examples/00_lego.stl")
        .expect("could not write part to .STL");
    part.write_step("examples/00_lego.step")
        .expect("could not write part to .STEP");
}
