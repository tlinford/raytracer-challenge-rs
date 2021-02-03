use crate::{color::Color, point::Point};

#[derive(Debug, PartialEq, Clone)]
pub struct GradientPattern {
    a: Color,
    b: Color,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn color_at(&self, point: Point) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();
        self.a + distance * fraction
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
        let pattern = GradientPattern::new(white, black);
        assert_eq!(pattern.color_at(Point::origin()), white);
        assert_eq!(
            pattern.color_at(Point::new(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at(Point::new(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at(Point::new(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        )
    }
}
