use std::any::Any;

use crate::{
    bounding_box::BoundingBox,
    geometry::{
        intersection::{intersections, Intersection},
        BaseShape, Shape,
    },
    point::Point,
    ray::Ray,
    vector::Vector,
};

#[derive(Debug, PartialEq)]
pub enum Operation {
    Union,
    Intersection,
    Difference,
}

impl Operation {
    fn intersection_allowed(&self, lhit: bool, inl: bool, inr: bool) -> bool {
        match self {
            Self::Union => Self::union_allowed(lhit, inl, inr),
            Self::Intersection => Self::op_intersection_allowed(lhit, inl, inr),
            Self::Difference => Self::difference_allowed(lhit, inl, inr),
        }
    }

    fn union_allowed(lhit: bool, inl: bool, inr: bool) -> bool {
        (lhit && !inr) || (!lhit && !inl)
    }

    fn op_intersection_allowed(lhit: bool, inl: bool, inr: bool) -> bool {
        (lhit && inr) || (!lhit && inl)
    }

    fn difference_allowed(lhit: bool, inl: bool, inr: bool) -> bool {
        (lhit && !inr) || (!lhit && inl)
    }
}

#[derive(Debug)]
pub struct Csg {
    base: BaseShape,
    operation: Operation,
    pub left: Box<dyn Shape>,
    pub right: Box<dyn Shape>,
}

impl Csg {
    pub fn new<L: 'static + Shape, R: 'static + Shape>(
        operation: Operation,
        left: L,
        right: R,
    ) -> Self {
        let mut bb = BoundingBox::default();
        bb.add_bounding_box(&left.parent_space_bounds());
        bb.add_bounding_box(&right.parent_space_bounds());
        Self {
            base: BaseShape {
                bounding_box: bb,
                ..Default::default()
            },
            operation,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn filter_intersections<'a>(&self, xs: Vec<Intersection<'a>>) -> Vec<Intersection<'a>> {
        let mut inl = false;
        let mut inr = false;

        let mut result = vec![];

        for intersection in xs {
            let lhit = self.left.includes(intersection.object());
            if self.operation.intersection_allowed(lhit, inl, inr) {
                result.push(intersection);
            }

            if lhit {
                inl = !inl
            } else {
                inr = !inr
            }
        }

        result
    }
}

impl Shape for Csg {
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
        other.as_any().downcast_ref::<Csg>().map_or(false, |a| {
            self.get_base() == other.get_base()
                && self.left.as_ref() == a.left.as_ref()
                && self.right.as_ref() == a.right.as_ref()
        })
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        if !self.get_bounds().intersects(ray) {
            return vec![];
        }

        let mut leftxs = self.left.intersect(ray);
        let rightxs = self.right.intersect(ray);

        leftxs.extend(rightxs);
        let xs = intersections(&leftxs);
        self.filter_intersections(xs)
    }

    fn local_normal_at(&self, _point: Point, _intersection: &Intersection) -> Vector {
        unreachable!()
    }

    fn includes(&self, other: &dyn Shape) -> bool {
        self.left.includes(other) || self.right.includes(other)
    }

    fn divide(&mut self, threshold: usize) {
        self.left.divide(threshold);
        self.right.divide(threshold);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        equal,
        geometry::{
            shape::{Cube, Group, Sphere},
            Shape,
        },
        transform::translation,
    };

    use super::*;

    #[test]
    fn create_csg() {
        let s1 = Sphere::default();
        let s2 = Cube::default();

        let c = Csg::new(Operation::Union, s1, s2);
        assert_eq!(c.operation, Operation::Union);
        let s1 = c.left.as_ref().as_any().downcast_ref::<Sphere>();
        assert!(s1.is_some());
        let s2 = c.right.as_ref().as_any().downcast_ref::<Cube>();
        assert!(s2.is_some());
    }

    #[test]
    fn evaluating_rule_csg_operation() {
        struct TestCase {
            operation: Operation,
            lhit: bool,
            inl: bool,
            inr: bool,
            result: bool,
        }

        impl TestCase {
            fn new(operation: Operation, lhit: bool, inl: bool, inr: bool, result: bool) -> Self {
                Self {
                    operation,
                    lhit,
                    inl,
                    inr,
                    result,
                }
            }
        }

        let test_cases = vec![
            TestCase::new(Operation::Union, true, true, true, false),
            TestCase::new(Operation::Union, true, true, false, true),
            TestCase::new(Operation::Union, true, false, true, false),
            TestCase::new(Operation::Union, true, false, false, true),
            TestCase::new(Operation::Union, false, true, true, false),
            TestCase::new(Operation::Union, false, true, false, false),
            TestCase::new(Operation::Union, false, false, true, true),
            TestCase::new(Operation::Union, false, false, false, true),
            TestCase::new(Operation::Intersection, true, true, true, true),
            TestCase::new(Operation::Intersection, true, true, false, false),
            TestCase::new(Operation::Intersection, true, false, true, true),
            TestCase::new(Operation::Intersection, true, false, false, false),
            TestCase::new(Operation::Intersection, false, true, true, true),
            TestCase::new(Operation::Intersection, false, true, false, true),
            TestCase::new(Operation::Intersection, false, false, true, false),
            TestCase::new(Operation::Intersection, false, false, false, false),
            TestCase::new(Operation::Difference, true, true, true, false),
            TestCase::new(Operation::Difference, true, true, false, true),
            TestCase::new(Operation::Difference, true, false, true, false),
            TestCase::new(Operation::Difference, true, false, false, true),
            TestCase::new(Operation::Difference, false, true, true, true),
            TestCase::new(Operation::Difference, false, true, false, true),
            TestCase::new(Operation::Difference, false, false, true, false),
            TestCase::new(Operation::Difference, false, false, false, false),
        ];

        for test_case in test_cases {
            assert_eq!(
                test_case.operation.intersection_allowed(
                    test_case.lhit,
                    test_case.inl,
                    test_case.inr
                ),
                test_case.result
            );
        }
    }

    #[test]
    fn filter_list_of_intersections() {
        struct TestCase {
            operation: Operation,
            x0: usize,
            x1: usize,
        }

        impl TestCase {
            fn new(operation: Operation, x0: usize, x1: usize) -> Self {
                Self { operation, x0, x1 }
            }
        }

        let test_cases = vec![
            TestCase::new(Operation::Union, 0, 3),
            TestCase::new(Operation::Intersection, 1, 2),
            TestCase::new(Operation::Difference, 0, 1),
        ];

        for test_case in test_cases {
            let s1 = Sphere::default();
            let s2 = Cube::default();
            let c = Csg::new(test_case.operation, s1, s2);
            let s1 = c.left.as_ref();
            let s2 = c.right.as_ref();
            let xs = vec![
                Intersection::new(1.0, s1),
                Intersection::new(2.0, s2),
                Intersection::new(3.0, s1),
                Intersection::new(4.0, s2),
            ];

            let result = c.filter_intersections(xs.clone());
            println!("{:?}", result);
            assert_eq!(result.len(), 2);
            assert!(equal(result[0].t(), xs[test_case.x0].t()));
            assert!(equal(result[1].t(), xs[test_case.x1].t()));
        }
    }

    #[test]
    fn ray_misses_csg_object() {
        let c = Csg::new(Operation::Union, Sphere::default(), Cube::default());
        let r = Ray::new(Point::new(0, 2, -5), Vector::new(0, 0, 1));
        let xs = c.local_intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_hits_csg_object() {
        let s1 = Sphere::default();
        let mut s2 = Sphere::default();
        s2.set_transform(translation(0.0, 0.0, 0.5));
        let c = Csg::new(Operation::Union, s1, s2);
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let xs = c.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert!(equal(xs[0].t(), 4.0));
        assert!(equal(xs[1].t(), 6.5));
    }

    #[test]
    fn csg_bounding_box_contains_its_children() {
        let left = Sphere::default();
        let mut right = Sphere::default();
        right.set_transform(translation(2, 3, 4));

        let shape = Csg::new(Operation::Difference, left, right);
        let bb = shape.get_bounds();

        assert_eq!(bb.get_min(), Point::new(-1, -1, -1));
        assert_eq!(bb.get_max(), Point::new(3, 4, 5));
    }

    #[test]
    fn subdividing_csg_shape_subdivides_its_children() {
        let mut s1 = Sphere::default();
        s1.set_transform(translation(-1.5, 0.0, 0.0));

        let mut s2 = Sphere::default();
        s2.set_transform(translation(1.5, 0.0, 0.0));

        let mut left = Group::default();
        left.add_child(Box::new(s1));
        left.add_child(Box::new(s2));

        let mut s3 = Sphere::default();
        s3.set_transform(translation(0.0, 0.0, -1.5));

        let mut s4 = Sphere::default();
        s4.set_transform(translation(0.0, 0.0, 1.5));

        let mut right = Group::default();
        right.add_child(Box::new(s3));
        right.add_child(Box::new(s4));

        let mut shape = Csg::new(Operation::Difference, left, right);
        shape.divide(1);

        let left = shape.left.as_any().downcast_ref::<Group>().unwrap();
        let right = shape.right.as_any().downcast_ref::<Group>().unwrap();

        let left_child_group0 = left.children[0].as_any().downcast_ref::<Group>().unwrap();
        let s1 = left_child_group0.children[0]
            .as_any()
            .downcast_ref::<Sphere>()
            .unwrap();
        assert_eq!(s1.transform(), &translation(-1.5, 0.0, 0.0));

        let left_child_group1 = left.children[1].as_any().downcast_ref::<Group>().unwrap();
        let s2 = left_child_group1.children[0]
            .as_any()
            .downcast_ref::<Sphere>()
            .unwrap();
        assert_eq!(s2.transform(), &translation(1.5, 0.0, 0.0));

        let right_child_group0 = right.children[0].as_any().downcast_ref::<Group>().unwrap();
        let s3 = right_child_group0.children[0]
            .as_any()
            .downcast_ref::<Sphere>()
            .unwrap();
        assert_eq!(s3.transform(), &translation(0.0, 0.0, -1.5));
        let right_child_group1 = right.children[1].as_any().downcast_ref::<Group>().unwrap();
        let s4 = right_child_group1.children[0]
            .as_any()
            .downcast_ref::<Sphere>()
            .unwrap();
        assert_eq!(s4.transform(), &translation(0.0, 0.0, 1.5));
    }
}
