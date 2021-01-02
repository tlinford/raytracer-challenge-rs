use crate::{color::Color, point::Point};

pub struct PointLight {
    intensity: Color,
    position: Point,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> Self {
        Self {
            intensity,
            position,
        }
    }

    pub fn intensity(&self) -> Color {
        self.intensity
    }

    pub fn position(&self) -> Point {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_point_light() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point::origin();
        let light = PointLight::new(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
