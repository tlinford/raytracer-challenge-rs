use std::vec;

use crate::{
    geometry::{intersection::Intersection, BaseShape, Shape},
    point::Point,
    ray::Ray,
    vector::Vector,
};

#[derive(Debug, PartialEq)]
pub struct Group {
    base: BaseShape,
    children: Vec<Box<dyn Shape>>,
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

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        vec![]
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        Vector::new(point.x, point.y, point.z)
    }
}

impl Group {
    fn add_child<T: Shape + 'static>(&mut self, mut shape: T) {
        shape.set_parent(self);
        self.children.push(Box::new(shape));
    }
}

#[cfg(test)]
mod tests {
    use crate::{geometry::shape::Sphere, matrix::Matrix};

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
        g.add_child(s);

        assert!(!g.children.is_empty());
        let s = &g.children[0];
        assert_eq!(s.parent().unwrap(), &g);
    }
}
