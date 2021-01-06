use stripe::StripePattern;

use crate::{color::Color, geometry::shape::Shape, matrix::Matrix, point::Point};

use self::test_pattern::TestPattern;

pub mod stripe;
mod test_pattern;

#[derive(Debug, PartialEq, Clone)]
pub struct Pattern {
    transform: Matrix,
    transform_inverse: Matrix,
    pattern: Kind,
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            transform: Matrix::identity(4, 4),
            transform_inverse: Matrix::identity(4, 4),
            pattern: Kind::TestPattern(TestPattern {}),
        }
    }
}

impl Pattern {
    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
        self.transform_inverse = self.transform.inverse();
    }

    pub fn color_at_shape(&self, shape: &Shape, world_point: Point) -> Color {
        let object_point = &shape.transform_inverse * world_point;
        let pattern_point = &self.transform_inverse * object_point;
        match &self.pattern {
            Kind::TestPattern(test_pattern) => test_pattern.color_at(pattern_point),
            Kind::StripePattern(stripe_pattern) => stripe_pattern.color_at(pattern_point),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Kind {
    TestPattern(TestPattern),
    StripePattern(StripePattern),
}

fn _test_pattern() -> Pattern {
    Pattern::default()
}

pub fn stripe_pattern(a: Color, b: Color) -> Pattern {
    Pattern {
        pattern: Kind::StripePattern(StripePattern::new(a, b)),
        ..Default::default()
    }
}
