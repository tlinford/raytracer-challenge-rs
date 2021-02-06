use std::{collections::HashMap, fs, vec};

use anyhow::Result;
use error::SceneParserError;
use raytracer::{
    camera::Camera, color::Color, geometry::Shape, light::PointLight, material::Material,
    point::Point, transform::view_transform, vector::Vector,
};
use yaml_rust::{yaml, Yaml, YamlLoader};

mod error;

pub struct SceneParser {
    camera: Option<Camera>,
    lights: Vec<PointLight>,
    materials: HashMap<String, Material>,
    shapes: Vec<Box<dyn Shape>>,
}

impl Default for SceneParser {
    fn default() -> Self {
        Self {
            camera: None,
            lights: vec![],
            materials: HashMap::new(),
            shapes: vec![],
        }
    }
}

impl SceneParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_file(&mut self, path: &str) -> Result<()> {
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
                        "light" => self.lights.push(parse_light(hash)?),
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

    let from = to_point(
        get_required_attribute(camera_el, "from".to_string())?
            .as_vec()
            .ok_or_else(|| SceneParserError::ParseVecError("from".to_string()))?,
    )?;

    let to = to_point(
        get_required_attribute(camera_el, "to".to_string())?
            .as_vec()
            .ok_or_else(|| SceneParserError::ParseVecError("to".to_string()))?,
    )?;

    let up = to_vector(
        get_required_attribute(camera_el, "up".to_string())?
            .as_vec()
            .ok_or_else(|| SceneParserError::ParseVecError("up".to_string()))?,
    )?;

    println!("from: {:?}, to: {:?}, up: {:?}", from, to, up);
    let mut camera = Camera::new(width as usize, height as usize, field_of_view);
    camera.set_transform(view_transform(from, to, up));

    println!("camera: {:?}", camera);
    Ok(camera)
}

fn parse_light(light_el: &yaml::Hash) -> Result<PointLight> {
    let at = to_point(
        get_required_attribute(light_el, "at".to_string())?
            .as_vec()
            .ok_or_else(|| SceneParserError::ParseVecError("from".to_string()))?,
    )?;
    let intensity = to_color(
        get_required_attribute(light_el, "intensity".to_string())?
            .as_vec()
            .ok_or_else(|| SceneParserError::ParseVecError("from".to_string()))?,
    )?;
    let light = PointLight::new(at, intensity);
    println!("light: {:?}", light);
    Ok(light)
}

fn get_required_attribute(hash: &yaml::Hash, key: String) -> Result<&Yaml> {
    Ok(hash
        .get(&Yaml::String(key.clone()))
        .ok_or(SceneParserError::MissingRequiredKey(key))?)
}

fn to_float_vec(v: &[Yaml]) -> Result<Vec<f64>> {
    let res = v
        .iter()
        .map(|f| match f {
            Yaml::Real(_) => f.as_f64(),
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

fn to_point(v: &[Yaml]) -> Result<Point> {
    let numbers = to_float_vec(v)?;
    if numbers.len() != 3 {
        Err(SceneParserError::ParseVecError("from".to_string()).into())
    } else {
        Ok(Point::new(numbers[0], numbers[1], numbers[2]))
    }
}

fn to_vector(v: &[Yaml]) -> Result<Vector> {
    let numbers = to_float_vec(v)?;
    if numbers.len() != 3 {
        Err(SceneParserError::ParseVecError("from".to_string()).into())
    } else {
        Ok(Vector::new(numbers[0], numbers[1], numbers[2]))
    }
}

fn to_color(v: &[Yaml]) -> Result<Color> {
    let numbers = to_float_vec(v)?;
    if numbers.len() != 3 {
        Err(SceneParserError::ParseVecError("from".to_string()).into())
    } else {
        Ok(Color::new(numbers[0], numbers[1], numbers[2]))
    }
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
        assert_eq!(p.lights.len(), 1);
    }
}
