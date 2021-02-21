use anyhow::Result;
use std::path::Path;

use crate::canvas::Canvas;

pub mod png;
pub mod ppm;

pub trait ExportCanvas {
    fn save(&self, canvas: &Canvas, path: &Path) -> Result<()>;
}
