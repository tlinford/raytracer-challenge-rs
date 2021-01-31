pub mod intersection;
pub mod shape;

use crate::{material::Material, matrix::Matrix, point::Point, ray::Ray, vector::Vector};
use std::{any::Any, fmt::Debug, ptr};

use self::intersection::Intersection;

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
    fn local_normal_at(&self, point: Point, intersection: &Intersection) -> Vector;
    fn as_any(&self) -> &dyn Any;

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(&self.get_base().transform_inverse);
        self.local_intersect(&local_ray)
    }

    fn normal_at(&self, point: Point, intersection: &Intersection) -> Vector {
        let local_point = &self.get_base().transform_inverse * point;
        let local_normal = self.local_normal_at(local_point, intersection);
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

    fn includes(&self, other: &dyn Shape) -> bool {
        ptr::eq(self.get_base(), other.get_base())
    }
}

impl<'a, 'b> PartialEq<dyn Shape + 'b> for dyn Shape + 'a {
    fn eq(&self, other: &dyn Shape) -> bool {
        self.get_base() == other.get_base()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use shape::Sphere;

    use crate::transform::{rotation_y, scaling, translation};

    use super::{shape::Group, *};

    #[test]
    fn normal_on_child_object() {
        let mut g1 = Group::default();
        g1.set_transform(rotation_y(PI / 2.0));

        let mut g2 = Box::new(Group::default());
        g2.set_transform(scaling(1, 2, 3));

        let mut s = Box::new(Sphere::default());
        s.set_transform(translation(5, 0, 0));

        g2.add_child(s);
        g1.add_child(g2);

        let g2: &Group = (g1.children[0])
            .as_ref()
            .as_any()
            .downcast_ref::<Group>()
            .unwrap();

        let s = &g2.children[0];

        let n = s.normal_at(
            Point::new(1.7321, 1.1547, -5.5774),
            &Intersection::new(-100.0, s.as_ref()),
        );
        assert_eq!(n, Vector::new(0.2857, 0.42854, -0.85716));
    }
}
