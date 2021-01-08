use crate::{color::Color, point::Point};

#[derive(Debug, PartialEq, Clone)]
pub struct CheckersPattern {
    a: Color,
    b: Color,
}

impl CheckersPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn color_at(&self, point: Point) -> Color {
        let distance = point.x.floor() + point.y.floor() + point.z.floor();
        if distance as isize % 2 == 0 {
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
    fn checkers_repeat_in_x() {
        let white = Color::white();
        let black = Color::black();
        let pattern = CheckersPattern::new(white, black);
        assert_eq!(pattern.color_at(Point::origin()), white);
        assert_eq!(pattern.color_at(Point::new(0.99, 0.0, 0.0)), white);
        assert_eq!(pattern.color_at(Point::new(1.01, 0.0, 0.0)), black)
    }

    #[test]
    fn checkers_repeat_in_y() {
        let white = Color::white();
        let black = Color::black();
        let pattern = CheckersPattern::new(white, black);
        assert_eq!(pattern.color_at(Point::origin()), white);
        assert_eq!(pattern.color_at(Point::new(0.0, 0.99, 0.0)), white);
        assert_eq!(pattern.color_at(Point::new(0.0, 1.01, 0.0)), black)
    }

    #[test]
    fn checkers_repeat_in_z() {
        let white = Color::white();
        let black = Color::black();
        let pattern = CheckersPattern::new(white, black);
        assert_eq!(pattern.color_at(Point::origin()), white);
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.99)), white);
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 1.01)), black)
    }
}
