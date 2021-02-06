use std::{collections::HashMap, fs, vec};

use anyhow::Result;
use error::SceneParserError;
use raytracer::{
    camera::Camera, geometry::Shape, material::Material, point::Point, transform::view_transform,
    vector::Vector,
};
use yaml_rust::{yaml, Yaml, YamlLoader};

mod error;

struct SceneParser {
    camera: Option<Camera>,
    materials: HashMap<String, Material>,
    shapes: Vec<Box<dyn Shape>>,
}

enum Add {
    Material,
    Shape,
    Light,
}

enum Define {
    Material,
    Transform,
}

impl SceneParser {
    fn new() -> Self {
        Self {
            camera: None,
            materials: HashMap::new(),
            shapes: vec![],
        }
    }

    fn load_file(&mut self, path: &str) -> Result<()> {
        let contents = fs::read_to_string(path).unwrap();
        let yaml = YamlLoader::load_from_str(&contents)?;
        let elements = &yaml[0];
        if let Yaml::Array(array) = elements {
            for el in array {
                self.parse_element(el)?;
            }
        }
        Ok(())
    }

    fn parse_element(&mut self, element: &Yaml) -> Result<()> {
        let add_key = Yaml::String("add".to_string());
        if let Yaml::Hash(hash) = element {
            if let Some(add) = hash.get(&add_key) {
                if let Yaml::String(kind) = add {
                    println!("found {} to add", kind);
                    match kind.as_str() {
                        "camera" => self.camera = Some(parse_camera(hash)?),
                        _ => println!("unhandled element: {}", kind),
                    }
                }
            }
        }
        Ok(())
    }
}

fn parse_camera(camera_el: &yaml::Hash) -> Result<Camera> {
    println!("{:?}", camera_el);
    let width = get_required_attribute(camera_el, "width".to_string())?
        .as_i64()
        .ok_or_else(|| SceneParserError::ParseIntError("width".to_string()))?;

    let height = get_required_attribute(camera_el, "height".to_string())?
        .as_i64()
        .ok_or_else(|| SceneParserError::ParseIntError("height".to_string()))?;

    let field_of_view = get_required_attribute(camera_el, "field-of-view".to_string())?
        .as_f64()
        .ok_or_else(|| SceneParserError::ParseFloatError("field-of-view".to_string()))?;

    let from_vec = to_float_vec(
        get_required_attribute(camera_el, "from".to_string())?
            .as_vec()
            .ok_or_else(|| SceneParserError::ParseVecError("from".to_string()))?,
    )?;
    if from_vec.len() != 3 {
        return Err(SceneParserError::ParseVecError("from".to_string()).into());
    }
    let to_vec = to_float_vec(
        get_required_attribute(camera_el, "to".to_string())?
            .as_vec()
            .ok_or_else(|| SceneParserError::ParseVecError("to".to_string()))?,
    )?;
    if to_vec.len() != 3 {
        return Err(SceneParserError::ParseVecError("from".to_string()).into());
    }
    let up_vec = to_float_vec(
        get_required_attribute(camera_el, "up".to_string())?
            .as_vec()
            .ok_or_else(|| SceneParserError::ParseVecError("up".to_string()))?,
    )?;
    if up_vec.len() != 3 {
        return Err(SceneParserError::ParseVecError("from".to_string()).into());
    }

    let from = Point::new(from_vec[0], from_vec[1], from_vec[2]);
    let to = Point::new(to_vec[0], to_vec[1], to_vec[2]);
    let up = Vector::new(up_vec[0], up_vec[1], up_vec[2]);

    println!("from: {:?}, to: {:?}, up: {:?}", from, to, up);
    let mut camera = Camera::new(width as usize, height as usize, field_of_view);
    camera.set_transform(view_transform(from, to, up));

    println!("camera: {:?}", camera);
    Ok(camera)
}

fn get_required_attribute(hash: &yaml::Hash, key: String) -> Result<&Yaml> {
    Ok(hash
        .get(&Yaml::String(key.clone()))
        .ok_or(SceneParserError::MissingRequiredKey(key))?)
}

fn to_float_vec(v: &Vec<Yaml>) -> Result<Vec<f64>> {
    let res = v
        .iter()
        .map(|f| match f {
            Yaml::Real(r) => f.as_f64(),
            Yaml::Integer(i) => Some(*i as f64),
            _ => None,
        })
        .map(|f| {
            f.ok_or_else(|| {
                anyhow::Error::from(SceneParserError::ParseFloatError(String::from("f")))
            })
        })
        .collect::<Result<Vec<_>>>();
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_file() {
        let file = "./examples/reflect-refract.yml";
        let mut p = SceneParser::new();
        let res = p.load_file(file);
        println!("res: {:?}", res);
        assert!(res.is_ok());
        assert!(p.camera.is_some());
    }
}
