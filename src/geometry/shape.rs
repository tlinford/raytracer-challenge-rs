use crate::{material::Material, matrix::Matrix, point::Point, ray::Ray, vector::Vector};

use super::{intersection::Intersection, sphere::Sphere, test_shape::TestShape};

#[derive(Debug, PartialEq)]
pub struct Shape {
    pub transform: Matrix,
    pub material: Material,
    pub shape: Kind,
}

impl Shape {
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(&self.transform.inverse());
        let xs = match &self.shape {
            Kind::TestShape(test_shape) => test_shape.local_intersect(&local_ray),
            Kind::Sphere(sphere) => sphere.local_intersect(&local_ray),
        };
        xs.iter().map(|&x| Intersection::new(x, &self)).collect()
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        let local_point = &self.transform().inverse() * point;
        let local_normal = match &self.shape {
            Kind::TestShape(test_shape) => test_shape.local_normal_at(local_point),
            Kind::Sphere(sphere) => sphere.local_normal_at(local_point),
        };
        let world_normal = &self.transform().inverse().transpose() * local_normal;
        world_normal.normalize()
    }

    pub fn transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: &Matrix) {
        self.transform = transform.clone();
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            transform: Matrix::identity(4, 4),
            material: Material::default(),
            shape: Kind::TestShape(TestShape::default()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    TestShape(TestShape),
    Sphere(Sphere),
}

pub fn sphere() -> Shape {
    Shape {
        shape: Kind::Sphere(Sphere {}),
        ..Default::default()
    }
}
