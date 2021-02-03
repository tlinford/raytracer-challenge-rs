use std::{any::Any, vec};

use crate::{
    geometry::{intersection::Intersection, BaseShape, Shape},
    point::Point,
    ray::Ray,
    vector::{cross, dot, Vector},
    EPSILON,
};

#[derive(Debug, PartialEq)]
pub struct SmoothTriangle {
    base: BaseShape,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub n1: Vector,
    pub n2: Vector,
    pub n3: Vector,
    e1: Vector,
    e2: Vector,
    normal: Vector,
}

impl SmoothTriangle {
    pub fn new(p1: Point, p2: Point, p3: Point, n1: Vector, n2: Vector, n3: Vector) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        Self {
            base: BaseShape::default(),
            p1,
            p2,
            p3,
            n1,
            n2,
            n3,
            e1,
            e2,
            normal: cross(e2, e1).normalize(),
        }
    }
}

impl Shape for SmoothTriangle {
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
        vec![Intersection::new_with_uv(t, self, u, v)]
    }

    fn local_normal_at(&self, _point: Point, hit: &Intersection) -> Vector {
        self.n2 * hit.u().unwrap()
            + self.n3 * hit.v().unwrap()
            + self.n1 * (1.0 - hit.u().unwrap() - hit.v().unwrap())
    }
}

#[cfg(test)]
mod tests {

    use crate::equal;

    use super::*;

    #[test]
    fn construct_smooth_triangle() {
        let p1 = Point::new(0, 1, 0);
        let p2 = Point::new(-1, 0, 0);
        let p3 = Point::new(1, 0, 0);

        let n1 = Vector::new(0, 1, 0);
        let n2 = Vector::new(-1, 0, 0);
        let n3 = Vector::new(1, 0, 0);

        let t = SmoothTriangle::new(p1, p2, p3, n1, n2, n3);

        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);
        assert_eq!(t.n1, n1);
        assert_eq!(t.n1, n1);
        assert_eq!(t.n2, n2);
        assert_eq!(t.n3, n3);
    }

    #[test]
    fn intersection_with_smooth_triangle_stores_u_and_v() {
        let p1 = Point::new(0, 1, 0);
        let p2 = Point::new(-1, 0, 0);
        let p3 = Point::new(1, 0, 0);

        let n1 = Vector::new(0, 1, 0);
        let n2 = Vector::new(-1, 0, 0);
        let n3 = Vector::new(1, 0, 0);

        let t = SmoothTriangle::new(p1, p2, p3, n1, n2, n3);

        let r = Ray::new(Point::new(-0.2, 0.3, -2.0), Vector::new(0, 0, 1));
        let xs = t.local_intersect(&r);
        assert!(equal(xs[0].u().unwrap(), 0.45));
        assert!(equal(xs[0].v().unwrap(), 0.25));
    }

    #[test]
    fn smooth_triangle_uses_u_and_v_to_interpolate_normal() {
        let p1 = Point::new(0, 1, 0);
        let p2 = Point::new(-1, 0, 0);
        let p3 = Point::new(1, 0, 0);

        let n1 = Vector::new(0, 1, 0);
        let n2 = Vector::new(-1, 0, 0);
        let n3 = Vector::new(1, 0, 0);

        let t = SmoothTriangle::new(p1, p2, p3, n1, n2, n3);

        let i = Intersection::new_with_uv(1.0, &t, 0.45, 0.25);
        let n = t.normal_at(Point::origin(), &i);
        assert_eq!(n, Vector::new(-0.5547, 0.83205, 0.0));
    }

    #[test]
    fn prepare_normal_on_smooth_triangle() {
        let p1 = Point::new(0, 1, 0);
        let p2 = Point::new(-1, 0, 0);
        let p3 = Point::new(1, 0, 0);

        let n1 = Vector::new(0, 1, 0);
        let n2 = Vector::new(-1, 0, 0);
        let n3 = Vector::new(1, 0, 0);

        let t = SmoothTriangle::new(p1, p2, p3, n1, n2, n3);

        let i = Intersection::new_with_uv(1.0, &t, 0.45, 0.25);
        let r = Ray::new(Point::new(-0.2, 0.3, -2.0), Vector::new(0, 0, 1));
        let xs = vec![i];
        let comps = i.prepare_computations(&r, &xs);
        assert_eq!(comps.normalv, Vector::new(-0.5547, 0.83205, 0.0));
    }
}
