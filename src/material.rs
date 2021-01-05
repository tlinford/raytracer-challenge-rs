use crate::{
    color::Color,
    light::PointLight,
    point::Point,
    vector::{dot, Vector},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn lighting(
        &self,
        light: &PointLight,
        point: &Point,
        eyev: &Vector,
        normalv: &Vector,
        in_shadow: bool,
    ) -> Color {
        let effective_color = self.color * light.intensity();
        let lightv = (light.position() - *point).normalize();
        let ambient = effective_color * self.ambient;

        if in_shadow {
            return ambient;
        }

        let light_dot_normal = dot(lightv, *normalv);

        let diffuse: Color;
        let specular: Color;

        if light_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(*normalv);
            let reflect_dot_eye = dot(reflectv, *eyev);
            specular = if reflect_dot_eye <= 0.0 {
                Color::black()
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                light.intensity() * self.specular * factor
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use crate::equal;

    use super::*;

    #[test]
    fn create_default_material() {
        let m = Material::default();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert!(equal(m.ambient, 0.1));
        assert!(equal(m.diffuse, 0.9));
        assert!(equal(m.specular, 0.9));
        assert!(equal(m.shininess, 200.0))
    }

    #[test]
    fn lighting_eye_between_eye_surface() {
        let m = Material::default();
        let position = Point::origin();
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_eye_between_eye_surface_eye_offset_45deg() {
        let m = Material::default();
        let position = Point::origin();
        let eyev = Vector::new(0.0, 2.0f64.sqrt() / 2.0, -(2.0f64.sqrt() / 2.0));
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_eye_opposite_surface_light_offset_45deg() {
        let m = Material::default();
        let position = Point::origin();
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, -10), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_eye_in_reflection_vector_path() {
        let m = Material::default();
        let position = Point::origin();
        let eyev = Vector::new(0.0, -(2.0f64.sqrt() / 2.0), -(2.0f64.sqrt() / 2.0));
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, -10), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_eye_behind_surface() {
        let m = Material::default();
        let position = Point::origin();
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, 10), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_surface_in_shadow() {
        let m = Material::default();
        let position = Point::origin();
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let result = m.lighting(&light, &position, &eyev, &normalv, in_shadow);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
