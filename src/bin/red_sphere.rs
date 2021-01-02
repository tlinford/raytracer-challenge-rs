use std::{error::Error, f64::consts::PI, path::Path};

use raytracer::{
    canvas::Canvas,
    color::Color,
    geometry::{intersection::hit, sphere::Sphere},
    matrix::Matrix,
    point::Point,
    ppm::save_ppm,
    ray::Ray,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("simple sphere render 1.0");

    let ray_origin = Point::new(0, 0, -5);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 1000;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);
    let mut shape = Sphere::default();
    shape.set_transform(
        &Matrix::identity(4, 4)
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

            if hit(&xs).is_some() {
                canvas.set_pixel(x, y, color);
            }
        }
    }

    save_ppm(&canvas, Path::new("renders/red_sphere.ppm"))
}
