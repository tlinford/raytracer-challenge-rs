use std::{f64::consts::PI, path::Path};

use anyhow::Result;

use raytracer::{canvas::Canvas, color::Color, point::Point, ppm::save_ppm, transform::rotation_y};

fn main() -> Result<()> {
    println!("clock simulator 0.1!");

    let mut canvas = Canvas::new(1000, 1000);

    // twelve
    let start = Point::new(0, 0, 1);
    let radius = 1000.0 * 3.0 / 8.0;
    let green = Color::new(0.0, 1.0, 0.0);

    for i in 0..12 {
        let rotation = rotation_y(i as f64 * PI / 6.0);
        let hour = &rotation * start;
        println!("hour {} position: {:?}", i, hour);

        let x = hour.x * radius + 500.0;
        let y = hour.z * radius + 500.0;
        canvas.set_pixel(x.round() as usize, y.round() as usize, green);
    }

    save_ppm(&canvas, Path::new("renders/clock.ppm"))
}
