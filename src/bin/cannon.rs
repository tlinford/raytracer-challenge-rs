use std::{error::Error, path::Path};

use raytracer::{canvas::Canvas, color::Color, point::Point, ppm::save_ppm, vector::Vector};

fn main() -> Result<(), Box<dyn Error>> {
    let mut p = Projectile::new(
        Point::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    );
    let env = Environment::new(Vector::new(0., -0.1, 0.), Vector::new(-0.01, 0., 0.));
    let mut canvas = Canvas::new(900, 550);

    println!(
        "Launching projectile {:?}\nwith environment: {:?}\n",
        p, env
    );

    let mut tick_count = 1;

    let red = Color::new(1.0, 0.0, 0.0);

    while p.position.y >= 0.0 {
        println!("projectile after {} ticks: {:?}", tick_count, p);
        let x = p.position.x.round() as usize;
        let y = canvas.height() - p.position.y.round() as usize;

        println!("writing pixel ({}, {})", x, y);

        canvas.set_pixel(x, y, red);
        p = tick(&env, &p);

        tick_count += 1;
    }

    save_ppm(&canvas, Path::new("renders/cannon.ppm"))
}

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity }
    }
}

#[derive(Debug)]
struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    fn new(gravity: Vector, wind: Vector) -> Self {
        Self { gravity, wind }
    }
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile::new(position, velocity)
}
