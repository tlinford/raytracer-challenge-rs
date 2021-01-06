use crate::{color::Color, geometry::shape::Shape, matrix::Matrix, point::Point};

#[derive(Debug, PartialEq, Clone)]
pub struct StripePattern {
    a: Color,
    b: Color,
    transform: Matrix,
    transform_inverse: Matrix,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Matrix::identity(4, 4),
            transform_inverse: Matrix::identity(4, 4),
        }
    }

    pub fn color_at(&self, point: Point) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn color_at_object(&self, object: &Shape, world_point: Point) -> Color {
        let object_point = &object.transform_inverse * world_point;
        let pattern_point = &self.transform_inverse * object_point;
        self.color_at(pattern_point)
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
        self.transform_inverse = self.transform.inverse();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::shape::sphere,
        transform::{scaling, translation},
    };

    use super::*;

    #[test]
    fn create_pattern() {
        let black = Color::black();
        let white = Color::white();
        let pattern = StripePattern::new(white, black);

        assert_eq!(pattern.a, white);
        assert_eq!(pattern.b, black);
    }

    #[test]
    fn stripe_pattern_constant_y() {
        let black = Color::black();
        let white = Color::white();
        let pattern = StripePattern::new(white, black);

        assert_eq!(pattern.color_at(Point::new(0, 0, 0)), white);
        assert_eq!(pattern.color_at(Point::new(0, 1, 0)), white);
        assert_eq!(pattern.color_at(Point::new(0, 2, 0)), white);
    }

    #[test]
    fn stripe_pattern_constant_z() {
        let black = Color::black();
        let white = Color::white();
        let pattern = StripePattern::new(white, black);

        assert_eq!(pattern.color_at(Point::new(0, 0, 0)), white);
        assert_eq!(pattern.color_at(Point::new(0, 0, 1)), white);
        assert_eq!(pattern.color_at(Point::new(0, 0, 2)), white);
    }

    #[test]
    fn stripe_pattern_alternates_x() {
        let black = Color::black();
        let white = Color::white();
        let pattern = StripePattern::new(white, black);

        assert_eq!(pattern.color_at(Point::new(0, 0, 0)), white);
        assert_eq!(pattern.color_at(Point::new(0.9, 0.0, 0.0)), white);
        assert_eq!(pattern.color_at(Point::new(1, 0, 0)), black);
        assert_eq!(pattern.color_at(Point::new(-0.1, 0.0, 0.0)), black);
        assert_eq!(pattern.color_at(Point::new(-1, 0, 0)), black);
        assert_eq!(pattern.color_at(Point::new(-1.1, 0.0, 0.0)), white);
    }

    #[test]
    fn stripes_with_object_transformation() {
        let mut object = sphere();
        object.set_transform(&scaling(2, 2, 2));

        let black = Color::black();
        let white = Color::white();
        let pattern = StripePattern::new(white, black);

        let c = pattern.color_at_object(&object, Point::new(1.5, 0.0, 0.0));
        assert_eq!(c, white);
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let object = sphere();

        let black = Color::black();
        let white = Color::white();
        let mut pattern = StripePattern::new(white, black);
        pattern.set_transform(scaling(2, 2, 2));

        let c = pattern.color_at_object(&object, Point::new(1.5, 0.0, 0.0));
        assert_eq!(c, white);
    }

    #[test]
    fn stripes_with_pattern_and_object_transformation() {
        let mut object = sphere();
        object.set_transform(&scaling(2, 2, 2));

        let black = Color::black();
        let white = Color::white();
        let mut pattern = StripePattern::new(white, black);
        pattern.set_transform(translation(0.5, 0.0, 0.0));

        let c = pattern.color_at_object(&object, Point::new(2.5, 0.0, 0.0));
        assert_eq!(c, white);
    }
}
