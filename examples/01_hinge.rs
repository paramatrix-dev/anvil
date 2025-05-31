use anvil::{Axis3D, Cylinder, IntoAngle, IntoLength, Part, Path, Plane, point};

fn construct() -> Part {
    let thickness = 1.m();
    let depth = 7.m();
    let mount_distance = 7.m();
    let mount_radius = 0.5.m();
    let hinge_radius = 2.5.m();

    let hinge_outer_radius = hinge_radius + thickness;
    let total_height = hinge_outer_radius + 2. * thickness;

    let part = Path::at(point!(0, 0))
        .line_by(0.m(), thickness)
        .line_by(mount_distance - hinge_outer_radius, 0.m())
        .arc_by(thickness, 90.deg())
        .line_by(0.m(), hinge_outer_radius - 2. * thickness)
        .arc_by(-hinge_outer_radius, 180.deg())
        .line_by(0.m(), -hinge_outer_radius + 2. * thickness)
        .arc_by(thickness, 90.deg())
        .line_by(mount_distance - hinge_outer_radius, 0.m())
        .line_by(0.m(), -thickness)
        .close()
        .extrude(Plane::yz(), depth)
        .unwrap()
        .move_to(point!(thickness / 4., 0.m(), total_height / 2.));

    let hinge_hole = Cylinder::from_radius(hinge_radius, depth + thickness)
        .rotate_around(Axis3D::y(), 90.deg())
        .move_to(point!(0.m(), 0.m(), hinge_outer_radius));

    let mount_hole = Cylinder::from_radius(mount_radius, thickness * 2.);

    part.subtract(&hinge_hole)
        .subtract(&mount_hole.move_by(depth / 2. - thickness, mount_distance, thickness / 2.))
        .subtract(&mount_hole.move_by(depth / 2. - thickness, -mount_distance, thickness / 2.))
        .subtract(&mount_hole.move_by(-depth / 2. + thickness, mount_distance, thickness / 2.))
        .subtract(&mount_hole.move_by(-depth / 2. + thickness, -mount_distance, thickness / 2.))
}

fn main() {
    let part = construct();
    part.write_stl_with_tolerance("examples/01_hinge.stl", 0.01)
        .expect("could not write part to .STL");
}
