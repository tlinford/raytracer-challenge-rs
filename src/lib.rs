pub mod camera;
pub mod canvas;
pub mod color;
pub mod geometry;
pub mod light;
pub mod material;
pub mod matrix;
pub mod pattern;
pub mod point;
pub mod ppm;
pub mod ray;
pub mod transform;
pub mod vector;
pub mod world;

const EPSILON: f64 = 0.00001;

fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
