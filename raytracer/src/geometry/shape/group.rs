use std::{any::Any, vec};

use crate::{
    bounding_box::BoundingBox,
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

    fn equals(&self, other: &dyn Shape) -> bool {
        other
            .as_any()
            .downcast_ref::<Group>()
            .map_or(false, |a| self == a)
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        if !self.get_bounds().intersects(ray) {
            return vec![];
        }

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

    fn local_normal_at(&self, _point: Point, _intersection: &Intersection) -> Vector {
        unreachable!()
    }

    fn set_transform(&mut self, transform: Matrix) {
        // remove current transform from children
        let inverse = &self.get_base().transform_inverse.clone();
        for child in &mut self.children {
            child.set_transform(inverse * &child.get_base().transform);
        }

        // apply new transform
        let inverse = transform.inverse();
        let inverse_transpose = inverse.transpose();
        self.get_base_mut().transform = transform;
        self.get_base_mut().transform_inverse = inverse;
        self.get_base_mut().transform_inverse_transpose = inverse_transpose;

        let transform = &self.get_base().transform.clone();
        let mut new_bb = BoundingBox::default();

        // apply new transform to children
        for child in &mut self.children {
            child.set_transform(transform * &child.get_base().transform);
            new_bb.add_bounding_box(child.get_bounds());
        }
        self.get_base_mut().bounding_box = new_bb;
    }

    fn set_material(&mut self, material: Material) {
        self.get_base_mut().material = material.clone();

        for child in &mut self.children {
            child.set_material(material.clone());
        }
    }

    fn includes(&self, other: &dyn Shape) -> bool {
        self.children.iter().any(|c| c.includes(other))
    }

    fn divide(&mut self, threshold: usize) {
        if threshold <= self.children.len() {
            let (left, right) = self.partition_children();
            if !left.is_empty() {
                self.make_subgroup(left);
            }
            if !right.is_empty() {
                self.make_subgroup(right);
            }
        }

        for child in self.children.iter_mut() {
            child.divide(threshold);
        }
    }
}

type ShapesSplit = (Vec<Box<dyn Shape>>, Vec<Box<dyn Shape>>);

impl Group {
    pub fn add_child(&mut self, mut shape: Box<dyn Shape>) {
        shape.set_transform(&self.get_base().transform * &shape.get_base().transform);
        let cbox = shape.parent_space_bounds();
        self.get_base_mut().bounding_box.add_bounding_box(&cbox);
        self.children.push(shape);
    }

    fn partition_children(&mut self) -> ShapesSplit {
        let mut left = vec![];
        let mut right = vec![];

        let (left_bb, right_bb) = self.get_bounds().split();

        let mut i = 0;
        while i != self.children.len() {
            if left_bb.contains_bounding_box(&self.children[i].parent_space_bounds()) {
                left.push(self.children.remove(i));
            } else {
                i += 1;
            }
        }

        let mut i = 0;
        while i != self.children.len() {
            if right_bb.contains_bounding_box(&self.children[i].parent_space_bounds()) {
                right.push(self.children.remove(i));
            } else {
                i += 1;
            }
        }

        // let fit_left_children = self
        //     .children
        //     .iter()
        //     .enumerate()
        //     .inspect(|(i, child)| println!("child {}: {:?}", i, child))
        //     .filter(|(_, child)| left_bb.contains_bounding_box(child.get_bounds()))
        //     .map(|(i, _)| i)
        //     .collect::<Vec<_>>();

        // println!("children fitting left: {:?}", fit_left_children);

        // for i in fit_left_children {
        //     left.push(self.children.remove(i));
        // }

        // let fit_right_children = self
        //     .children
        //     .iter()
        //     .enumerate()
        //     .filter(|(_, child)| right_bb.contains_bounding_box(child.get_bounds()))
        //     .map(|(i, _)| i)
        //     .collect::<Vec<_>>();

        // println!("children fitting right: {:?}", fit_right_children);

        // for i in fit_right_children {
        //     right.push(self.children.remove(i));
        // }

        (left, right)
    }

    fn make_subgroup(&mut self, shapes: Vec<Box<dyn Shape>>) {
        let mut g = Group::default();
        for shape in shapes {
            g.add_child(shape);
        }
        self.add_child(Box::new(g));
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        geometry::{
            intersection::intersections,
            shape::{Cylinder, Sphere},
            Shape,
        },
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

    #[test]
    fn group_bounding_box_contains_its_children() {
        let mut s = Sphere::default();
        s.set_transform(&translation(2, 5, -3) * &scaling(2, 2, 2));
        println!("s bounding box: {:?}", s.get_bounds());

        let mut c = Cylinder::new(-2, 2, false);
        c.set_transform(&translation(-4, -1, 4) * &scaling(0.5, 1.0, 0.5));
        println!("c bounding box: {:?}", c.get_bounds());

        let mut shape = Group::default();
        shape.add_child(Box::new(s));
        shape.add_child(Box::new(c));

        let bb = shape.get_bounds();
        assert_eq!(bb.get_min(), Point::new(-4.5, -3.0, -5.0));
        assert_eq!(bb.get_max(), Point::new(4.0, 7.0, 4.5));
    }

    #[test]
    fn partition_group_children() {
        let mut s1 = Sphere::default();
        s1.set_transform(translation(-2, 0, 0));

        let mut s2 = Sphere::default();
        s2.set_transform(translation(2, 0, 0));

        let s3 = Sphere::default();

        let mut g = Group::default();
        g.add_child(Box::new(s1));
        g.add_child(Box::new(s2));
        g.add_child(Box::new(s3));

        let (left, right) = g.partition_children();

        assert_eq!(g.children.len(), 1);
        let s3 = &g.children[0].as_any().downcast_ref::<Sphere>().unwrap();
        assert_eq!(s3.transform(), &Matrix::identity(4, 4));

        assert_eq!(left.len(), 1);
        let s1 = &left[0].as_any().downcast_ref::<Sphere>().unwrap();
        assert_eq!(s1.transform(), &translation(-2, 0, 0));

        assert_eq!(right.len(), 1);
        let s2 = &right[0].as_any().downcast_ref::<Sphere>().unwrap();
        assert_eq!(s2.transform(), &translation(2, 0, 0));
    }

    #[test]
    fn create_subgroup_from_list_of_children() {
        let s1 = Sphere::default();
        let s2 = Sphere::default();

        let mut g = Group::default();
        g.make_subgroup(vec![Box::new(s1), Box::new(s2)]);

        assert_eq!(g.children.len(), 1);
        let g0 = g.children[0].as_any().downcast_ref::<Group>().unwrap();
        assert_eq!(g0.children.len(), 2);

        let s1 = g0.children[0].as_any().downcast_ref::<Sphere>().unwrap();
        let s2 = g0.children[1].as_any().downcast_ref::<Sphere>().unwrap();

        assert_eq!(s1, &Sphere::default());
        assert_eq!(s2, &Sphere::default())
    }

    #[test]
    fn subdividing_a_group_partitions_its_children() {
        let mut s1 = Sphere::default();
        s1.set_transform(translation(-2, -2, 0));

        let mut s2 = Sphere::default();
        s2.set_transform(translation(-2, 2, 0));

        let mut s3 = Sphere::default();
        s3.set_transform(scaling(4, 4, 4));

        let mut g = Group::default();
        g.add_child(Box::new(s1));
        g.add_child(Box::new(s2));
        g.add_child(Box::new(s3));

        g.divide(1);

        let s3 = g.children[0].as_any().downcast_ref::<Sphere>().unwrap();
        assert_eq!(s3.transform(), &scaling(4, 4, 4));

        let subgroup = g.children[1].as_any().downcast_ref::<Group>().unwrap();
        assert_eq!(subgroup.children.len(), 2);

        println!("subgroup child 0: {:?}", subgroup.children[0]);
        let subgroup_child0 = subgroup.children[0]
            .as_any()
            .downcast_ref::<Group>()
            .unwrap();
        let s1 = subgroup_child0.children[0]
            .as_any()
            .downcast_ref::<Sphere>()
            .unwrap();
        assert_eq!(s1.transform(), &translation(-2, -2, 0));

        println!("subgroup child 1: {:?}", subgroup.children[1]);
        let subgroup_child1 = subgroup.children[1]
            .as_any()
            .downcast_ref::<Group>()
            .unwrap();
        let s2 = subgroup_child1.children[0]
            .as_any()
            .downcast_ref::<Sphere>()
            .unwrap();
        assert_eq!(s2.transform(), &translation(-2, 2, 0));
    }
}
