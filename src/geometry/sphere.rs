use crate::{matrix::Matrix, point::Point, ray::Ray, vector::dot};

use super::intersection::{intersections, Intersection};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    transform: Matrix,
}

impl Sphere {
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let obj_ray = ray.transform(&self.transform.inverse());

        let sphere_to_ray = obj_ray.origin() - Point::origin();
        let a = dot(obj_ray.direction(), obj_ray.direction());
        let b = 2.0 * dot(obj_ray.direction(), sphere_to_ray);
        let c = dot(sphere_to_ray, sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            intersections(&[Intersection::new(t1, &self), Intersection::new(t2, &self)])
        }
    }

    pub fn transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: &Matrix) {
        self.transform = transform.clone();
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transform: Matrix::identity(4, 4),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        transform::{scaling, translation},
        vector::Vector,
    };

    use super::*;
    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert!(crate::equal(xs[0].t(), 4.0));
        assert!(crate::equal(xs[1].t(), 6.0));
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Point::new(0, 1, -5), Vector::new(0, 0, 1));
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert!(crate::equal(xs[0].t(), 5.0));
        assert!(crate::equal(xs[1].t(), 5.0));
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Point::new(0, 2, -5), Vector::new(0, 0, 1));
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert!(crate::equal(xs[0].t(), -1.0));
        assert!(crate::equal(xs[1].t(), 1.0));
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Point::new(0, 0, 5), Vector::new(0, 0, 1));
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert!(crate::equal(xs[0].t(), -6.0));
        assert!(crate::equal(xs[1].t(), -4.0));
    }

    #[test]
    fn sphere_default_transformation() {
        let s = Sphere::default();
        assert_eq!(s.transform(), &Matrix::identity(4, 4));
    }

    #[test]
    fn change_sphere_transformation() {
        let mut s = Sphere::default();
        let t = translation(2, 3, 4);
        s.set_transform(&t);
        assert_eq!(s.transform(), &t);
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut s = Sphere::default();
        s.set_transform(&scaling(2, 2, 2));

        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert!(crate::equal(xs[0].t(), 3.0));
        assert!(crate::equal(xs[1].t(), 7.0));
    }

    #[test]
    fn intersect_translated_sphere_with_ray() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut s = Sphere::default();
        s.set_transform(&translation(5, 0, 0));

        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }
}
