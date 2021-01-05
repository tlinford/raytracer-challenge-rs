use crate::{
    point::Point,
    ray::Ray,
    vector::{dot, Vector},
    EPSILON,
};

use super::shape::Shape;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Shape) -> Self {
        Self { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &Shape {
        self.object
    }

    pub fn prepare_computations(&self, ray: &Ray) -> Computations {
        let point = ray.position(self.t);
        let eyev = -ray.direction();
        let mut normalv = self.object.normal_at(point);
        let mut inside = false;
        if dot(normalv, eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        Computations {
            object: self.object,
            t: self.t,
            point,
            over_point: point + normalv * EPSILON,
            eyev,
            normalv,
            inside,
        }
    }
}

pub fn intersections<'a>(xs: &[Intersection<'a>]) -> Vec<Intersection<'a>> {
    let mut v = Vec::new();

    v.extend_from_slice(xs);
    v.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

    v
}

pub fn hit<'a>(xs: &'a [Intersection<'a>]) -> Option<&'a Intersection<'a>> {
    xs.iter().find(|&&i| i.t() >= 0.0)
}

// TODO: figure out how to make this work
// pub struct Intersections<'a> {
//     xs: Vec<Intersection<'a>>,
// }

pub struct Computations<'a> {
    pub object: &'a Shape,
    pub t: f64,
    pub point: Point,
    pub over_point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use crate::{equal, geometry::shape::sphere, transform::translation, EPSILON};

    use super::*;
    #[test]
    fn create_intersection() {
        let s = sphere();
        let i = Intersection::new(3.5, &s);
        assert!(crate::equal(i.t, 3.5));
        assert!(ptr::eq(i.object, &s));
    }

    #[test]
    fn aggregate_intersections() {
        let s = sphere();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = intersections(&[i1, i2]);
        assert_eq!(xs.len(), 2);
        assert!(crate::equal(xs[0].t(), 1.0));
        assert!(crate::equal(xs[1].t(), 2.0));
    }

    #[test]
    fn hit_all_intersections_positive_t() {
        let s = sphere();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = intersections(&[i1, i2]);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i1);
    }

    #[test]
    fn hit_some_intersections_negative_t() {
        let s = sphere();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = intersections(&[i1, i2]);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i2);
    }

    #[test]
    fn hit_all_intersections_negative_t() {
        let s = sphere();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = intersections(&[i1, i2]);
        let i = hit(&xs);
        assert!(i.is_none());
    }

    #[test]
    fn hit_is_always_nonnegative_intersection() {
        let s = sphere();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);

        let xs = intersections(&[i1, i2, i3, i4]);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i4);
    }

    #[test]
    fn precompute_intersection_state() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = sphere();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(&r);
        assert!(equal(comps.t, i.t));
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Point::new(0, 0, -1));
        assert_eq!(comps.eyev, Vector::new(0, 0, -1));
        assert_eq!(comps.normalv, Vector::new(0, 0, -1));
    }

    #[test]
    fn precompute_intersection_hit_outside() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = sphere();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn precompute_intersection_hit_inside() {
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let shape = sphere();
        let i = Intersection::new(1.0, &shape);
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.point, Point::new(0, 0, 1));
        assert_eq!(comps.eyev, Vector::new(0, 0, -1));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, Vector::new(0, 0, -1));
    }

    #[test]
    fn hit_should_offset_point() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut shape = sphere();
        shape.set_transform(&translation(0, 0, 1));
        let i = Intersection::new(5.0, &shape);
        let comps = i.prepare_computations(&r);
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
