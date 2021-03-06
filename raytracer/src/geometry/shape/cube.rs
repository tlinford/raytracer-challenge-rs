use std::any::Any;

use crate::{
    bounding_box::BoundingBox,
    equal,
    geometry::{intersection::Intersection, BaseShape, Shape},
    point::Point,
    ray::Ray,
    vector::Vector,
    EPSILON,
};

#[derive(Debug, PartialEq)]
pub struct Cube {
    base: BaseShape,
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            base: BaseShape {
                bounding_box: BoundingBox::new(Point::new(-1, -1, -1), Point::new(1, 1, 1)),
                ..Default::default()
            },
        }
    }
}

impl Cube {
    fn check_axis(&self, origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let (tmin, tmax) = if direction.abs() >= EPSILON {
            (tmin_numerator / direction, tmax_numerator / direction)
        } else {
            (
                tmin_numerator * f64::INFINITY,
                tmax_numerator * f64::INFINITY,
            )
        };

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }
}

impl Shape for Cube {
    fn get_base(&self) -> &BaseShape {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn Shape) -> bool {
        other
            .as_any()
            .downcast_ref::<Cube>()
            .map_or(false, |a| self == a)
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let (xtmin, xtmax) = self.check_axis(ray.origin().x, ray.direction().x);
        let (ytmin, ytmax) = self.check_axis(ray.origin().y, ray.direction().y);
        let (ztmin, ztmax) = self.check_axis(ray.origin().z, ray.direction().z);

        // let tmin = [xtmin, ytmin, ztmin]
        //     .iter()
        //     .copied()
        //     .fold(f64::NAN, f64::max);
        let tmin = xtmin.max(ytmin).max(ztmin);

        // let tmax = [xtmax, ytmax, ztmax]
        //     .iter()
        //     .copied()
        //     .fold(f64::INFINITY, f64::min);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin > tmax {
            vec![]
        } else {
            vec![Intersection::new(tmin, self), Intersection::new(tmax, self)]
        }
    }

    fn local_normal_at(&self, point: Point, _intersection: &Intersection) -> Vector {
        let maxc = point.x.abs().max(point.y.abs()).max(point.z.abs());

        if equal(maxc, point.x.abs()) {
            Vector::new(point.x, 0.0, 0.0)
        } else if equal(maxc, point.y.abs()) {
            Vector::new(0.0, point.y, 0.0)
        } else {
            Vector::new(0.0, 0.0, point.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{equal, point::Point, vector::Vector};

    use super::*;

    #[test]
    fn ray_intersects_cube() {
        let c = Cube::default();
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
            TestCase::new(Point::new(5.0, 0.5, 0.0), Vector::new(-1, 0, 0), 4.0, 6.0),
            TestCase::new(Point::new(-5.0, 0.5, 0.0), Vector::new(1, 0, 0), 4.0, 6.0),
            TestCase::new(Point::new(0.5, 5.0, 0.0), Vector::new(0, -1, 0), 4.0, 6.0),
            TestCase::new(Point::new(0.5, -5.0, 0.0), Vector::new(0, 1, 0), 4.0, 6.0),
            TestCase::new(Point::new(0.5, 0.0, 5.0), Vector::new(0, 0, -1), 4.0, 6.0),
            TestCase::new(Point::new(0.5, 0.0, -5.0), Vector::new(0, 0, 1), 4.0, 6.0),
            TestCase::new(Point::new(0.0, 0.5, 0.0), Vector::new(0, 0, 1), -1.0, 1.0),
        ];

        for test in test_cases {
            let r = Ray::new(test.origin, test.direction);
            let xs = c.local_intersect(&r);
            assert_eq!(xs.len(), 2);
            assert!(equal(xs[0].t(), test.t1));
            assert!(equal(xs[1].t(), test.t2));
        }
    }

    #[test]
    fn ray_misses_cube() {
        struct TestCase {
            origin: Point,
            direction: Vector,
        }

        impl TestCase {
            fn new(origin: Point, direction: Vector) -> Self {
                Self { origin, direction }
            }
        }

        let test_cases = vec![
            TestCase::new(Point::new(-2, 0, 0), Vector::new(0.2673, 0.5345, 0.8018)),
            TestCase::new(Point::new(0, -2, 0), Vector::new(0.8018, 0.2673, 0.5345)),
            TestCase::new(Point::new(0, 0, -2), Vector::new(0.5345, 0.8018, 0.2673)),
            TestCase::new(Point::new(2, 0, 2), Vector::new(0, 0, -1)),
            TestCase::new(Point::new(0, 2, 2), Vector::new(0, -1, 0)),
            TestCase::new(Point::new(2, 2, 0), Vector::new(-1, 0, 0)),
        ];

        let c = Cube::default();

        for test in test_cases {
            let r = Ray::new(test.origin, test.direction);
            let xs = c.local_intersect(&r);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn normal_on_cube_surface() {
        struct TestCase {
            point: Point,
            normal: Vector,
        }

        impl TestCase {
            fn new(point: Point, normal: Vector) -> Self {
                Self { point, normal }
            }
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase::new(Point::new(1.0, 0.5, -0.8), Vector::new(1, 0, 0)),
            TestCase::new(Point::new(-1.0, -0.2, -0.9), Vector::new(-1, 0, 0)),
            TestCase::new(Point::new(-0.4, 1.0, -0.1), Vector::new(0, 1, 0)),
            TestCase::new(Point::new(0.3, -1.0, -0.7), Vector::new(0, -1, 0)),
            TestCase::new(Point::new(-0.6, 0.3, 1.0), Vector::new(0, 0, 1)),
            TestCase::new(Point::new(0.4, 0.4, -1.0), Vector::new(0, 0, -1)),
            TestCase::new(Point::new(1, 1, 1), Vector::new(1, 0, 0)),
            TestCase::new(Point::new(-1, -1, -1), Vector::new(-1, 0, 0)),
        ];

        let c = Cube::default();

        for test in test_cases {
            let normal = c.local_normal_at(test.point, &Intersection::new(-100.0, &c));
            assert_eq!(normal, test.normal);
        }
    }

    #[test]
    fn cube_bounding_box() {
        let s = Cube::default();
        let bb = s.get_bounds();
        assert_eq!(bb.get_min(), Point::new(-1, -1, -1));
        assert_eq!(bb.get_max(), Point::new(1, 1, 1));
    }
}
