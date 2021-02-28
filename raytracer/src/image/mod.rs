use anyhow::Result;
use std::fmt::Debug;
use std::path::Path;

use crate::canvas::Canvas;

pub mod png;
pub mod ppm;

pub trait ExportCanvas: Debug + Send + Sync {
    fn save(&self, canvas: &Canvas, path: &Path) -> Result<()>;
}
