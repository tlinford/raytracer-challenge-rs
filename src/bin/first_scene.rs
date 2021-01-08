use std::{error::Error, f64::consts::PI, path::Path};

use raytracer::{
    camera::Camera,
    color::Color,
    geometry::shape::{glass_sphere, plane, sphere},
    light::PointLight,
    material::Material,
    matrix::Matrix,
    pattern::{checkers_pattern, ring_pattern, stripe_pattern},
    point::Point,
    ppm::save_ppm,
    transform::{scaling, translation, view_transform},
    vector::Vector,
    world::World,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("simple scene 1.0!");

    let mut floor = plane();
    floor.material.set_pattern(checkers_pattern(
        Color::new(0.0, 0.5, 0.5),
        Color::new(0.5, 0.0, 0.5),
    ));
    let mut left_wall = plane();

    let left_wall_transform = Matrix::identity(4, 4)
        .rotate_x(PI / 2.0)
        .rotate_y(-PI / 4.0)
        .translate(0, 0, 5);

    left_wall.set_transform(&left_wall_transform);
    let mut left_wall_pattern = ring_pattern(Color::new(0.0, 0.0, 1.0), Color::new(0.0, 1.0, 1.0));
    left_wall_pattern.set_transform(scaling(0.333, 0.333, 0.333));
    left_wall.material.set_pattern(left_wall_pattern);

    let mut right_wall = plane();

    let right_wall_transform = Matrix::identity(4, 4)
        .rotate_x(PI / 2.0)
        .rotate_y(PI / 4.0)
        .translate(0, 0, 5);

    right_wall.set_transform(&right_wall_transform);
    right_wall.material = Material::default();
    right_wall
        .material
        .set_pattern(stripe_pattern(Color::white(), Color::black()));

    let mut middle = sphere();
    middle.set_transform(&translation(-0.5, 1.0, 0.5));
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle.material.reflective = 0.9;

    let mut right = sphere();
    right.set_transform(&(&translation(1.5, 0.5, -0.5) * &scaling(0.5, 0.5, 0.5)));
    right.material.set_pattern(checkers_pattern(
        Color::new(1.0, 0.0, 0.0),
        Color::new(0.0, 1.0, 0.0),
    ));

    let mut left = glass_sphere();
    left.material.color = Color::new(0.1, 0.0, 0.0);
    left.material.ambient = 0.1;
    left.material.diffuse = 0.05;
    left.material.reflective = 0.3;
    left.material.specular = 1.0;
    left.material.shininess = 300.0;
    left.set_transform(&(&translation(-1.5, 0.33, -0.75) * &scaling(0.33, 0.33, 0.33)));

    let mut world = World::new();
    let light_source1 = PointLight::new(Point::new(-10, 10, -10), Color::new(1.0, 1.0, 1.0));
    let light_source2 = PointLight::new(Point::new(-5.0, 10.0, -6.0), Color::new(0.33, 0.33, 0.33));
    world.add_light(light_source1);
    world.add_light(light_source2);

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
