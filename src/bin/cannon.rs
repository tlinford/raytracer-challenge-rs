use raytracer_challenge_rs::tuple::Tuple;

fn main() {
    let mut p = Projectile::new(
        Tuple::point(0., 1., 0.0),
        Tuple::vector(1., 1., 0.).normalize() * 2.0f64,
    );
    let e = Environment::new(Tuple::vector(0., -0.1, 0.), Tuple::vector(-0.01, 0., 0.));

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
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    fn new(position: Tuple, velocity: Tuple) -> Self {
        Self { position, velocity }
    }
}

#[derive(Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Environment {
    fn new(gravity: Tuple, wind: Tuple) -> Self {
        Self { gravity, wind }
    }
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile::new(position, velocity)
}
