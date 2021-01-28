use std::any::Any;

use crate::{
    geometry::{intersection::Intersection, BaseShape, Shape},
    point::Point,
    ray::Ray,
    vector::Vector,
    EPSILON,
};

#[derive(Debug)]
pub struct Plane {
    base: BaseShape,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            base: BaseShape::default(),
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

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        if ray.direction().y.abs() < EPSILON {
            vec![]
        } else {
            let t = -ray.origin().y / ray.direction().y;
            vec![Intersection::new(t, self)]
        }
    }

    fn local_normal_at(&self, _point: Point) -> Vector {
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
        let n1 = p.local_normal_at(Point::origin());
        let n2 = p.local_normal_at(Point::new(10, 0, -10));
        let n3 = p.local_normal_at(Point::new(-5, 0, 150));

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
}
