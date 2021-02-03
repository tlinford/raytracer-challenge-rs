use std::ops::{Add, Sub};

use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new<T: Into<f64> + Copy>(x: T, y: T, z: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn origin() -> Self {
        Self::new(0, 0, 0)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        crate::equal(self.x, other.x)
            && crate::equal(self.y, other.y)
            && crate::equal(self.z, other.z)
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, vector: Vector) -> Self {
        Self::new(self.x + vector.x, self.y + vector.y, self.z + vector.z)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, vector: Vector) -> Self {
        Self::new(self.x - vector.x, self.y - vector.y, self.z - vector.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_point() {
        let a = Point::new(4.3, -4.2, 3.1);
        assert!(crate::equal(a.x, 4.3));
        assert!(crate::equal(a.y, -4.2));
        assert!(crate::equal(a.z, 3.1));
    }

    #[test]
    fn add_vector_to_point() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        let expected = Point::new(1.0, 1.0, 6.0);
        assert_eq!(p + v, expected);
    }

    #[test]
    fn subract_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        let expected = Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(p1 - p2, expected);
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        let expected = Point::new(-2.0, -4.0, -6.0);
        assert_eq!(p - v, expected);
    }
}
