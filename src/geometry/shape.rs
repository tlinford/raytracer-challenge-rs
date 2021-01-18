use crate::{material::Material, matrix::Matrix, point::Point, ray::Ray, vector::Vector};
use std::fmt::Debug;

use super::intersection::Intersection;

#[derive(Debug, PartialEq)]
pub struct BaseShape {
    transform: Matrix,
    pub transform_inverse: Matrix,
    transform_inverse_transpose: Matrix,
    pub material: Material,
}

impl Default for BaseShape {
    fn default() -> Self {
        let transform = Matrix::identity(4, 4);
        let transform_inverse = Matrix::identity(4, 4);
        let transform_inverse_transpose = Matrix::identity(4, 4);
        Self {
            transform,
            transform_inverse,
            transform_inverse_transpose,
            material: Material::default(),
        }
    }
}

pub trait Shape: Debug {
    fn get_base(&self) -> &BaseShape;
    fn get_base_mut(&mut self) -> &mut BaseShape;
    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn local_normal_at(&self, point: Point) -> Vector;

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(&self.get_base().transform_inverse);
        self.local_intersect(&local_ray)
    }

    fn normal_at(&self, point: Point) -> Vector {
        let local_point = &self.get_base().transform_inverse * point;
        let local_normal = self.local_normal_at(local_point);
        let world_normal = &self.get_base().transform_inverse_transpose * local_normal;
        world_normal.normalize()
    }

    fn material(&self) -> &Material {
        &self.get_base().material
    }

    fn set_material(&mut self, material: Material) {
        self.get_base_mut().material = material;
    }

    fn transform(&self) -> &Matrix {
        &self.get_base().transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        let inverse = transform.inverse();
        let inverse_transpose = inverse.transpose();
        self.get_base_mut().transform = transform;
        self.get_base_mut().transform_inverse = inverse;
        self.get_base_mut().transform_inverse_transpose = inverse_transpose;
    }
}

impl<'a, 'b> PartialEq<dyn Shape + 'b> for dyn Shape + 'a {
    fn eq(&self, other: &dyn Shape) -> bool {
        self.get_base() == other.get_base()
    }
}
