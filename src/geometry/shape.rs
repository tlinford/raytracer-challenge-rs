use crate::{material::Material, matrix::Matrix, point::Point, ray::Ray, vector::Vector};

use super::{
    cube::Cube, intersection::Intersection, plane::Plane, sphere::Sphere, test_shape::TestShape,
};

#[derive(Debug, PartialEq)]
pub struct Shape {
    pub transform: Matrix,
    pub transform_inverse: Matrix,
    transform_inverse_transpose: Matrix,
    pub material: Material,
    pub shape: Kind,
}

impl Shape {
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(&self.transform_inverse);
        let xs = match &self.shape {
            Kind::TestShape(test_shape) => test_shape.local_intersect(&local_ray),
            Kind::Sphere(sphere) => sphere.local_intersect(&local_ray),
            Kind::Plane(plane) => plane.local_intersect(&local_ray),
            Kind::Cube(cube) => cube.local_intersect(&local_ray),
        };
        xs.iter().map(|&x| Intersection::new(x, &self)).collect()
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        let local_point = &self.transform_inverse * point;
        let local_normal = match &self.shape {
            Kind::TestShape(test_shape) => test_shape.local_normal_at(local_point),
            Kind::Sphere(sphere) => sphere.local_normal_at(local_point),
            Kind::Plane(plane) => plane.local_normal_at(local_point),
            Kind::Cube(cube) => cube.local_normal_at(local_point),
        };
        let world_normal = &self.transform_inverse_transpose * local_normal;
        world_normal.normalize()
    }

    pub fn transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: &Matrix) {
        self.transform = transform.clone();
        self.transform_inverse = self.transform.inverse();
        self.transform_inverse_transpose = self.transform_inverse.transpose();
    }
}

impl Default for Shape {
    fn default() -> Self {
        let transform = Matrix::identity(4, 4);
        let transform_inverse = Matrix::identity(4, 4);
        let transform_inverse_transpose = Matrix::identity(4, 4);
        Self {
            transform,
            transform_inverse,
            transform_inverse_transpose,
            material: Material::default(),
            shape: Kind::TestShape(TestShape::default()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    TestShape(TestShape),
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
}

pub fn sphere() -> Shape {
    Shape {
        shape: Kind::Sphere(Sphere {}),
        ..Default::default()
    }
}

pub fn glass_sphere() -> Shape {
    let mut sphere = sphere();
    sphere.material.transparency = 1.0;
    sphere.material.refractive_index = 1.5;

    sphere
}

pub fn plane() -> Shape {
    Shape {
        shape: Kind::Plane(Plane {}),
        ..Default::default()
    }
}

pub fn cube() -> Shape {
    Shape {
        shape: Kind::Cube(Cube {}),
        ..Default::default()
    }
}
