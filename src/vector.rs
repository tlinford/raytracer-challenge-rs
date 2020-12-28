use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        crate::equal(self.x, other.x)
            && crate::equal(self.y, other.y)
            && crate::equal(self.z, other.z)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, point: Point) -> Point {
        point + self
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, s: f64) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        vector * self
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, s: f64) -> Self {
        Self::new(self.x / s, self.y / s, self.z / s)
    }
}

pub fn dot(a: Vector, b: Vector) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross(a: Vector, b: Vector) -> Vector {
    Vector::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_vector() {
        let a = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
    }

    #[test]
    fn add_point_to_vector() {
        let v = Vector::new(-2.0, 3.0, 1.0);
        let p = Point::new(3.0, -2.0, 5.0);
        let expected = Point::new(1.0, 1.0, 6.0);
        assert_eq!(v + p, expected);
    }

    #[test]
    fn add_two_vectors() {
        let v1 = Vector::new(1.0, -2.0, 3.0);
        let v2 = Vector::new(-2.0, -3.0, 4.0);
        let expected = Vector::new(-1.0, -5.0, 7.0);
        assert_eq!(v1 + v2, expected);
        assert_eq!(v2 + v1, expected);
    }

    #[test]
    fn subtract_two_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        let expected = Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(v1 - v2, expected);
    }

    #[test]
    fn subtract_vector_from_zero_vector() {
        let zero = Vector::new(0.0, 0.0, 0.0);
        let v = Vector::new(1.0, -2.0, 3.0);
        let expected = Vector::new(-1.0, 2.0, -3.0);
        assert_eq!(zero - v, expected);
    }

    #[test]
    fn negate_vector() {
        let a = Vector::new(1.0, -2.0, 3.0);
        let expected = Vector::new(-1.0, 2.0, -3.0);
        assert_eq!(-a, expected);
    }

    #[test]
    fn multiply_vector_by_scalar() {
        let a = Vector::new(1.0, -2.0, 3.0);
        let expected = Vector::new(3.5, -7.0, 10.5);
        assert_eq!(a * 3.5, expected);
        assert_eq!(3.5 * a, expected);
    }

    #[test]
    fn multiply_vector_by_fraction() {
        let a = Vector::new(1.0, -2.0, 3.0);
        let expected = Vector::new(0.5, -1.0, 1.5);
        assert_eq!(a * 0.5, expected);
        assert_eq!(0.5 * a, expected);
    }

    #[test]
    fn divide_vector_by_scalar() {
        let a = Vector::new(1.0, -2.0, 3.0);
        let expected = Vector::new(0.5, -1.0, 1.5);
        assert_eq!(a / 2.0, expected);
    }

    #[test]
    fn vector_magnitude() {
        let table = vec![
            (Vector::new(1.0, 0.0, 0.0), 1.0),
            (Vector::new(0.0, 1.0, 0.0), 1.0),
            (Vector::new(0.0, 0.0, 1.0), 1.0),
            (Vector::new(1.0, 2.0, 3.0), (14.0f64).sqrt()),
            (Vector::new(-1.0, -2.0, -3.0), (14.0f64).sqrt()),
        ];

        for (vector, result) in table {
            assert_eq!(vector.magnitude(), result);
        }
    }

    #[test]
    fn vector_normalize() {
        let table = vec![
            (Vector::new(4.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0)),
            (
                Vector::new(1.0, 2.0, 3.0),
                Vector::new(0.26726, 0.53452, 0.80178),
            ),
        ];

        for (vector, result) in table {
            assert_eq!(vector.normalize(), result);
            assert_eq!(vector.normalize().magnitude(), 1.);
        }
    }

    #[test]
    fn vector_dot_product() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        let expected = 20.0;
        assert_eq!(dot(a, b), expected);
        assert_eq!(dot(b, a), expected);
    }

    #[test]
    fn vector_cross_product() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        let expected1 = Vector::new(-1.0, 2.0, -1.0);
        let expected2 = Vector::new(1.0, -2.0, 1.0);
        assert_eq!(cross(a, b), expected1);
        assert_eq!(cross(b, a), expected2);
    }
}
