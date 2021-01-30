use std::{any::Any, vec};

use crate::{
    geometry::{intersection::Intersection, BaseShape, Shape},
    material::Material,
    matrix::Matrix,
    point::Point,
    ray::Ray,
    vector::Vector,
};

#[derive(Debug, PartialEq)]
pub struct Group {
    base: BaseShape,
    // TODO: make it private?
    pub children: Vec<Box<dyn Shape>>,
}

impl Default for Group {
    fn default() -> Self {
        Self {
            base: BaseShape::default(),
            children: vec![],
        }
    }
}

impl Shape for Group {
    fn get_base(&self) -> &BaseShape {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        self.children
            .iter()
            .flat_map(|c| c.intersect(ray))
            .collect()
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        self.children
            .iter()
            .flat_map(|c| c.intersect(ray))
            .collect()
    }

    fn local_normal_at(&self, _point: Point) -> Vector {
        unreachable!()
    }

    fn set_transform(&mut self, transform: Matrix) {
        let inverse_transform = &self.get_base().transform_inverse.clone();
        for child in &mut self.children {
            child.set_transform(inverse_transform * &child.get_base().transform);
        }

        let inverse = transform.inverse();
        let inverse_transpose = inverse.transpose();
        self.get_base_mut().transform = transform;
        self.get_base_mut().transform_inverse = inverse;
        self.get_base_mut().transform_inverse_transpose = inverse_transpose;

        let transform = &self.get_base().transform.clone();

        for child in &mut self.children {
            child.set_transform(transform * &child.get_base().transform);
        }
    }

    fn set_material(&mut self, material: Material) {
        self.get_base_mut().material = material.clone();

        for child in &mut self.children {
            child.set_material(material.clone());
        }
    }
}

impl Group {
    pub fn add_child(&mut self, mut shape: Box<dyn Shape>) {
        shape.set_transform(&self.get_base().transform * &shape.get_base().transform);
        self.children.push(shape);
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        geometry::{intersection::intersections, shape::Sphere},
        matrix::Matrix,
        transform::{scaling, translation},
    };

    use super::*;

    #[test]
    fn create_group() {
        let g = Group::default();
        assert!(g.children.is_empty());
        assert_eq!(g.transform(), &Matrix::identity(4, 4));
    }

    #[test]
    fn add_child_to_group() {
        let mut g = Group::default();
        let s = Sphere::default();
        g.add_child(Box::new(s));

        assert!(!g.children.is_empty());
    }

    #[test]
    fn intersect_ray_with_empty_group() {
        let g = Group::default();
        let r = Ray::new(Point::origin(), Vector::new(0, 0, 1));
        let xs = g.local_intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_ray_with_nonempty_group() {
        let mut g = Group::default();
        let s1 = Sphere::default();
        let mut s2 = Sphere::default();
        s2.set_transform(translation(0, 0, -3));
        let mut s3 = Sphere::default();
        s3.set_transform(translation(5, 0, 0));

        g.add_child(Box::new(s1));
        g.add_child(Box::new(s2));
        g.add_child(Box::new(s3));

        let s1 = &g.children[0];
        let s2 = &g.children[1];

        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let xs = g.local_intersect(&r);
        let xs = intersections(&xs);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object(), s2.as_ref());
        assert_eq!(xs[1].object(), s2.as_ref());
        assert_eq!(xs[2].object(), s1.as_ref());
        assert_eq!(xs[3].object(), s1.as_ref());
    }

    #[test]
    fn intersect_transformed_group() {
        let mut g = Group::default();
        g.set_transform(scaling(2, 2, 2));
        let mut s = Sphere::default();
        s.set_transform(translation(5, 0, 0));
        g.add_child(Box::new(s));

        let r = Ray::new(Point::new(10, 0, -10), Vector::new(0, 0, 1));
        let xs = g.intersect(&r);

        assert_eq!(xs.len(), 2);
    }
}
