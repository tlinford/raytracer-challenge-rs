use std::{error::Error, f64::consts::PI, path::Path};

use raytracer::{
    camera::Camera,
    color::Color,
    geometry::shape::{plane, sphere},
    light::PointLight,
    matrix::Matrix,
    point::Point,
    ppm::save_ppm,
    transform::{scaling, translation, view_transform},
    vector::Vector,
    world::World,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("simple scene 1.0!");

    let mut floor = plane();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = plane();

    let left_wall_transform = Matrix::identity(4, 4)
        .rotate_x(PI / 2.0)
        .rotate_y(-PI / 4.0)
        .translate(0, 0, 5);

    left_wall.set_transform(&left_wall_transform);
    left_wall.material = floor.material;

    let mut right_wall = plane();

    let right_wall_transform = Matrix::identity(4, 4)
        .rotate_x(PI / 2.0)
        .rotate_y(PI / 4.0)
        .translate(0, 0, 5);

    right_wall.set_transform(&right_wall_transform);
    right_wall.material = floor.material;

    let mut middle = sphere();
    middle.set_transform(&translation(-0.5, 1.0, 0.5));
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = sphere();
    right.set_transform(&(&translation(1.5, 0.5, -0.5) * &scaling(0.5, 0.5, 0.5)));
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = sphere();
    left.set_transform(&(&translation(-1.5, 0.33, -0.75) * &scaling(0.33, 0.33, 0.33)));
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::new();
    let light_source = PointLight::new(Point::new(-10, 10, -10), Color::new(1.0, 1.0, 1.0));
    world.add_light(light_source);

    world.add_object(floor);
    world.add_object(left_wall);
    world.add_object(right_wall);
    world.add_object(middle);
    world.add_object(right);
    world.add_object(left);

    let mut camera = Camera::new(2560, 1440, PI / 3.0);
    camera.set_transform(view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0, 1, 0),
        Vector::new(0, 1, 0),
    ));

    let canvas = camera.render(&world);
    save_ppm(&canvas, Path::new("renders/first_scene.ppm"))
}
