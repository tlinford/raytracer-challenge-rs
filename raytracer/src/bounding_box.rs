use std::f64;

use crate::{matrix::Matrix, point::Point};

#[derive(Debug)]
pub struct BoundingBox {
    min: Point,
    max: Point,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            min: Point::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            max: Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }
}

impl PartialEq for BoundingBox {
    fn eq(&self, other: &Self) -> bool {
        Point::eq_ignore_inf(&self.min, &other.min) && Point::eq_ignore_inf(&self.max, &other.max)
    }
}

impl BoundingBox {
    pub fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    pub fn get_min(&self) -> Point {
        self.min
    }

    pub fn get_max(&self) -> Point {
        self.max
    }

    pub fn add_point(&mut self, point: Point) {
        if point.x > self.max.x {
            self.max.x = point.x;
        }
        if point.y > self.max.y {
            self.max.y = point.y;
        }
        if point.z > self.max.z {
            self.max.z = point.z;
        }

        if point.x < self.min.x {
            self.min.x = point.x;
        }
        if point.y < self.min.y {
            self.min.y = point.y;
        }
        if point.z < self.min.z {
            self.min.z = point.z;
        }
    }

    pub fn add_bounding_box(&mut self, bounding_box: &BoundingBox) {
        self.add_point(bounding_box.min);
        self.add_point(bounding_box.max);
    }

    pub fn contains_point(&self, point: Point) -> bool {
        (self.min.x..=self.max.x).contains(&point.x)
            && (self.min.y..=self.max.y).contains(&point.y)
            && (self.min.z..=self.max.z).contains(&point.z)
    }

    pub fn contains_bounding_box(&self, bounding_box: &BoundingBox) -> bool {
        self.contains_point(bounding_box.min) && self.contains_point(bounding_box.max)
    }

    pub fn transform(&self, matrix: &Matrix) -> BoundingBox {
        let p1 = self.min;
        let p2 = Point::new(self.min.x, self.min.y, self.max.z);
        let p3 = Point::new(self.min.x, self.max.y, self.min.z);
        let p4 = Point::new(self.min.x, self.max.y, self.max.z);
        let p5 = Point::new(self.max.x, self.min.y, self.min.z);
        let p6 = Point::new(self.max.x, self.min.y, self.max.z);
        let p7 = Point::new(self.max.x, self.max.y, self.min.z);
        let p8 = self.max;

        let mut new_bb = BoundingBox::default();
        for &p in [p1, p2, p3, p4, p5, p6, p7, p8].iter() {
            let new_point = matrix * p;
            new_bb.add_point(new_point);
        }

        new_bb
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};

    use crate::{
        geometry::{shape::Sphere, Shape},
        transform::{rotation_x, rotation_y, scaling, translation},
    };

    use super::*;

    #[test]
    fn create_bounding_box() {
        let bb = BoundingBox::default();
        assert!(bb.min.x.is_infinite() && bb.min.x > 0.0);
        assert!(bb.min.y.is_infinite() && bb.min.y > 0.0);
        assert!(bb.min.z.is_infinite() && bb.min.z > 0.0);

        assert!(bb.max.x.is_infinite() && bb.max.x < 0.0);
        assert!(bb.max.y.is_infinite() && bb.max.y < 0.0);
        assert!(bb.max.z.is_infinite() && bb.max.z < 0.0);
    }

    #[test]
    fn create_bounding_box_with_volume() {
        let bb = BoundingBox::new(Point::new(-1, -2, -3), Point::new(3, 2, 1));
        assert_eq!(bb.min, Point::new(-1, -2, -3));
        assert_eq!(bb.max, Point::new(3, 2, 1));
    }

    #[test]
    fn add_points_to_empty_bounding_box() {
        let mut bb = BoundingBox::default();

        let p1 = Point::new(-5, 2, 0);
        let p2 = Point::new(7, 0, -3);

        bb.add_point(p1);
        bb.add_point(p2);

        assert_eq!(bb.min, Point::new(-5, 0, -3));
        assert_eq!(bb.max, Point::new(7, 2, 0));
    }

    #[test]
    fn add_one_bounding_box_to_another() {
        let mut box1 = BoundingBox::new(Point::new(-5, -2, 0), Point::new(7, 4, 4));
        let box2 = BoundingBox::new(Point::new(8, -7, -2), Point::new(14, 2, 8));

        box1.add_bounding_box(&box2);
        assert_eq!(box1.get_min(), Point::new(-5, -7, -2));
        assert_eq!(box1.get_max(), Point::new(14, 4, 8));
    }

    #[test]
    fn bounding_box_contains_point() {
        let bb = BoundingBox::new(Point::new(5, -2, 0), Point::new(11, 4, 7));
        let tests = [
            (Point::new(5, -2, 0), true),
            (Point::new(11, 4, 7), true),
            (Point::new(8, 1, 3), true),
            (Point::new(3, 0, 3), false),
            (Point::new(8, -4, 3), false),
            (Point::new(8, 1, -1), false),
            (Point::new(13, 1, 3), false),
            (Point::new(8, 5, 3), false),
            (Point::new(8, 1, 8), false),
        ];

        for &(point, result) in tests.iter() {
            assert_eq!(bb.contains_point(point), result);
        }
    }

    #[test]
    fn bounding_box_contains_bounding_box() {
        let bb = BoundingBox::new(Point::new(5, -2, 0), Point::new(11, 4, 7));
        let tests = [
            (Point::new(5, -2, 0), Point::new(11, 4, 7), true),
            (Point::new(6, -1, 1), Point::new(10, 3, 6), true),
            (Point::new(4, -3, -1), Point::new(10, 3, 6), false),
            (Point::new(6, -1, 1), Point::new(12, 5, 8), false),
        ];

        for &(min, max, result) in tests.iter() {
            let bb2 = BoundingBox::new(min, max);
            assert_eq!(bb.contains_bounding_box(&bb2), result);
        }
    }

    #[test]
    fn transform_bounding_box() {
        let bb = BoundingBox::new(Point::new(-1, -1, -1), Point::new(1, 1, 1));
        let matrix = &rotation_x(PI / 4.0) * &rotation_y(PI / 4.0);

        let bb2 = bb.transform(&matrix);
        assert_eq!(bb2.get_min(), Point::new(-SQRT_2, -1.70711, -1.70711));
        assert_eq!(bb2.get_max(), Point::new(SQRT_2, 1.70711, 1.70711))
    }

    #[test]
    fn query_shape_bounding_box_in_parent_space() {
        let mut shape = Sphere::default();
        shape.set_transform(&translation(1, -3, 5) * &scaling(0.5, 2.0, 4.0));
        let bb = shape.parent_space_bounds();

        assert_eq!(bb.min, Point::new(0.5, -5.0, 1.0));
        assert_eq!(bb.max, Point::new(1.5, -1.0, 9.0));
    }
}
