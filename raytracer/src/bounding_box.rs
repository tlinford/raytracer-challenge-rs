use std::f64;

use crate::point::Point;

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

    pub fn add(&mut self, point: Point) {
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
}

#[cfg(test)]
mod tests {
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

        bb.add(p1);
        bb.add(p2);

        assert_eq!(bb.min, Point::new(-5, 0, -3));
        assert_eq!(bb.max, Point::new(7, 2, 0));
    }
}
