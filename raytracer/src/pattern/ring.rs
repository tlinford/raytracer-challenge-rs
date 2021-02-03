use crate::{color::Color, point::Point};

#[derive(Debug, PartialEq, Clone)]
pub struct RingPattern {
    a: Color,
    b: Color,
}

impl RingPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn color_at(&self, point: Point) -> Color {
        let distance = (point.x * point.x + point.z * point.z).sqrt().floor();
        if distance % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::color::Color;

    use super::*;

    #[test]
    fn gradient_linearly_interpolates_colors() {
        let white = Color::white();
        let black = Color::black();
        let pattern = RingPattern::new(white, black);
        assert_eq!(pattern.color_at(Point::origin()), white);
        assert_eq!(pattern.color_at(Point::new(1, 0, 0)), black);
        assert_eq!(pattern.color_at(Point::new(0, 0, 1)), black);
        assert_eq!(pattern.color_at(Point::new(0.708, 0.0, 0.708)), black)
    }
}
