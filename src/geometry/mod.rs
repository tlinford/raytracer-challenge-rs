pub mod intersection;
pub mod shape;

use crate::{material::Material, matrix::Matrix, point::Point, ray::Ray, vector::Vector};
use std::{fmt::Debug, ptr};

use self::{intersection::Intersection, shape::Group};

#[derive(Debug, PartialEq)]
pub struct BaseShape {
    transform: Matrix,
    pub transform_inverse: Matrix,
    transform_inverse_transpose: Matrix,
    pub material: Material,
    parent: *const Group,
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
            parent: ptr::null(),
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

    fn material_mut(&mut self) -> &mut Material {
        &mut self.get_base_mut().material
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

    fn set_parent(&mut self, parent: &Group) {
        self.get_base_mut().parent = parent;
    }

    fn parent(&self) -> Option<&Group> {
        unsafe { self.get_base().parent.as_ref() }
    }
}

impl<'a, 'b> PartialEq<dyn Shape + 'b> for dyn Shape + 'a {
    fn eq(&self, other: &dyn Shape) -> bool {
        self.get_base() == other.get_base()
    }
}

#[cfg(test)]
mod tests {
    use shape::Sphere;

    use super::*;

    #[test]
    fn shape_has_parent() {
        let s = Sphere::default();
        assert!(s.parent().is_none());
    }
}
