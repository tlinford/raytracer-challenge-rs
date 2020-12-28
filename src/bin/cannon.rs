use raytracer::{point::Point, vector::Vector};

fn main() {
    let mut p = Projectile::new(
        Point::new(0., 1., 0.0),
        Vector::new(1., 1., 0.).normalize() * 2.0f64,
    );
    let e = Environment::new(Vector::new(0., -0.1, 0.), Vector::new(-0.01, 0., 0.));

    println!("Launching projectile {:?}\nwith environment: {:?}\n", p, e);

    let mut tick_count = 1;

    while p.position.y >= 0.0 {
        p = tick(&e, &p);
        println!("projectile after {} ticks: {:?}", tick_count, p);
        tick_count += 1;
    }
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
