pub mod canvas;
pub mod color;
pub mod point;
pub mod vector;

const EPSILON: f64 = 0.00001;

fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
