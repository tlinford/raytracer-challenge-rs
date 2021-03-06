use std::{
    iter::Sum,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn average(colors: &[Color]) -> Color {
        let mut avg_color = Color::black();
        for &color in colors {
            avg_color = avg_color + color;
        }
        avg_color = avg_color * (1.0 / colors.len() as f64);
        avg_color
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        crate::equal(self.red, other.red)
            && crate::equal(self.green, other.green)
            && crate::equal(self.blue, other.blue)
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl Sub<Color> for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, s: f64) -> Self {
        Self::new(self.red * s, self.green * s, self.blue * s)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        color * self
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

impl Sum<Self> for Color {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::new(0.0, 0.0, 0.0), |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_color() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert!(crate::equal(c.red, -0.5));
        assert!(crate::equal(c.green, 0.4));
        assert!(crate::equal(c.blue, 1.7));
    }

    #[test]
    fn add_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(1.6, 0.7, 1.0);
        assert_eq!(c1 + c2, expected);
        assert_eq!(c2 + c1, expected);
    }

    #[test]
    fn subtract_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(0.2, 0.5, 0.5);
        assert_eq!(c1 - c2, expected);
    }

    #[test]
    fn multiply_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        let expected = Color::new(0.4, 0.6, 0.8);
        assert_eq!(c * 2.0, expected);
        assert_eq!(2.0 * c, expected);
    }

    #[test]
    fn multiply_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let expected = Color::new(0.9, 0.2, 0.04);
        assert_eq!(c1 * c2, expected);
        assert_eq!(c2 * c1, expected);
    }
}
