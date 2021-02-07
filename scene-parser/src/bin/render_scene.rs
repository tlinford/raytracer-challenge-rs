use std::env;

use anyhow::Result;
use scene_parser::SceneParser;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("filename not specified");
    }
    let filename = &args[1];
    let mut parser = SceneParser::new();
    parser.load_file(filename)?;
    parser.render();
    Ok(())
}
