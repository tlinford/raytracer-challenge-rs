use std::{f64::consts::PI, path::Path};

use anyhow::Result;

use raytracer::{
    canvas::Canvas,
    color::Color,
    geometry::{
        intersection::{hit, Intersection},
        shape::Sphere,
        Shape,
    },
    image::ppm::save_ppm,
    light::PointLight,
    material::Material,
    matrix::Matrix,
    point::Point,
    ray::Ray,
};

fn main() -> Result<()> {
    println!("simple sphere render 2.0");

    let ray_origin = Point::new(0, 0, -5);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 1000;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    // let color = Color::new(1.0, 0.0, 0.0);
    let mut shape = Sphere::default();
    let mut material = Material::default();
    material.color = Color::new(1.0, 0.2, 1.0);
    shape.set_material(material);

    let light_position = Point::new(-10, 10, -10);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    shape.set_transform(
        Matrix::identity(4, 4)
            .scale(0.5, 1.0, 1.0)
            .rotate_z(PI / 4.0),
    );

    for y in 0..canvas_pixels {
        if y % 10 == 0 {
            println!("rendering row {}", y);
        }
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(&r);

            if let Some(hit) = hit(&xs) {
                let point = r.position(hit.t());
                let normal = hit
                    .object()
                    .normal_at(point, &Intersection::new(-100.0, &shape));
                let eye = -(r.direction());
                let color = hit
                    .object()
                    .material()
                    .lighting(&shape, &light, &point, &eye, &normal, false);
                canvas.set_pixel(x, y, color);
            }
        }
    }

    save_ppm(&canvas, Path::new("renders/sphere_with_light.ppm"))
}
