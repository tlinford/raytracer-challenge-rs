use crate::{point::Point, ray::Ray, vector::Vector, EPSILON};

#[derive(Debug, PartialEq)]
pub struct Cylinder {
    minimum: f64,
    maximum: f64,
    closed: bool,
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            closed: false,
        }
    }
}

impl Cylinder {
    pub fn new<T: Into<f64> + Copy>(minimum: T, maximum: T, closed: bool) -> Self {
        Self {
            minimum: minimum.into(),
            maximum: maximum.into(),
            closed,
        }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        let a = ray.direction().x.powi(2) + ray.direction().z.powi(2);
        if a.abs() < EPSILON {
            return self.intersect_caps(ray);
        }

        let b = 2.0 * ray.origin().x * ray.direction().x + 2.0 * ray.origin().z * ray.direction().z;
        let c = ray.origin().x.powi(2) + ray.origin().z.powi(2) - 1.0;

        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            return vec![];
        }

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);

        let mut xs = vec![];
        let y0 = ray.origin().y + t0 * ray.direction().y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(t0);
        }

        let y1 = ray.origin().y + t1 * ray.direction().y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(t1);
        }

        xs.append(&mut self.intersect_caps(ray));

        xs
    }

    pub fn local_normal_at(&self, point: Point) -> Vector {
        let dist = point.x * point.x + point.z * point.z;
        if dist < 1.0 && point.y >= self.maximum - EPSILON {
            Vector::new(0, 1, 0)
        } else if dist < 1.0 && point.y <= self.minimum + EPSILON {
            Vector::new(0, -1, 0)
        } else {
            Vector::new(point.x, 0.0, point.z)
        }
    }

    fn check_cap(&self, ray: &Ray, t: f64) -> bool {
        let x = ray.origin().x + t * ray.direction().x;
        let z = ray.origin().z + t * ray.direction().z;
        (x * x + z * z) <= 1.0
    }

    fn intersect_caps(&self, ray: &Ray) -> Vec<f64> {
        let mut xs = vec![];
        if !self.closed {
            return xs;
        }

        let t = (self.minimum - ray.origin().y) / ray.direction().y;
        if self.check_cap(ray, t) {
            xs.push(t);
        }

        let t = (self.maximum - ray.origin().y) / ray.direction().y;
        if self.check_cap(ray, t) {
            xs.push(t);
        }

        xs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{equal, point::Point, vector::Vector};

    #[test]
    fn ray_misses_cylinder() {
        let cyl = Cylinder::default();

        let test_cases = vec![
            (Point::new(1, 0, 0), Vector::new(0, 1, 0)),
            (Point::origin(), Vector::new(0, 1, 0)),
            (Point::new(0, 0, -5), Vector::new(1, 1, 1)),
        ];

        for test in test_cases {
            let r = Ray::new(test.0, test.1);
            let xs = cyl.local_intersect(&r);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn ray_strikes_cylinder() {
        let cyl = Cylinder::default();
        struct TestCase {
            origin: Point,
            direction: Vector,
            t1: f64,
            t2: f64,
        }

        impl TestCase {
            fn new(origin: Point, direction: Vector, t1: f64, t2: f64) -> Self {
                Self {
                    origin,
                    direction,
                    t1,
                    t2,
                }
            }
        }

        let test_cases = vec![
            TestCase::new(Point::new(1, 0, -5), Vector::new(0, 0, 1), 5.0, 5.0),
            TestCase::new(Point::new(0, 0, -5), Vector::new(0, 0, 1), 4.0, 6.0),
            TestCase::new(
                Point::new(0.5, 0.0, -5.0),
                Vector::new(0.1, 1.0, 1.0),
                6.80798,
                7.08872,
            ),
        ];

        for test in test_cases {
            let direction = test.direction.normalize();
            let r = Ray::new(test.origin, direction);
            let xs = cyl.local_intersect(&r);
            assert_eq!(xs.len(), 2);
            assert!(equal(xs[0], test.t1));
            assert!(equal(xs[1], test.t2));
        }
    }

    #[test]
    fn normal_vector_on_cylinder() {
        let cyl = Cylinder::default();

        let test_cases = vec![
            (Point::new(1, 0, 0), Vector::new(1, 0, 0)),
            (Point::new(0, 5, -1), Vector::new(0, 0, -1)),
            (Point::new(0, -2, 1), Vector::new(0, 0, 1)),
            (Point::new(-1, 1, 0), Vector::new(-1, 0, 0)),
        ];

        for test in test_cases {
            let n = cyl.local_normal_at(test.0);
            assert_eq!(n, test.1);
        }
    }

    #[test]
    fn default_minimum_and_maximum_for_cylinder() {
        let cyl = Cylinder::default();
        assert!(cyl.minimum.is_infinite() && cyl.minimum.is_sign_negative());
        assert!(cyl.maximum.is_infinite() && cyl.maximum.is_sign_positive());
    }

    #[test]
    fn intersect_constrained_cylinder() {
        let cyl = Cylinder::new(1.0, 2.0, false);

        let test_cases = vec![
            (Point::new(0.0, 1.5, 0.0), Vector::new(0.1, 1.0, 0.0), 0),
            (Point::new(0, 3, -5), Vector::new(0, 0, 1), 0),
            (Point::new(0, 0, -5), Vector::new(0, 0, 1), 0),
            (Point::new(0, 2, -5), Vector::new(0, 0, 1), 0),
            (Point::new(0, 1, -5), Vector::new(0, 0, 1), 0),
            (Point::new(0.0, 1.5, -2.0), Vector::new(0, 0, 1), 2),
        ];

        for test in test_cases {
            let direction = test.1.normalize();
            let r = Ray::new(test.0, direction);
            let xs = cyl.local_intersect(&r);
            assert_eq!(xs.len(), test.2);
        }
    }

    #[test]
    fn default_closed_value_for_cylinder() {
        let cyl = Cylinder::default();
        assert_eq!(cyl.closed, false);
    }

    #[test]
    fn intersect_caps_closed_cylinder() {
        let cyl = Cylinder::new(1, 2, true);

        let test_cases = vec![
            (Point::new(0, 3, 0), Vector::new(0, -1, 0), 2),
            (Point::new(0, 3, -2), Vector::new(0, -1, 2), 2),
            (Point::new(0, 4, -2), Vector::new(0, -1, 1), 2),
            (Point::new(0, 0, -2), Vector::new(0, 1, 2), 2),
            (Point::new(0, -1, -2), Vector::new(0, 1, 1), 2),
        ];

        for test in test_cases {
            let direction = test.1.normalize();
            let r = Ray::new(test.0, direction);
            let xs = cyl.local_intersect(&r);
            assert_eq!(xs.len(), test.2);
        }
    }

    #[test]
    fn normal_vector_on_cylinder_end_cap() {
        let cyl = Cylinder::new(1, 2, true);
        let test_cases = vec![
            (Point::new(0, 1, 0), Vector::new(0, -1, 0)),
            (Point::new(0.5, 1.0, 0.0), Vector::new(0, -1, 0)),
            (Point::new(0.0, 1.0, 0.5), Vector::new(0, -1, 0)),
            (Point::new(0, 2, 0), Vector::new(0, 1, 0)),
            (Point::new(0.5, 2.0, 0.0), Vector::new(0, 1, 0)),
            (Point::new(0.0, 2.0, 0.5), Vector::new(0, 1, 0)),
        ];

        for test in test_cases {
            let n = cyl.local_normal_at(test.0);
            assert_eq!(n, test.1);
        }
    }
}
