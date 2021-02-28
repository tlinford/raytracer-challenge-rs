use std::path::Path;

use anyhow::Result;
use image::{ImageBuffer, RgbImage};

use super::ExportCanvas;
use crate::canvas::Canvas;

#[derive(Debug)]
pub struct PngExporter {}

impl ExportCanvas for PngExporter {
    fn save(&self, canvas: &Canvas, path: &Path) -> Result<()> {
        let mut img: RgbImage = ImageBuffer::new(canvas.width() as u32, canvas.height() as u32);
        for y in 0..canvas.height() {
            for x in 0..canvas.width() {
                let color = canvas.get_pixel(x, y);
                let r = scale_color_component(color.red);
                let g = scale_color_component(color.green);
                let b = scale_color_component(color.blue);
                img.put_pixel(x as u32, y as u32, image::Rgb([r, g, b]));
            }
        }
        img.save(path)?;
        Ok(())
    }
}

fn scale_color_component(value: f64) -> u8 {
    (value * 255.0).round() as u8
}
