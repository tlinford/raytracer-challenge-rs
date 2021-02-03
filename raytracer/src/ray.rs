use crate::{matrix::Matrix, point::Point, vector::Vector};

#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }

    pub fn position<T: Into<f64> + Copy>(&self, t: T) -> Point {
        self.origin + self.direction * t.into()
    }

    pub fn transform(&self, m: &Matrix) -> Self {
        Self::new(m * self.origin, m * self.direction)
    }
}

#[cfg(test)]
mod tests {
    use crate::transform::{scaling, translation};

    use super::*;

    #[test]
    fn create_ray() {
        let origin = Point::new(1, 2, 3);
        let direction = Vector::new(4, 5, 6);

        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn compute_point_from_distance() {
        let r = Ray::new(Point::new(2, 3, 4), Vector::new(1, 0, 0));
        assert_eq!(r.position(0), Point::new(2, 3, 4));
        assert_eq!(r.position(1), Point::new(3, 3, 4));
        assert_eq!(r.position(-1), Point::new(1, 3, 4));
        assert_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn translate_ray() {
        let r = Ray::new(Point::new(1, 2, 3), Vector::new(0, 1, 0));
        let m = translation(3, 4, 5);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, Point::new(4, 6, 8));
        assert_eq!(r2.direction, Vector::new(0, 1, 0));
    }

    #[test]
    fn scale_ray() {
        let r = Ray::new(Point::new(1, 2, 3), Vector::new(0, 1, 0));
        let m = scaling(2, 3, 4);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, Point::new(2, 6, 12));
        assert_eq!(r2.direction, Vector::new(0, 3, 0));
    }
}
