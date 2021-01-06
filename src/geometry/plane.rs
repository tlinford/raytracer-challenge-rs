use crate::{point::Point, ray::Ray, vector::Vector, EPSILON};

#[derive(Debug, PartialEq)]
pub struct Plane {}

impl Plane {
    pub fn local_normal_at(&self, _point: Point) -> Vector {
        Vector::new(0, 1, 0)
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        if ray.direction().y.abs() < EPSILON {
            vec![]
        } else {
            let t = -ray.origin().y / ray.direction().y;
            vec![t]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::equal;

    use super::*;

    #[test]
    fn plane_normal_is_constant() {
        let p = Plane {};
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
        let p = Plane {};
        let r = Ray::new(Point::new(0, 10, 0), Vector::new(0, 0, 1));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_coplanar_ray() {
        let p = Plane {};
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_ray_from_above() {
        let p = Plane {};
        let r = Ray::new(Point::new(0, 1, 0), Vector::new(0, -1, 0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert!(equal(xs[0], 1.0))
    }

    #[test]
    fn intersect_ray_from_below() {
        let p = Plane {};
        let r = Ray::new(Point::new(0, -1, 0), Vector::new(0, 1, 0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert!(equal(xs[0], 1.0))
    }
}
