use std::{f64::consts::PI, path::Path};

use anyhow::Result;

use raytracer::{
    camera::Camera,
    color::Color,
    geometry::{
        shape::{Cylinder, Group, Sphere},
        Shape,
    },
    image::ppm::save_ppm,
    light::PointLight,
    material::Material,
    matrix::Matrix,
    pattern::checkers_pattern,
    point::Point,
    transform::{rotation_y, scaling, translation, view_transform},
    vector::Vector,
    world::World,
};

fn main() -> Result<()> {
    println!("hexagon scene");

    let mut world = World::new();
    let light_source1 = PointLight::new(Point::new(-10, 10, -10), Color::new(1.0, 1.0, 1.0));
    let light_source2 = PointLight::new(Point::new(-5.0, 10.0, -6.0), Color::new(0.33, 0.33, 0.33));
    world.add_light(light_source1);
    world.add_light(light_source2);

    world.add_object(hexagon());

    let mut camera = Camera::new(2560, 1440, PI / 3.0);
    camera.set_transform(view_transform(
        Point::new(0.0, 2.5, -5.0),
        Point::new(0, 0, 0),
        Vector::new(0, 1, 0),
    ));

    let canvas = camera.render(&world);
    save_ppm(&canvas, Path::new("renders/hexagon.ppm"))
}

fn hexagon_corner() -> Box<dyn Shape> {
    let mut corner = Sphere::default();
    corner.set_transform(&translation(0, 0, -1) * &scaling(0.25, 0.25, 0.25));
    Box::new(corner)
}

fn hexagon_edge() -> Box<dyn Shape> {
    let mut edge = Cylinder::new(0, 1, false);
    edge.set_transform(
        Matrix::identity(4, 4)
            .scale(0.25, 1.0, 0.25)
            .rotate_z(-PI / 2.0)
            .rotate_y(-PI / 6.0)
            .translate(0, 0, -1),
    );
    Box::new(edge)
}

fn hexagon_side() -> Box<dyn Shape> {
    let mut side = Group::default();
    side.add_child(hexagon_corner());
    side.add_child(hexagon_edge());

    Box::new(side)
}

fn hexagon() -> Group {
    let mut hex = Group::default();
    for n in 0..=5 {
        let mut side = hexagon_side();
        side.set_transform(rotation_y(n as f64 * PI / 3.0));
        hex.add_child(side)
    }

    let mut material = Material::default();
    material.set_pattern(checkers_pattern(
        Color::new(1.0, 0.0, 0.0),
        Color::new(0.0, 1.0, 0.0),
    ));

    hex.set_material(material);
    hex.set_transform(scaling(1.5, 1.5, 1.5));
    hex
}
