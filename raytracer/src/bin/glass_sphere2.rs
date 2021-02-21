use std::{
    f64::consts::{FRAC_PI_2, PI},
    path::Path,
};

use anyhow::Result;

use raytracer::{
    camera::Camera,
    color::Color,
    geometry::{
        shape::Plane,
        shape::{Csg, Operation, Sphere},
        Shape,
    },
    image::ppm::save_ppm,
    light::PointLight,
    pattern::checkers_pattern,
    point::Point,
    transform::{rotation_x, rotation_y, translation, view_transform},
    vector::Vector,
    world::World,
};

fn main() -> Result<()> {
    let mut world = World::new();

    let mut camera = Camera::new(2560, 2560, 0.45);
    camera.set_transform(view_transform(
        Point::new(0, 0, -7),
        Point::origin(),
        Vector::new(0, 1, 0),
    ));

    let light = PointLight::new(Point::new(2.0, 10.0, -5.0), Color::new(0.9, 0.9, 0.9));
    world.add_light(light);

    let mut wall = Plane::default();
    wall.set_transform(&translation(0, 0, 10) * &rotation_x(FRAC_PI_2));
    wall.get_base_mut().material.set_pattern(checkers_pattern(
        Color::new(0.15, 0.15, 0.15),
        Color::new(0.8, 0.8, 0.8),
    ));
    wall.get_base_mut().material.ambient = 0.8;
    wall.get_base_mut().material.diffuse = 0.2;
    wall.get_base_mut().material.specular = 0.0;
    world.add_object(wall);

    let mut ball1 = Sphere::default();
    ball1.get_base_mut().material.color = Color::white();
    ball1.get_base_mut().material.ambient = 0.0;
    ball1.get_base_mut().material.diffuse = 0.0;
    ball1.get_base_mut().material.specular = 0.9;
    ball1.get_base_mut().material.shininess = 300.0;
    ball1.get_base_mut().material.reflective = 0.9;
    ball1.get_base_mut().material.transparency = 0.9;
    ball1.get_base_mut().material.refractive_index = 1.5;

    ball1.set_transform(translation(0.25, 0.0, 0.0));

    let mut ball2 = Sphere::default();
    ball2.get_base_mut().material.color = Color::white();
    ball2.get_base_mut().material.ambient = 0.0;
    ball2.get_base_mut().material.diffuse = 0.0;
    ball2.get_base_mut().material.specular = 0.9;
    ball2.get_base_mut().material.shininess = 300.0;
    ball2.get_base_mut().material.reflective = 0.9;
    ball2.get_base_mut().material.transparency = 0.9;
    ball2.get_base_mut().material.refractive_index = 1.5;

    ball2.set_transform(translation(-0.25, 0.0, 0.0));

    let mut csg = Csg::new(Operation::Difference, ball1, ball2);
    csg.set_transform(rotation_y(-PI / 4.0));
    world.add_object(csg);

    // world.add_object(ball);

    // let mut center = Sphere::default();
    // center.set_transform(scaling(0.5, 0.5, 0.5));
    // center.get_base_mut().material.color = Color::white();
    // center.get_base_mut().material.ambient = 0.0;
    // center.get_base_mut().material.diffuse = 0.0;
    // center.get_base_mut().material.specular = 0.9;
    // center.get_base_mut().material.shininess = 300.0;
    // center.get_base_mut().material.reflective = 0.9;
    // center.get_base_mut().material.transparency = 0.9;
    // center.get_base_mut().material.refractive_index = 1.0000034;
    // world.add_object(center);

    let canvas = camera.render(&world);
    save_ppm(&canvas, Path::new("renders/glass_sphere2-difference.ppm"))
}
