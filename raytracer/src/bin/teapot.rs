use std::{f64::consts::PI, path::Path, sync::Arc};

use anyhow::Result;

use obj_parser::parse_obj_file;
use raytracer::{
    camera::{self, Camera},
    color::Color,
    geometry::{shape::Plane, Shape},
    image::ExportCanvas,
    light::PointLight,
    material::Material,
    matrix::Matrix,
    obj_parser,
    pattern::checkers_pattern,
    point::Point,
    transform::view_transform,
    vector::Vector,
    world::World,
};

fn main() -> Result<()> {
    println!("hexagon scene");

    let mut world = World::new();
    let light_source1 = PointLight::new(Point::new(-4, 4, -5), Color::new(1.0, 1.0, 1.0));
    let light_source2 = PointLight::new(Point::new(-3.0, 5.0, -3.0), Color::new(0.33, 0.33, 0.33));
    world.add_light(light_source1);
    world.add_light(light_source2);

    let mut floor = Plane::default();
    let mut floor_material = Material::default();
    floor_material.set_pattern(checkers_pattern(
        Color::new(0.8, 0.8, 0.8),
        Color::new(0.6, 0.6, 0.6),
    ));
    floor_material.specular = 0.8;
    floor_material.reflective = 0.4;
    floor.set_material(floor_material);
    world.add_object(floor);

    let mut parser1 = parse_obj_file(Path::new("raytracer/models/teapot-low.obj")).unwrap();
    let mut teapot_smooth = parser1.as_group();
    println!(
        "smooth teapot bounds before transform: {:?}",
        teapot_smooth.get_bounds()
    );
    teapot_smooth.set_transform(
        Matrix::identity(4, 4)
            .scale(0.12, 0.12, 0.12)
            .rotate_x(-PI / 2.0)
            .rotate_y(PI / 7.0)
            .translate(1.5, 0.0, 0.0),
    );

    println!(
        "smooth teapot bounds after transform : {:?}",
        teapot_smooth.get_bounds()
    );

    let mut material = Material::default();
    material.color = Color::new(0.6, 0.4, 0.2);
    material.ambient = 0.35;
    material.diffuse = 0.3;
    material.specular = 0.2;
    material.reflective = 0.1;

    teapot_smooth.set_material(material.clone());

    let mut parser2 = parse_obj_file(Path::new("raytracer/models/teapot_hr.obj")).unwrap();
    let mut teapot = parser2.as_group();
    println!(
        "teapot bounds before transform:        {:?}",
        teapot.get_bounds()
    );
    teapot.set_material(material);
    teapot.set_transform(
        Matrix::identity(4, 4)
            .scale(0.6, 0.6, 0.6)
            .rotate_y(PI / 5.0)
            .translate(-1.9, 0.0, 0.0),
    );
    println!(
        "teapot bounds after transform:        {:?}",
        teapot.get_bounds()
    );

    teapot.divide(1000);

    world.add_object(teapot_smooth);
    world.add_object(teapot);

    let mut camera = Camera::new(2560, 1440, PI / 3.0);
    camera.set_transform(view_transform(
        Point::new(0.0, 2.5, -7.0),
        Point::new(0.0, 1.25, 0.0),
        Vector::new(0, 1, 0),
    ));

    // let canvas = camera.render(&world);
    let canvas = camera::Camera::render_multithreaded(Arc::new(camera), Arc::new(world), 16);
    let exporter = raytracer::image::png::PngExporter {};
    exporter.save(
        &canvas,
        Path::new("raytracer/renders/teapot_multithreaded_debug.png"),
    )
}
