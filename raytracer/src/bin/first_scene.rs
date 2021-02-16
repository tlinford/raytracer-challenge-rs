use std::{f64::consts::PI, path::Path};

use anyhow::Result;

use raytracer::{
    camera::Camera,
    color::Color,
    geometry::{shape::Cone, shape::Cube, shape::Cylinder, shape::Plane, shape::Sphere, Shape},
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

fn main() -> Result<()> {
    println!("simple scene 1.0!");

    let mut floor = Plane::default();
    floor.get_base_mut().material.set_pattern(checkers_pattern(
        Color::new(0.0, 0.5, 0.5),
        Color::new(0.5, 0.0, 0.5),
    ));
    let mut left_wall = Plane::default();

    let left_wall_transform = Matrix::identity(4, 4)
        .rotate_x(PI / 2.0)
        .rotate_y(-PI / 4.0)
        .translate(0, 0, 5);

    left_wall.set_transform(left_wall_transform);
    let mut left_wall_pattern = ring_pattern(Color::new(0.0, 0.0, 1.0), Color::new(0.0, 1.0, 1.0));
    left_wall_pattern.set_transform(scaling(0.333, 0.333, 0.333));
    left_wall
        .get_base_mut()
        .material
        .set_pattern(left_wall_pattern);

    let mut right_wall = Plane::default();

    let right_wall_transform = Matrix::identity(4, 4)
        .rotate_x(PI / 2.0)
        .rotate_y(PI / 4.0)
        .translate(0, 0, 5);

    right_wall.set_transform(right_wall_transform);
    right_wall.get_base_mut().material = Material::default();
    right_wall
        .get_base_mut()
        .material
        .set_pattern(stripe_pattern(Color::white(), Color::black()));

    let mut middle = Sphere::default();
    middle.set_transform(translation(-0.5, 1.0, 0.5));
    middle.get_base_mut().material.color = Color::new(0.1, 1.0, 0.5);
    middle.get_base_mut().material.diffuse = 0.7;
    middle.get_base_mut().material.specular = 0.3;
    middle.get_base_mut().material.reflective = 0.9;

    let mut right = Sphere::default();
    right.set_transform(&translation(1.5, 0.5, -0.5) * &scaling(0.5, 0.5, 0.5));
    right.get_base_mut().material.set_pattern(checkers_pattern(
        Color::new(1.0, 0.0, 0.0),
        Color::new(0.0, 1.0, 0.0),
    ));

    let mut left = Sphere::glass();
    left.get_base_mut().material.color = Color::new(0.1, 0.0, 0.0);
    left.get_base_mut().material.ambient = 0.1;
    left.get_base_mut().material.diffuse = 0.05;
    left.get_base_mut().material.reflective = 0.3;
    left.get_base_mut().material.specular = 1.0;
    left.get_base_mut().material.shininess = 300.0;
    left.set_transform(&translation(-1.5, 0.33, -0.75) * &scaling(0.33, 0.33, 0.33));

    let mut cube = Cube::default();
    cube.set_transform(
        Matrix::identity(4, 4)
            .rotate_y(PI / 4.0)
            .scale(0.25, 0.25, 0.25)
            .translate(0.0, 0.25, -1.0),
    );

    let mut cylinder = Cylinder::new(0.0, 1.0, true);
    cylinder.set_transform(&translation(1.0, 0.0, -1.2) * &scaling(0.33, 0.33, 0.33));

    let mut cone = Cone::new(-1.0, 0.0, true);
    cone.set_transform(&translation(-1.0, 0.33, -1.2) * &scaling(0.33, 0.33, 0.33));

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
    world.add_object(cube);
    world.add_object(cylinder);
    world.add_object(cone);

    let mut camera = Camera::new(2560, 1440, PI / 3.0);
    camera.set_transform(view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0, 1, 0),
        Vector::new(0, 1, 0),
    ));

    let canvas = camera.render(&world);
    save_ppm(&canvas, Path::new("renders/first_scene.ppm"))
}
