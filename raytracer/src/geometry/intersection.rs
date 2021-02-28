use crate::{
    point::Point,
    ray::Ray,
    vector::{dot, Vector},
    EPSILON,
};

use super::Shape;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'a> {
    t: f64,
    object: &'a dyn Shape,
    u: Option<f64>,
    v: Option<f64>,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a dyn Shape) -> Self {
        Self {
            t,
            object,
            u: None,
            v: None,
        }
    }

    pub fn new_with_uv(t: f64, object: &'a dyn Shape, u: f64, v: f64) -> Self {
        Self {
            t,
            object,
            u: Some(u),
            v: Some(v),
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &dyn Shape {
        self.object
    }

    pub fn u(&self) -> Option<f64> {
        self.u
    }

    pub fn v(&self) -> Option<f64> {
        self.v
    }

    pub fn prepare_computations(&self, ray: &Ray, xs: &[Intersection]) -> Computations {
        let point = ray.position(self.t);
        let eyev = -ray.direction();
        let mut normalv = self.object.normal_at(point, self);
        let mut inside = false;
        if dot(normalv, eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        let mut containers: Vec<&dyn Shape> = vec![];
        let mut n1 = -1.0;
        let mut n2 = -1.0;
        for i in xs {
            if i == self {
                if containers.is_empty() {
                    n1 = 1.0;
                } else {
                    n1 = containers.last().unwrap().material().refractive_index;
                }
            }

            if containers.contains(&i.object) {
                let idx = containers.iter().position(|&el| el == i.object).unwrap();
                containers.remove(idx);
            } else {
                containers.push(i.object);
            }

            if i == self {
                if containers.is_empty() {
                    n2 = 1.0;
                } else {
                    n2 = containers.last().unwrap().material().refractive_index;
                }
                break;
            }
        }

        Computations {
            object: self.object,
            t: self.t,
            point,
            over_point: point + normalv * EPSILON,
            under_point: point - normalv * EPSILON,
            eyev,
            normalv,
            inside,
            reflectv: ray.direction().reflect(normalv),
            n1,
            n2,
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

pub fn shadow_hit<'a>(xs: &'a [Intersection<'a>]) -> Option<&'a Intersection<'a>> {
    xs.iter()
        .find(|&&i| i.t() >= 0.0 && i.object().has_shadow())
}

// TODO: figure out how to make this work
// pub struct Intersections<'a> {
//     xs: Vec<Intersection<'a>>,
// }

pub struct Computations<'a> {
    pub object: &'a dyn Shape,
    pub t: f64,
    pub point: Point,
    pub over_point: Point,
    pub under_point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
    pub reflectv: Vector,
    pub n1: f64,
    pub n2: f64,
}

impl<'a> Computations<'a> {
    pub fn schlick(&self) -> f64 {
        let mut cos = dot(self.eyev, self.normalv);
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n * n * (1.0 - cos * cos);
            if sin2_t > 1.0 {
                return 1.0;
            }

            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t;
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        equal,
        geometry::shape::{Plane, Sphere, Triangle},
        transform::{scaling, translation},
        EPSILON,
    };

    use super::*;
    #[test]
    fn create_intersection() {
        let s = Sphere::default();
        let i = Intersection::new(3.5, &s);
        assert!(crate::equal(i.t, 3.5));
        assert_eq!(i.object.get_base(), s.get_base());
    }

    #[test]
    fn aggregate_intersections() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = intersections(&[i1, i2]);
        assert_eq!(xs.len(), 2);
        assert!(crate::equal(xs[0].t(), 1.0));
        assert!(crate::equal(xs[1].t(), 2.0));
    }

    #[test]
    fn hit_all_intersections_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = intersections(&[i1, i2]);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i1);
    }

    #[test]
    fn hit_some_intersections_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = intersections(&[i1, i2]);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i2);
    }

    #[test]
    fn hit_all_intersections_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = intersections(&[i1, i2]);
        let i = hit(&xs);
        assert!(i.is_none());
    }

    #[test]
    fn hit_is_always_nonnegative_intersection() {
        let s = Sphere::default();
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
        let shape = Sphere::default();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(&r, &[i]);
        assert!(equal(comps.t, i.t));
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Point::new(0, 0, -1));
        assert_eq!(comps.eyev, Vector::new(0, 0, -1));
        assert_eq!(comps.normalv, Vector::new(0, 0, -1));
    }

    #[test]
    fn precompute_intersection_hit_outside() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(&r, &[i]);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn precompute_intersection_hit_inside() {
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let shape = Sphere::default();
        let i = Intersection::new(1.0, &shape);
        let comps = i.prepare_computations(&r, &[i]);
        assert_eq!(comps.point, Point::new(0, 0, 1));
        assert_eq!(comps.eyev, Vector::new(0, 0, -1));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, Vector::new(0, 0, -1));
    }

    #[test]
    fn hit_should_offset_point() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut shape = Sphere::default();
        shape.set_transform(translation(0, 0, 1));
        let i = Intersection::new(5.0, &shape);
        let comps = i.prepare_computations(&r, &[i]);
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precompute_reflection_vector() {
        let shape = Plane::default();
        let r = Ray::new(
            Point::new(0, 1, -1),
            Vector::new(0.0, -(2f64.sqrt() / 2.0), 2f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0f64.sqrt(), &shape);
        let comps = i.prepare_computations(&r, &[i]);
        assert_eq!(
            comps.reflectv,
            Vector::new(0.0, 2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn finding_n1_n2_at_various_intersections() {
        let mut a = Sphere::glass();
        a.set_transform(scaling(2, 2, 2));
        a.get_base_mut().material.refractive_index = 1.5;

        let mut b = Sphere::glass();
        b.set_transform(translation(0.0, 0.0, -0.25));
        b.get_base_mut().material.refractive_index = 2.0;

        let mut c = Sphere::glass();
        c.set_transform(translation(0.0, 0.0, 0.25));
        c.get_base_mut().material.refractive_index = 2.5;

        let r = Ray::new(Point::new(0, 0, -4), Vector::new(0, 0, 1));
        let xs = intersections(&[
            Intersection::new(2.0, &a),
            Intersection::new(2.75, &b),
            Intersection::new(3.25, &c),
            Intersection::new(4.75, &b),
            Intersection::new(5.25, &c),
            Intersection::new(6.0, &a),
        ]);

        let expected = [
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];

        for (i, (n1, n2)) in expected.iter().enumerate() {
            let comps = xs[i].prepare_computations(&r, &xs);
            assert!(equal(comps.n1, *n1));
            assert!(equal(comps.n2, *n2));
        }
    }

    #[test]
    fn under_point_is_offset_below_surface() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut shape = Sphere::glass();
        shape.set_transform(translation(0, 0, 1));
        let i = Intersection::new(5.0, &shape);
        let comps = i.prepare_computations(&r, &[i]);
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn schlick_approximation_under_total_internal_reflection() {
        let shape = Sphere::glass();
        let r = Ray::new(
            Point::new(0.0, 0.0, 2.0f64.sqrt() / 2.0),
            Vector::new(0, 1, 0),
        );
        let xs = &[
            Intersection::new(-(2.0f64.sqrt() / 2.0), &shape),
            Intersection::new(2.0f64.sqrt() / 2.0, &shape),
        ];
        let comps = xs[1].prepare_computations(&r, xs);
        println!("n1: {}, n2: {}", comps.n1, comps.n2);
        let reflectance = comps.schlick();
        println!("reflectance: {}", reflectance);
        assert!(equal(reflectance, 1.0));
    }

    #[test]
    fn schlick_approximation_perpendicular_viewing_angle() {
        let shape = Sphere::glass();
        let r = Ray::new(Point::origin(), Vector::new(0, 1, 0));
        let xs = intersections(&[
            Intersection::new(-1.0, &shape),
            Intersection::new(1.0, &shape),
        ]);
        let comps = xs[1].prepare_computations(&r, &xs);
        let reflectance = comps.schlick();

        println!("reflectance: {}", reflectance);
        assert!(equal(reflectance, 0.04));
    }

    #[test]
    fn schlick_approximation_with_small_angle_and_n1_greater_than_n2() {
        let shape = Sphere::glass();
        let r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector::new(0, 0, 1));
        let xs = intersections(&[Intersection::new(1.8589, &shape)]);
        let comps = xs[0].prepare_computations(&r, &xs);
        let reflectance = comps.schlick();
        println!("reflectance: {}", reflectance);
        assert!(equal(reflectance, 0.48873));
    }

    #[test]
    fn intersection_can_have_u_and_v() {
        let s = Triangle::new(
            Point::new(0, 1, 0),
            Point::new(-1, 0, 0),
            Point::new(1, 0, 0),
        );
        let i = Intersection::new_with_uv(3.5, &s, 0.2, 0.4);
        assert!(equal(i.u.unwrap(), 0.2));
        assert!(equal(i.v.unwrap(), 0.4));
    }

    #[test]
    fn skip_hits_with_no_shadow() {
        let mut s1 = Sphere::default();
        s1.no_shadow();
        let i1 = Intersection::new(1.0, &s1);
        let i2 = Intersection::new(2.0, &s1);

        let s2 = Sphere::default();
        let i3 = Intersection::new(1.0, &s2);
        let i4 = Intersection::new(2.0, &s2);

        let xs = intersections(&[i1, i2, i3, i4]);
        let i = shadow_hit(&xs);
        assert_eq!(*i.unwrap(), i3);
    }
}
