use crate::{
    material::Material,
    matrix::Matrix,
    point::Point,
    ray::Ray,
    vector::{dot, Vector},
};

use super::intersection::{intersections, Intersection};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    transform: Matrix,
    material: Material,
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

    pub fn normal_at(&self, world_point: Point) -> Vector {
        let object_point = &self.transform.inverse() * world_point;
        let object_normal = object_point - Point::origin();
        let world_normal = &self.transform.inverse().transpose() * object_normal;
        world_normal.normalize()
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            transform: Matrix::identity(4, 4),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{f32::consts::FRAC_1_SQRT_2, f64::consts::PI};

    use crate::{
        transform::{rotation_z, scaling, translation},
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

    #[test]
    fn sphere_normal_point_on_x_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Point::new(1, 0, 0));
        assert_eq!(n, Vector::new(1, 0, 0));
    }

    #[test]
    fn sphere_normal_point_on_y_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Point::new(0, 1, 0));
        assert_eq!(n, Vector::new(0, 1, 0));
    }

    #[test]
    fn sphere_normal_point_on_z_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Point::new(0, 0, 1));
        assert_eq!(n, Vector::new(0, 0, 1));
    }

    #[test]
    fn sphere_normal_nonaxial_point() {
        let s = Sphere::default();
        let n = s.normal_at(Point::new(
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
        ));
        assert_eq!(
            n,
            Vector::new(
                3.0f64.sqrt() / 3.0,
                3.0f64.sqrt() / 3.0,
                3.0f64.sqrt() / 3.0
            )
        );
    }

    #[test]
    fn sphere_normal_is_normalized_vector() {
        let s = Sphere::default();
        let n = s.normal_at(Point::new(
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
        ));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn translated_sphere_normal() {
        let mut s = Sphere::default();
        s.set_transform(&translation(0, 1, 0));
        let n = s.normal_at(Point::new(0.0, 1.70711, -FRAC_1_SQRT_2));
        assert_eq!(n, Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn transformed_sphere_normal() {
        let mut s = Sphere::default();
        let m = &scaling(1.0, 0.5, 1.0) * &rotation_z(PI / 5.0);

        s.set_transform(&m);
        let n = s.normal_at(Point::new(0.0, 2.0f64.sqrt() / 2.0, -(2.0f64.sqrt() / 2.0)));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn sphere_default_material() {
        let s = Sphere::default();
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn sphere_assign_material() {
        let mut s = Sphere::default();
        let mut m = Material::default();
        m.ambient = 1.0;
        s.set_material(m);
        assert_eq!(s.material, m);
    }
}
