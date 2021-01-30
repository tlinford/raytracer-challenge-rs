use std::any::Any;

use crate::{
    geometry::{intersection::Intersection, BaseShape, Shape},
    point::Point,
    ray::Ray,
    vector::{cross, dot, Vector},
    EPSILON,
};

#[derive(Debug)]
pub struct Triangle {
    base: BaseShape,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    e1: Vector,
    e2: Vector,
    normal: Vector,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        Self {
            base: BaseShape::default(),
            p1,
            p2,
            p3,
            e1,
            e2,
            normal: cross(e2, e1).normalize(),
        }
    }
}

impl Shape for Triangle {
    fn get_base(&self) -> &BaseShape {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let dir_cross_e2 = cross(ray.direction(), self.e2);
        let det = dot(self.e1, dir_cross_e2);

        if det.abs() < EPSILON {
            return vec![];
        }

        let f = 1.0 / det;
        let p1_to_origin = ray.origin() - self.p1;
        let u = f * dot(p1_to_origin, dir_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            return vec![];
        }

        let origin_cross_e1 = cross(p1_to_origin, self.e1);
        let v = f * dot(ray.direction(), origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return vec![];
        }

        let t = f * dot(self.e2, origin_cross_e1);
        vec![Intersection::new(t, self)]
    }

    fn local_normal_at(&self, _point: Point, _intersection: &Intersection) -> Vector {
        self.normal
    }
}

#[cfg(test)]
mod tests {

    use crate::equal;

    use super::*;

    #[test]
    fn construct_triangle() {
        let p1 = Point::new(0, 1, 0);
        let p2 = Point::new(-1, 0, 0);
        let p3 = Point::new(1, 0, 0);

        let t = Triangle::new(p1, p2, p3);

        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);

        assert_eq!(t.e1, Vector::new(-1, -1, 0));
        assert_eq!(t.e2, Vector::new(1, -1, 0));
        assert_eq!(t.normal, Vector::new(0, 0, -1));
    }

    #[test]
    fn find_normal_on_triangle() {
        let t = Triangle::new(
            Point::new(0, 1, 0),
            Point::new(-1, 0, 0),
            Point::new(1, 0, 0),
        );
        let n1 = t.local_normal_at(Point::new(0.0, 0.5, 0.0), &Intersection::new(-100.0, &t));
        let n2 = t.local_normal_at(Point::new(-0.5, 0.75, 0.0), &Intersection::new(-100.0, &t));
        let n3 = t.local_normal_at(Point::new(0.5, 0.25, 0.0), &Intersection::new(-100.0, &t));

        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
        assert_eq!(n3, t.normal);
    }

    #[test]
    fn intersect_ray_parallel_to_triangle() {
        let t = Triangle::new(
            Point::new(0, 1, 0),
            Point::new(-1, 0, 0),
            Point::new(1, 0, 0),
        );
        let r = Ray::new(Point::new(0, -1, -2), Vector::new(0, 1, 0));
        let xs = t.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_misses_p1_p3_edge() {
        let t = Triangle::new(
            Point::new(0, 1, 0),
            Point::new(-1, 0, 0),
            Point::new(1, 0, 0),
        );
        let r = Ray::new(Point::new(1, 1, -2), Vector::new(0, 0, 1));
        let xs = t.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_misses_p1_p2_edge() {
        let t = Triangle::new(
            Point::new(0, 1, 0),
            Point::new(-1, 0, 0),
            Point::new(1, 0, 0),
        );
        let r = Ray::new(Point::new(-1, 1, -2), Vector::new(0, 0, 1));
        let xs = t.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_misses_p2_p3_edge() {
        let t = Triangle::new(
            Point::new(0, 1, 0),
            Point::new(-1, 0, 0),
            Point::new(1, 0, 0),
        );
        let r = Ray::new(Point::new(0, -1, -2), Vector::new(0, 0, 1));
        let xs = t.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_strikes_triangle() {
        let t = Triangle::new(
            Point::new(0, 1, 0),
            Point::new(-1, 0, 0),
            Point::new(1, 0, 0),
        );
        let r = Ray::new(Point::new(0.0, 0.5, -2.0), Vector::new(0, 0, 1));
        let xs = t.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert!(equal(xs[0].t(), 2.0));
    }
}
