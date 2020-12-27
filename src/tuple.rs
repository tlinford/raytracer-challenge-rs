use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1. }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0. }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        crate::equal(self.x, other.x)
            && crate::equal(self.y, other.y)
            && crate::equal(self.z, other.z)
            && self.w == other.w
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn is_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn is_tuple() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_create() {
        let p = Tuple::point(4.0, -4.0, 3.0);
        let a = Tuple::new(4.0, -4.0, 3.0, 1.0);
        assert_eq!(p, a);
    }

    #[test]
    fn vector_create() {
        let v = Tuple::vector(4., -4., 3.);
        let a = Tuple::new(4., -4., 3., 0.);
        assert_eq!(v, a);
    }

    #[test]
    fn add_tuples() {
        let a1 = Tuple::new(3., -2., 5., 1.);
        let a2 = Tuple::new(-2., 3., 1., 0.);
        let result = Tuple::new(1., 1., 6., 1.);
        assert_eq!(a1 + a2, result);
    }

    #[test]
    fn subtract_points() {
        let p1 = Tuple::point(3., 2., 1.);
        let p2 = Tuple::point(5., 6., 7.);
        let result = Tuple::vector(-2., -4., -6.);
        assert_eq!(p1 - p2, result);
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = Tuple::point(3., 2., 1.);
        let v = Tuple::vector(5., 6., 7.);
        let result = Tuple::point(-2., -4., -6.);
        assert_eq!(p - v, result);
    }

    #[test]
    fn subtract_vectors() {
        let v1 = Tuple::vector(3., 2., 1.);
        let v2 = Tuple::vector(5., 6., 7.);
        let result = Tuple::vector(-2., -4., -6.);
        assert_eq!(v1 - v2, result);
    }

    #[test]
    fn subtract_vector_from_zero_vector() {
        let zero = Tuple::vector(0., 0., 0.);
        let v = Tuple::vector(1., -2., 3.);
        let result = Tuple::vector(-1., 2., -3.);
        assert_eq!(zero - v, result);
    }

    #[test]
    fn negate_tuple() {
        let a = Tuple::new(1., -2., 3., -4.);
        let result = Tuple::new(-1., 2., -3., 4.);
        assert_eq!(-a, result);
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let a = Tuple::new(1., -2., 3., -4.);
        let result = Tuple::new(3.5, -7., 10.5, -14.);
        assert_eq!(a * 3.5, result);
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let a = Tuple::new(1., -2., 3., -4.);
        let result = Tuple::new(0.5, -1., 1.5, -2.);
        assert_eq!(a * 0.5, result);
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let a = Tuple::new(1., -2., 3., -4.);
        let result = Tuple::new(0.5, -1., 1.5, -2.);
        assert_eq!(a / 2., result);
    }

    #[test]
    fn magnitude() {
        let table = vec![
            (Tuple::vector(1., 0., 0.), 1.),
            (Tuple::vector(0., 1., 0.), 1.),
            (Tuple::vector(0., 0., 1.), 1.),
            (Tuple::vector(1., 2., 3.), (14.0f64).sqrt()),
            (Tuple::vector(-1., -2., -3.), (14.0f64).sqrt()),
        ];

        for (tuple, result) in table {
            assert_eq!(tuple.magnitude(), result);
        }
    }

    #[test]
    fn normalize() {
        let table = vec![
            (Tuple::vector(4., 0., 0.), Tuple::vector(1., 0., 0.)),
            (
                Tuple::vector(1., 2., 3.),
                Tuple::vector(0.26726, 0.53452, 0.80178),
            ),
        ];

        for (tuple, result) in table {
            assert_eq!(tuple.normalize(), result);
            assert_eq!(tuple.normalize().magnitude(), 1.);
        }
    }

    #[test]
    fn dot_product() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2., 3., 4.);
        let result = 20.;
        assert_eq!(a.dot(&b), result);
    }

    #[test]
    fn cross() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2., 3., 4.);
        let result1 = Tuple::vector(-1., 2., -1.);
        assert_eq!(a.cross(&b), result1);
        let result2 = Tuple::vector(1., -2., 1.);
        assert_eq!(b.cross(&a), result2);
    }
}
