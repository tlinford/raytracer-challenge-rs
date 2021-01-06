use crate::{
    color::Color,
    geometry::shape::Shape,
    light::PointLight,
    pattern::Pattern,
    point::Point,
    vector::{dot, Vector},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pattern: Option<Pattern>,
}

impl Material {
    pub fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
        }
    }

    pub fn lighting(
        &self,
        object: &Shape,
        light: &PointLight,
        point: &Point,
        eyev: &Vector,
        normalv: &Vector,
        in_shadow: bool,
    ) -> Color {
        let color = if let Some(pattern) = &self.pattern {
            pattern.color_at_shape(object, *point)
        } else {
            self.color
        };

        let effective_color = color * light.intensity();
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

    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.pattern = Some(pattern);
    }
}

#[cfg(test)]
mod tests {
    use crate::{equal, geometry::shape::sphere, pattern::stripe_pattern};

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
        let result = m.lighting(&sphere(), &light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_eye_between_eye_surface_eye_offset_45deg() {
        let m = Material::default();
        let position = Point::origin();
        let eyev = Vector::new(0.0, 2.0f64.sqrt() / 2.0, -(2.0f64.sqrt() / 2.0));
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&sphere(), &light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_eye_opposite_surface_light_offset_45deg() {
        let m = Material::default();
        let position = Point::origin();
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, -10), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&sphere(), &light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_eye_in_reflection_vector_path() {
        let m = Material::default();
        let position = Point::origin();
        let eyev = Vector::new(0.0, -(2.0f64.sqrt() / 2.0), -(2.0f64.sqrt() / 2.0));
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 10, -10), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&sphere(), &light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_eye_behind_surface() {
        let m = Material::default();
        let position = Point::origin();
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, 10), Color::new(1.0, 1.0, 1.0));
        let result = m.lighting(&sphere(), &light, &position, &eyev, &normalv, false);
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
        let result = m.lighting(&sphere(), &light, &position, &eyev, &normalv, in_shadow);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_pattern() {
        let mut m = Material::default();
        m.set_pattern(stripe_pattern(Color::white(), Color::black()));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = Vector::new(0, 0, -1);
        let normalv = Vector::new(0, 0, -1);
        let light = PointLight::new(Point::new(0, 0, -10), Color::new(1.0, 1.0, 1.0));
        let c1 = m.lighting(
            &sphere(),
            &light,
            &Point::new(0.9, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
        );
        let c2 = m.lighting(
            &sphere(),
            &light,
            &Point::new(1.1, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
        );
        assert_eq!(c1, Color::white());
        assert_eq!(c2, Color::black());
    }
}
