use std::{collections::HashMap, fs, path::Path, vec};

use anyhow::Result;
use error::SceneParserError;
use lazy_static::lazy_static;
use raytracer::{
    camera::Camera,
    color::Color,
    geometry::{
        shape::{Plane, Sphere},
        Shape,
    },
    light::PointLight,
    material::Material,
    matrix::Matrix,
    pattern::{checkers_pattern, stripe_pattern, Pattern},
    point::Point,
    ppm::save_ppm,
    transform::{self, rotation_y, rotation_z, view_transform},
    vector::Vector,
    world::World,
};
use transform::{rotation_x, scaling, translation};
use yaml_rust::{yaml, Yaml, YamlLoader};

mod error;

lazy_static! {
    static ref ADD_KEY: Yaml = Yaml::String(String::from("add"));
    static ref DEFINE_KEY: Yaml = Yaml::String(String::from("define"));
    static ref VALUE_KEY: Yaml = Yaml::String(String::from("value"));
    static ref TRANSFORM_KEY: Yaml = Yaml::String(String::from("transform"));
    static ref MATERIAL_KEY: Yaml = Yaml::String(String::from("material"));
    static ref MATERIAL_COLOR_KEY: Yaml = Yaml::String(String::from("color"));
    static ref MATERIAL_PATTERN_KEY: Yaml = Yaml::String(String::from("pattern"));
    static ref MATERIAL_AMBIENT_KEY: Yaml = Yaml::String(String::from("ambient"));
    static ref MATERIAL_DIFFUSE_KEY: Yaml = Yaml::String(String::from("diffuse"));
    static ref MATERIAL_SPECULAR_KEY: Yaml = Yaml::String(String::from("specular"));
    static ref MATERIAL_SHININESS_KEY: Yaml = Yaml::String(String::from("shininess"));
    static ref MATERIAL_REFLECTIVE_KEY: Yaml = Yaml::String(String::from("reflective"));
    static ref MATERIAL_TRANSPARENCY_KEY: Yaml = Yaml::String(String::from("transparency"));
    static ref MATERIAL_REFRACTIVE_INDEX_KEY: Yaml = Yaml::String(String::from("refractive-index"));
    static ref PATTERN_TYPE_KEY: Yaml = Yaml::String(String::from("type"));
    static ref PATTERN_COLORS_KEY: Yaml = Yaml::String(String::from("colors"));
}

pub struct Scene {
    camera: Option<Camera>,
    lights: Vec<PointLight>,
    materials: HashMap<String, Material>,
    shapes: Vec<Box<dyn Shape>>,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            camera: None,
            lights: vec![],
            materials: HashMap::new(),
            shapes: vec![],
        }
    }
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct SceneParser {
    scene: Scene,
}

impl Default for SceneParser {
    fn default() -> Self {
        Self {
            scene: Scene::new(),
        }
    }
}

impl SceneParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_file(&mut self, path: &str) -> Result<()> {
        println!("path to scene: {:?}", path);
        let contents = fs::read_to_string(path).unwrap();
        let yaml = YamlLoader::load_from_str(&contents)?;
        let elements = &yaml[0];
        if let Yaml::Array(array) = elements {
            let define_elements: Vec<&Yaml> = array
                .iter()
                .filter(|&element| is_define_element(element))
                .collect();
            println!("found {} define elements", define_elements.len());

            for el in define_elements {
                self.parse_define_element(el)?;
            }

            let add_elements: Vec<&Yaml> = array
                .iter()
                .filter(|&element| is_add_element(element))
                .collect();
            println!("found {} add elements", add_elements.len());

            for el in add_elements {
                self.parse_add_element(el)?;
            }
        } else {
            return Err(error::SceneParserError::BadInputFile(String::from(path)).into());
        }
        Ok(())
    }

    fn parse_add_element(&mut self, element: &Yaml) -> Result<()> {
        if let Yaml::Hash(hash) = element {
            if let Some(add) = hash.get(&ADD_KEY) {
                if let Yaml::String(kind) = add {
                    match kind.as_str() {
                        "camera" => self.scene.camera = Some(parse_camera(hash)?),
                        "light" => self.scene.lights.push(parse_light(hash)?),
                        "sphere" | "plane" => self.scene.shapes.push(self.parse_shape(kind, hash)?),
                        _ => println!("unhandled element: {}", kind),
                    }
                    return Ok(());
                }
            }
        }
        Err(error::SceneParserError::InvalidAddElementError.into())
    }

    fn parse_define_element(&mut self, element: &Yaml) -> Result<()> {
        if let Yaml::Hash(hash) = element {
            let name = hash
                .get(&DEFINE_KEY)
                .unwrap()
                .as_str()
                .ok_or(error::SceneParserError::InvalidDefineElementError)?;
            let define_value_el = hash
                .get(&VALUE_KEY)
                .ok_or(error::SceneParserError::InvalidDefineElementError)?;
            match define_value_el {
                Yaml::Array(_) => {
                    println!("found transform");
                }
                Yaml::Hash(_) => {
                    println!("found material");
                    let material = self.parse_material(define_value_el)?;
                    self.scene.materials.insert(String::from(name), material);
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    fn parse_shape(&self, kind: &str, shape_el: &yaml::Hash) -> Result<Box<dyn Shape>> {
        let mut shape: Box<dyn Shape> = match kind {
            "sphere" => Box::new(Sphere::default()),
            "plane" => Box::new(Plane::default()),
            _ => unreachable!(),
        };

        if let Some(transform) = shape_el.get(&TRANSFORM_KEY) {
            let transform = parse_transform(transform)?;
            shape.set_transform(transform);
        }

        if let Some(material) = shape_el.get(&MATERIAL_KEY) {
            let material = self.parse_material(material)?;
            shape.set_material(material);
        }

        println!("shape: {:?}", shape);
        Ok(shape)
    }

    fn parse_material(&self, material_el: &Yaml) -> Result<Material> {
        if let Yaml::String(defined_material) = material_el {
            println!("found defined material: {:?}", defined_material);
            let material = self
                .scene
                .materials
                .get(defined_material)
                .ok_or(error::SceneParserError::ParseMaterialError)?
                .clone();
            Ok(material)
        } else if let Yaml::Hash(material_def) = material_el {
            let mut material = Material::default();
            if let Some(color_el) = material_def.get(&MATERIAL_COLOR_KEY) {
                material.color = to_color(
                    color_el
                        .as_vec()
                        .ok_or(error::SceneParserError::ParseMaterialError)?,
                )?;
            }
            if let Some(pattern_el) = material_def.get(&MATERIAL_PATTERN_KEY) {
                material.set_pattern(parse_pattern(pattern_el)?);
            }
            if let Some(ambient_el) = material_def.get(&MATERIAL_AMBIENT_KEY) {
                material.ambient = to_f64(ambient_el)?;
            }

            if let Some(diffuse_el) = material_def.get(&MATERIAL_DIFFUSE_KEY) {
                material.diffuse = to_f64(diffuse_el)?;
            }

            if let Some(specular_el) = material_def.get(&MATERIAL_SPECULAR_KEY) {
                material.specular = to_f64(specular_el)?;
            }

            if let Some(shininess_el) = material_def.get(&MATERIAL_SHININESS_KEY) {
                material.shininess = to_f64(shininess_el)?;
            }

            if let Some(reflective_el) = material_def.get(&MATERIAL_REFLECTIVE_KEY) {
                material.reflective = to_f64(reflective_el)?;
            }

            if let Some(transparency_el) = material_def.get(&MATERIAL_TRANSPARENCY_KEY) {
                material.transparency = to_f64(transparency_el)?;
            }

            if let Some(refractive_index_el) = material_def.get(&MATERIAL_REFRACTIVE_INDEX_KEY) {
                material.refractive_index = to_f64(refractive_index_el)?;
            }

            println!("material: {:?}", material);
            Ok(material)
        } else {
            Err(error::SceneParserError::ParseMaterialError.into())
        }
    }

    pub fn render(&mut self) {
        let mut world = World::new();
        for light in self.scene.lights.drain(0..) {
            world.add_light(light);
        }
        for shape in self.scene.shapes.drain(0..) {
            world.add_boxed_object(shape);
        }

        let camera = self.scene.camera.as_mut().unwrap();

        let canvas = camera.render(&world);
        save_ppm(&canvas, Path::new("test.ppm"));
    }
}

fn is_add_element(element: &Yaml) -> bool {
    if let Yaml::Hash(hash) = element {
        hash.contains_key(&ADD_KEY)
    } else {
        false
    }
}

fn is_define_element(element: &Yaml) -> bool {
    if let Yaml::Hash(hash) = element {
        hash.contains_key(&DEFINE_KEY)
    } else {
        false
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

fn parse_transform(transform_el: &Yaml) -> Result<Matrix> {
    if let Yaml::Array(transforms) = transform_el {
        let mut transform = Matrix::identity(4, 4);
        for transform_item_el in transforms {
            let transform_item = parse_transform_item(transform_item_el)?;
            transform = &transform_item * &transform;
        }

        Ok(transform)
    } else {
        Err(error::SceneParserError::ParseTransformError.into())
    }
}

fn parse_transform_item(transform_item_el: &Yaml) -> Result<Matrix> {
    if let Yaml::Array(transform) = transform_item_el {
        let kind = transform[0]
            .as_str()
            .ok_or(error::SceneParserError::ParseTransformError)?;
        let args = to_float_vec(&transform[1..])?;
        match kind {
            "scale" => Ok(scaling(args[0], args[1], args[2])),
            "translate" => Ok(translation(args[0], args[1], args[2])),
            "rotate-x" => Ok(rotation_x(args[0])),
            "rotate-y" => Ok(rotation_y(args[0])),
            "rotate-z" => Ok(rotation_z(args[0])),
            _ => Err(error::SceneParserError::ParseTransformError.into()),
        }
    } else {
        Err(error::SceneParserError::ParseTransformError.into())
    }
}

fn parse_pattern(pattern_el: &Yaml) -> Result<Pattern> {
    if let Yaml::Hash(pattern_def) = pattern_el {
        let kind = pattern_def
            .get(&PATTERN_TYPE_KEY)
            .ok_or(error::SceneParserError::ParsePatternError)?
            .as_str()
            .ok_or(error::SceneParserError::ParsePatternError)?;
        let colors_el = pattern_def
            .get(&PATTERN_COLORS_KEY)
            .ok_or_else(|| anyhow::Error::from(error::SceneParserError::ParsePatternError))?;
        let color_defs = colors_el
            .as_vec()
            .ok_or(error::SceneParserError::ParsePatternError)?;

        let colors = color_defs
            .iter()
            .map(|color_def_el| {
                color_def_el
                    .as_vec()
                    .ok_or_else(|| error::SceneParserError::ParsePatternError.into())
            })
            .collect::<Result<Vec<_>>>()?
            .iter()
            .map(|&color_vec| to_color(color_vec))
            .collect::<Result<Vec<_>>>()?;

        let pattern = match kind {
            "stripes" => stripe_pattern(colors[0], colors[1]),
            "checkers" => checkers_pattern(colors[0], colors[1]),
            _ => Pattern::default(),
        };
        Ok(pattern)
    } else {
        Err(error::SceneParserError::ParsePatternError.into())
    }
}

fn get_required_attribute(hash: &yaml::Hash, key: String) -> Result<&Yaml> {
    Ok(hash
        .get(&Yaml::String(key.clone()))
        .ok_or(SceneParserError::MissingRequiredKey(key))?)
}

fn to_f64(f: &Yaml) -> Result<f64> {
    match f {
        Yaml::Real(_) => f
            .as_f64()
            .ok_or_else(|| error::SceneParserError::ParseFloatError(String::from("f")).into()),
        Yaml::Integer(i) => Ok(*i as f64),
        // Yaml::Integer(i) => Ok(i as f64),
        _ => Err(error::SceneParserError::ParseFloatError(String::from("f")).into()),
    }
}

fn to_float_vec(v: &[Yaml]) -> Result<Vec<f64>> {
    let res = v.iter().map(to_f64).collect::<Result<Vec<_>>>();
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
        assert!(p.scene.camera.is_some());
        assert_eq!(p.scene.lights.len(), 1);
        assert_eq!(p.scene.shapes.len(), 13);
        assert_eq!(p.scene.materials.len(), 1);

        p.render();
    }

    #[test]
    fn test_is_add_element() {
        let add_element = &YamlLoader::load_from_str("add: plane").unwrap()[0];
        let define_element = &YamlLoader::load_from_str("define: some-material").unwrap()[0];
        assert!(is_add_element(add_element));
        assert!(!is_add_element(define_element));
    }

    #[test]
    fn test_is_define_element() {
        let add_element = &YamlLoader::load_from_str("add: plane").unwrap()[0];
        let define_element = &YamlLoader::load_from_str("define: some-material").unwrap()[0];
        assert!(is_define_element(define_element));
        assert!(!is_define_element(add_element));
    }
}
