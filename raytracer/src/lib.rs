pub mod bounding_box;
pub mod camera;
pub mod canvas;
pub mod color;
pub mod geometry;
pub mod image;
pub mod light;
pub mod material;
pub mod matrix;
pub mod obj_parser;
pub mod pattern;
pub mod point;
pub mod ray;
pub mod transform;
pub mod vector;
pub mod world;

const EPSILON: f64 = 0.00001;

fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

fn equal_ignore_inf(a: f64, b: f64) -> bool {
    // TODO: WORKAROUND TO MAKE EQUALITY WORK WHEN BOUNDING BOXES HAVE -INF OR +INF COORDS
    if a.is_infinite() && b.is_infinite() {
        return (a.is_sign_positive() && a.is_sign_positive())
            || (b.is_sign_negative() && b.is_sign_negative());
    }
    equal(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn infiny_is_equal() {
        assert!(!equal(f64::INFINITY, f64::INFINITY));
        assert!(equal_ignore_inf(f64::INFINITY, f64::INFINITY));
    }
}
