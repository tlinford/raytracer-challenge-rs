use std::any::Any;

use std::f64::{INFINITY, NEG_INFINITY};

use crate::{
    bounding_box::BoundingBox,
    geometry::{intersection::Intersection, BaseShape, Shape},
    point::Point,
    ray::Ray,
    vector::Vector,
    EPSILON,
};

#[derive(Debug, PartialEq)]
pub struct Plane {
    base: BaseShape,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            base: BaseShape {
                bounding_box: BoundingBox::new(
                    Point::new(NEG_INFINITY, 0.0, NEG_INFINITY),
                    Point::new(INFINITY, 0.0, INFINITY),
                ),
                ..Default::default()
            },
        }
    }
}

impl Shape for Plane {
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
            .downcast_ref::<Plane>()
            .map_or(false, |a| self == a)
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        if ray.direction().y.abs() < EPSILON {
            vec![]
        } else {
            let t = -ray.origin().y / ray.direction().y;
            vec![Intersection::new(t, self)]
        }
    }

    fn local_normal_at(&self, _point: Point, _intersection: &Intersection) -> Vector {
        Vector::new(0, 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::equal;

    use super::*;

    #[test]
    fn plane_normal_is_constant() {
        let p = Plane::default();
        let n1 = p.local_normal_at(Point::origin(), &Intersection::new(-100.0, &p));
        let n2 = p.local_normal_at(Point::new(10, 0, -10), &Intersection::new(-100.0, &p));
        let n3 = p.local_normal_at(Point::new(-5, 0, 150), &Intersection::new(-100.0, &p));

        let expected = Vector::new(0, 1, 0);
        assert_eq!(n1, expected);
        assert_eq!(n2, expected);
        assert_eq!(n3, expected);
    }

    #[test]
    fn intersect_parallel_ray() {
        let p = Plane::default();
        let r = Ray::new(Point::new(0, 10, 0), Vector::new(0, 0, 1));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_coplanar_ray() {
        let p = Plane::default();
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_ray_from_above() {
        let p = Plane::default();
        let r = Ray::new(Point::new(0, 1, 0), Vector::new(0, -1, 0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert!(equal(xs[0].t(), 1.0))
    }

    #[test]
    fn intersect_ray_from_below() {
        let p = Plane::default();
        let r = Ray::new(Point::new(0, -1, 0), Vector::new(0, 1, 0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert!(equal(xs[0].t(), 1.0))
    }

    #[test]
    fn plane_bounding_box() {
        let s = Plane::default();
        let bb = s.get_bounds();
        assert!(bb.get_min().x.is_infinite() && bb.get_min().x < 0.0);
        assert!(equal(bb.get_min().y, 0.0));
        assert!(bb.get_min().z.is_infinite() && bb.get_min().z < 0.0);

        assert!(bb.get_max().x.is_infinite() && bb.get_max().x > 0.0);
        assert!(equal(bb.get_min().y, 0.0));
        assert!(bb.get_max().z.is_infinite() && bb.get_max().z > 0.0);
    }
}
