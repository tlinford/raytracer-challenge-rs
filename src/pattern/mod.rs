use checkers::CheckersPattern;
use gradient::GradientPattern;
use ring::RingPattern;
use stripe::StripePattern;

use crate::{color::Color, geometry::shape::Shape, matrix::Matrix, point::Point};

use self::test_pattern::TestPattern;

mod checkers;
mod gradient;
mod ring;
mod stripe;
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
            pattern: Kind::Test(TestPattern {}),
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
            Kind::Test(test_pattern) => test_pattern.color_at(pattern_point),
            Kind::Stripe(stripe_pattern) => stripe_pattern.color_at(pattern_point),
            Kind::Gradient(gradient_pattern) => gradient_pattern.color_at(pattern_point),
            Kind::Ring(ring_pattern) => ring_pattern.color_at(pattern_point),
            Kind::Checkers(checkers_pattern) => checkers_pattern.color_at(pattern_point),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Kind {
    Test(TestPattern),
    Stripe(StripePattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
    Checkers(CheckersPattern),
}

fn _test_pattern() -> Pattern {
    Pattern::default()
}

pub fn stripe_pattern(a: Color, b: Color) -> Pattern {
    Pattern {
        pattern: Kind::Stripe(StripePattern::new(a, b)),
        ..Default::default()
    }
}

pub fn gradient_pattern(a: Color, b: Color) -> Pattern {
    Pattern {
        pattern: Kind::Gradient(GradientPattern::new(a, b)),
        ..Default::default()
    }
}

pub fn ring_pattern(a: Color, b: Color) -> Pattern {
    Pattern {
        pattern: Kind::Ring(RingPattern::new(a, b)),
        ..Default::default()
    }
}

pub fn checkers_pattern(a: Color, b: Color) -> Pattern {
    Pattern {
        pattern: Kind::Checkers(CheckersPattern::new(a, b)),
        ..Default::default()
    }
}
