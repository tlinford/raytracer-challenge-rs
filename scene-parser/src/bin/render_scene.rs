use std::{env, path::Path};

use anyhow::Result;
use scene_parser::SceneParser;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("usage: render_scene <scene-file> <output-file>");
    }
    let filename = &args[1];
    let mut parser = SceneParser::new();
    parser.load_file(filename)?;
    let output_filename = &args[2];
    parser.render(&Path::new(output_filename))?;
    Ok(())
}
