use crate::color::Color;

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Color::new(0.0, 0.0, 0.0); width * height];

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[self.pixel_idx(x, y)]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let idx = self.pixel_idx(x, y);
        self.pixels[idx] = color;
    }

    fn pixel_idx(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width);
        assert!(y < self.height);
        y * self.width + x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for pixel in c.pixels {
            assert_eq!(pixel, Color::new(0.0, 0.0, 0.0));
        }
    }

    #[test]
    fn calculate_pixel_idx() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.pixel_idx(5, 0), 5);
        assert_eq!(c.pixel_idx(5, 1), 15);
        assert_eq!(c.pixel_idx(0, 2), 20);
        assert_eq!(c.pixel_idx(9, 19), 199);
    }

    #[test]
    #[should_panic]
    fn bad_x_index() {
        let c = Canvas::new(10, 20);
        c.pixel_idx(10, 1);
    }

    #[test]
    #[should_panic]
    fn bad_y_index() {
        let c = Canvas::new(10, 20);
        c.pixel_idx(5, 200);
    }

    #[test]
    fn write_pixel() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.set_pixel(2, 3, red);
        assert_eq!(c.get_pixel(2, 3), red);
    }
}
