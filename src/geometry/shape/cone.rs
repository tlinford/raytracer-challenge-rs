use crate::{
    geometry::{intersection::Intersection, BaseShape, Shape},
    point::Point,
    ray::Ray,
    vector::Vector,
    EPSILON,
};

#[derive(Debug)]
pub struct Cone {
    base: BaseShape,
    minimum: f64,
    maximum: f64,
    closed: bool,
}

impl Default for Cone {
    fn default() -> Self {
        Self::new(f64::NEG_INFINITY, f64::INFINITY, false)
    }
}

impl Cone {
    pub fn new<T: Into<f64> + Copy>(minimum: T, maximum: T, closed: bool) -> Self {
        Self {
            base: BaseShape::default(),
            minimum: minimum.into(),
            maximum: maximum.into(),
            closed,
        }
    }

    fn intersect_caps(&self, ray: &Ray) -> Vec<Intersection> {
        let mut xs = vec![];
        if !self.closed {
            return xs;
        }

        let t = (self.minimum - ray.origin().y) / ray.direction().y;
        if self.check_cap(ray, t, self.minimum) {
            xs.push(Intersection::new(t, self));
        }

        let t = (self.maximum - ray.origin().y) / ray.direction().y;
        if self.check_cap(ray, t, self.maximum) {
            xs.push(Intersection::new(t, self));
        }

        xs
    }

    fn check_cap(&self, ray: &Ray, t: f64, radius: f64) -> bool {
        let x = ray.origin().x + t * ray.direction().x;
        let z = ray.origin().z + t * ray.direction().z;
        (x * x + z * z) <= radius * radius
    }
}

impl Shape for Cone {
    fn get_base(&self) -> &BaseShape {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let a = ray.direction().x.powi(2) - ray.direction().y.powi(2) + ray.direction().z.powi(2);
        let b = 2.0 * ray.origin().x * ray.direction().x - 2.0 * ray.origin().y * ray.direction().y
            + 2.0 * ray.origin().z * ray.direction().z;
        let c = ray.origin().x.powi(2) - ray.origin().y.powi(2) + ray.origin().z.powi(2);

        let mut xs = vec![];

        if a.abs() < EPSILON {
            if b.abs() < EPSILON {
                return self.intersect_caps(ray);
            } else {
                let t = -c / 2.0 * b;
                xs.push(Intersection::new(t, self));
                xs.append(&mut self.intersect_caps(ray));
                return xs;
            }
        }

        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            return vec![];
        }

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);

        let y0 = ray.origin().y + t0 * ray.direction().y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(Intersection::new(t0, self));
        }

        let y1 = ray.origin().y + t1 * ray.direction().y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(Intersection::new(t1, self));
        }

        xs.append(&mut self.intersect_caps(ray));

        xs
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        let dist = point.x * point.x + point.z * point.z;
        if dist < 1.0 && point.y >= self.maximum - EPSILON {
            Vector::new(0, 1, 0)
        } else if dist < 1.0 && point.y <= self.minimum + EPSILON {
            Vector::new(0, -1, 0)
        } else {
            let mut y = (point.x * point.x + point.z * point.z).sqrt();
            if point.y > 0.0 {
                y = -y;
            }
            Vector::new(point.x, y, point.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{equal, point::Point, vector::Vector};

    use super::*;

    #[test]
    fn intersect_cone_with_ray() {
        let shape = Cone::default();
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

        let test_cases: Vec<TestCase> = vec![
            TestCase::new(Point::new(0, 0, -5), Vector::new(0, 0, 1), 5.0, 5.0),
            TestCase::new(Point::new(0, 0, -5), Vector::new(1, 1, 1), 8.66025, 8.66025),
            TestCase::new(
                Point::new(1, 1, -5),
                Vector::new(-0.5, -1.0, 1.0),
                4.55006,
                49.44994,
            ),
        ];

        for test in test_cases {
            let direction = test.direction.normalize();
            let r = Ray::new(test.origin, direction);
            let xs = shape.local_intersect(&r);
            assert_eq!(xs.len(), 2);
            println!("xs[0] = {}, t1 = {}", xs[0].t(), test.t1);
            assert!(equal(xs[0].t(), test.t1));
            println!("xs[1] = {}, t2 = {}", xs[0].t(), test.t2);
            assert!(equal(xs[1].t(), test.t2));
        }
    }

    #[test]
    fn intersect_cone_end_caps() {
        let shape = Cone::new(-0.5, 0.5, true);
        struct TestCase {
            origin: Point,
            direction: Vector,
            count: usize,
        }

        impl TestCase {
            fn new(origin: Point, direction: Vector, count: usize) -> Self {
                Self {
                    origin,
                    direction,
                    count,
                }
            }
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase::new(Point::new(0, 0, -5), Vector::new(0, 1, 0), 0),
            TestCase::new(Point::new(0.0, 0.0, -0.25), Vector::new(0, 1, 1), 2),
            TestCase::new(Point::new(0.0, 0.0, -0.25), Vector::new(0, 1, 0), 4),
        ];

        for test in test_cases {
            let direction = test.direction.normalize();
            let r = Ray::new(test.origin, direction);
            let xs = shape.local_intersect(&r);
            assert_eq!(xs.len(), test.count);
        }
    }

    #[test]
    fn computing_normal_vector_cone() {
        let shape = Cone::default();
        let test_cases = vec![
            (Point::origin(), Vector::new(0, 0, 0)),
            (Point::new(1, 1, 1), Vector::new(1.0, -(2.0f64.sqrt()), 1.0)),
            (Point::new(-1, -1, 0), Vector::new(-1, 1, 0)),
        ];

        for test in test_cases {
            let n = shape.local_normal_at(test.0);
            assert_eq!(n, test.1);
        }
    }
}
