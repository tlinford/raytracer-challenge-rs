use crate::{
    color::Color,
    geometry::{
        intersection::{hit, intersections, Computations, Intersection},
        shape::sphere,
        shape::Shape,
    },
    light::PointLight,
    point::Point,
    ray::Ray,
    transform::scaling,
};

pub struct World {
    objects: Vec<Shape>,
    lights: Vec<PointLight>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let xs: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|obj| obj.intersect(ray))
            .collect();
        intersections(&xs)
    }

    pub fn shade_hit(&self, comps: &Computations) -> Color {
        self.lights
            .iter()
            .map(|light| {
                let shadowed = self.is_shadowed(comps.over_point, light);

                comps.object.material.lighting(
                    light,
                    &comps.point,
                    &comps.eyev,
                    &comps.normalv,
                    shadowed,
                )
            })
            .sum()
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersect(ray);
        let hit = hit(&xs);

        match hit {
            None => Color::black(),
            Some(hit) => {
                let comps = hit.prepare_computations(ray);
                self.shade_hit(&comps)
            }
        }
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    pub fn add_object(&mut self, object: Shape) {
        self.objects.push(object);
    }

    pub fn is_shadowed(&self, point: Point, light: &PointLight) -> bool {
        let v = light.position() - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point, direction);
        let intersections = self.intersect(&r);
        let h = hit(&intersections);

        h.is_some() && h.unwrap().t() < distance
    }
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Point::new(-10, 10, -10), Color::new(1.0, 1.0, 1.0));
        let mut s1 = sphere();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = sphere();
        s2.set_transform(&scaling(0.5, 0.5, 0.5));

        Self {
            objects: vec![s1, s2],
            lights: vec![light],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{equal, transform::translation, vector::Vector};

    use super::*;

    #[test]
    fn create_world() {
        let w = World::new();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    fn create_default_world() {
        let light = PointLight::new(Point::new(-10, 10, -10), Color::new(1.0, 1.0, 1.0));
        let mut s1 = sphere();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = sphere();
        s2.set_transform(&scaling(0.5, 0.5, 0.5));

        let w = World::default();
        assert!(w.lights.contains(&light));
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let xs = w.intersect(&r);
        assert_eq!(xs.len(), 4);
        assert!(equal(xs[0].t(), 4.0));
        assert!(equal(xs[1].t(), 4.5));
        assert!(equal(xs[2].t(), 5.5));
        assert!(equal(xs[3].t(), 6.0));
    }

    #[test]
    fn shade_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shade_intersection_inside() {
        let mut w = World::default();
        w.lights[0] = PointLight::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_ray_miss() {
        let w = World::default();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 1, 0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn color_ray_hit() {
        let w = World::default();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_intersection_behind_ray() {
        let mut w = World::default();
        let outer = &mut w.objects[0];
        outer.material.ambient = 1.0;
        let inner = &mut w.objects[1];
        inner.material.ambient = 1.0;
        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0, 0, -1));
        let c = w.color_at(&r);
        let inner = &w.objects[1];
        assert_eq!(c, inner.material.color);
    }

    #[test]
    fn no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Point::new(0, 10, 0);
        assert_eq!(w.is_shadowed(p, &w.lights[0]), false);
    }

    #[test]
    fn shadow_when_object_is_between_point_and_light() {
        let w = World::default();
        let p = Point::new(10, -10, 10);
        assert_eq!(w.is_shadowed(p, &w.lights[0]), true);
    }

    #[test]
    fn no_shadow_when_object_is_behind_light() {
        let w = World::default();
        let p = Point::new(-20, 20, -20);
        assert_eq!(w.is_shadowed(p, &w.lights[0]), false);
    }

    #[test]
    fn no_shadow_when_object_is_behind_point() {
        let w = World::default();
        let p = Point::new(-2, 2, -2);
        assert_eq!(w.is_shadowed(p, &w.lights[0]), false);
    }

    #[test]
    fn shade_hit_with_intersection_in_shadow() {
        let mut w = World::new();
        w.add_light(PointLight::new(
            Point::new(0, 0, -10),
            Color::new(1.0, 1.0, 1.0),
        ));
        let s1 = sphere();
        w.add_object(s1);
        let mut s2 = sphere();
        s2.set_transform(&translation(0, 0, 10));
        w.add_object(s2);
        let r = Ray::new(Point::new(0, 0, 5), Vector::new(0, 0, 1));
        let i = Intersection::new(4.0, &w.objects[1]);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
