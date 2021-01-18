use crate::{color::Color, point::Point};

#[derive(Debug, PartialEq, Clone)]
pub struct StripePattern {
    a: Color,
    b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn color_at(&self, point: Point) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::{shape::Sphere, Shape},
        pattern::stripe_pattern,
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
        let mut object = Sphere::default();
        object.set_transform(scaling(2, 2, 2));

        let black = Color::black();
        let white = Color::white();
        let pattern = stripe_pattern(white, black);

        let c = pattern.color_at_shape(&object, Point::new(1.5, 0.0, 0.0));
        assert_eq!(c, white);
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let object = Sphere::default();

        let black = Color::black();
        let white = Color::white();
        let mut pattern = stripe_pattern(white, black);
        pattern.set_transform(scaling(2, 2, 2));

        let c = pattern.color_at_shape(&object, Point::new(1.5, 0.0, 0.0));
        assert_eq!(c, white);
    }

    #[test]
    fn stripes_with_pattern_and_object_transformation() {
        let mut object = Sphere::default();
        object.set_transform(scaling(2, 2, 2));

        let black = Color::black();
        let white = Color::white();
        let mut pattern = stripe_pattern(white, black);
        pattern.set_transform(translation(0.5, 0.0, 0.0));

        let c = pattern.color_at_shape(&object, Point::new(2.5, 0.0, 0.0));
        assert_eq!(c, white);
    }
}
