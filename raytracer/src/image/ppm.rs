use anyhow::Result;
use std::{fs::File, io::Write, path::Path};

use crate::{canvas::Canvas, color::Color};

use super::ExportCanvas;

pub struct PpmExporter {}

impl ExportCanvas for PpmExporter {
    fn save(&self, canvas: &Canvas, path: &Path) -> Result<()> {
        save_ppm(canvas, path)
    }
}

pub fn save_ppm(canvas: &Canvas, path: &Path) -> Result<()> {
    let ppm = canvas_to_ppm(&canvas);
    let mut file = File::create(path)?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}

pub fn canvas_to_ppm(canvas: &Canvas) -> String {
    let mut ppm = ppm_header(canvas);

    for j in 0..canvas.height() {
        let mut line = String::new();
        for i in 0..canvas.width() {
            let pixel = encode_pixel(&canvas.get_pixel(i, j));
            for (idx, val) in pixel.iter().enumerate() {
                if line.len() + val.len() > 70 {
                    ppm += &line.trim_end();
                    ppm += "\n";
                    line = String::new();
                }
                line += val;
                if idx < 2 {
                    line += " ";
                }
            }
            if i < canvas.width() - 1 {
                line += " ";
            }
        }
        ppm += &line;
        ppm += "\n";
    }

    ppm
}

fn ppm_header(canvas: &Canvas) -> String {
    format!(
        "\
    P3\n\
    {} {}\n\
    255\n\
    ",
        canvas.width(),
        canvas.height()
    )
}

fn encode_pixel(color: &Color) -> [String; 3] {
    [
        scale_color_component(color.red).to_string(),
        scale_color_component(color.green).to_string(),
        scale_color_component(color.blue).to_string(),
    ]
}

fn scale_color_component(value: f64) -> u8 {
    (value * 255.0).round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = canvas_to_ppm(&c);
        let expected = "P3\n5 3\n255";
        let header: Vec<_> = ppm.lines().take(3).collect();
        assert_eq!(header.join("\n"), expected);
    }

    #[test]
    fn construct_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.set_pixel(0, 0, c1);
        c.set_pixel(2, 1, c2);
        c.set_pixel(4, 2, c3);
        let ppm = canvas_to_ppm(&c);

        let expected = "\
        255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
        0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\
        ";

        let pixel_data: Vec<_> = ppm.lines().skip(3).collect();
        assert_eq!(pixel_data.join("\n"), expected);
    }

    #[test]
    fn color_component_scaling() {
        assert_eq!(scale_color_component(0.0), 0);
        assert_eq!(scale_color_component(255.0), 255);
        assert_eq!(scale_color_component(-0.5), 0);
        assert_eq!(scale_color_component(1.5), 255);
        assert_eq!(scale_color_component(0.5), 128);
    }

    #[test]
    fn encode_single_pixel() {
        let c = Color::new(0.0, 0.5, 0.0);
        let expected = ["0", "128", "0"];
        assert_eq!(encode_pixel(&c), expected);
    }

    #[test]
    fn split_long_lines() {
        let mut canvas = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);

        for j in 0..canvas.height() {
            for i in 0..canvas.width() {
                canvas.set_pixel(i, j, color);
            }
        }

        let ppm = canvas_to_ppm(&canvas);

        let expected = "\
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
        153 255 204 153 255 204 153 255 204 153 255 204 153\n\
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
        153 255 204 153 255 204 153 255 204 153 255 204 153\
        ";

        let lines: Vec<_> = ppm.lines().skip(3).take(4).collect();

        assert_eq!(lines.join("\n"), expected);
    }
}
