use anvil::{Axis3D, Cylinder, Part, Path, Plane, Point3D, angle, length, point};

fn construct() -> Part {
    let thickness = length!(1 m);
    let depth = length!(7 m);
    let mount_distance = length!(7 m);
    let mount_radius = length!(0.5 m);
    let hinge_radius = length!(2.5 m);

    let hinge_outer_radius = hinge_radius + thickness;
    let total_height = hinge_outer_radius + 2. * thickness;

    let part = Path::at(point!(0 m, 0 m))
        .line_by(length!(0), thickness)
        .line_by(mount_distance - hinge_outer_radius, length!(0))
        .arc_by(thickness, angle!(90 deg))
        .line_by(length!(0), hinge_outer_radius - 2. * thickness)
        .arc_by(-hinge_outer_radius, angle!(180 deg))
        .line_by(length!(0), -hinge_outer_radius + 2. * thickness)
        .arc_by(thickness, angle!(90 deg))
        .line_by(mount_distance - hinge_outer_radius, length!(0))
        .line_by(length!(0), -thickness)
        .close()
        .extrude(Plane::yz(), depth)
        .unwrap()
        .move_to(Point3D::new(thickness / 4., length!(0), total_height / 2.));

    let hinge_hole = Cylinder::from_radius(hinge_radius, depth + thickness)
        .rotate_around(Axis3D::y(), angle!(90 deg))
        .move_to(point!(0 m, 0 m, hinge_outer_radius));

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
