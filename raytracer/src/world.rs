use crate::{
    color::Color,
    equal,
    geometry::{
        intersection::{hit, intersections, shadow_hit, Computations, Intersection},
        shape::Sphere,
        Shape,
    },
    light::PointLight,
    point::Point,
    ray::Ray,
    transform::scaling,
    vector::dot,
};

pub const MAX_RECURSION_DEPTH: usize = 5;

pub struct World {
    objects: Vec<Box<dyn Shape>>,
    lights: Vec<PointLight>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn intersect<'a, 'b>(&'a self, ray: &'b Ray) -> Vec<Intersection> {
        let xs: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|obj| obj.intersect(ray))
            .collect();
        intersections(&xs)
    }

    pub fn shade_hit(&self, comps: &Computations, remaining: usize) -> Color {
        let surface: Color = self
            .lights
            .iter()
            .map(|light| {
                let shadowed = self.is_shadowed(comps.over_point, light);

                comps.object.material().lighting(
                    comps.object,
                    light,
                    &comps.over_point,
                    &comps.eyev,
                    &comps.normalv,
                    shadowed,
                )
            })
            .sum();

        let reflected = self.reflected_color(comps, remaining);
        let refracted = self.refracted_color(comps, remaining);

        let material = &comps.object.material();
        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = comps.schlick();
            surface + reflected * reflectance + refracted * (1.0 - reflectance)
        } else {
            surface + reflected + refracted
        }
    }

    pub fn color_at(&self, ray: &Ray, remaining: usize) -> Color {
        let xs = self.intersect(ray);
        let hit = hit(&xs);

        match hit {
            None => Color::black(),
            Some(hit) => {
                let comps = hit.prepare_computations(ray, &xs);
                self.shade_hit(&comps, remaining)
            }
        }
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    pub fn add_object<T: 'static + Shape>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn add_boxed_object(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object);
    }

    pub fn is_shadowed(&self, point: Point, light: &PointLight) -> bool {
        let v = light.position() - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point, direction);
        let intersections = self.intersect(&r);
        let h = shadow_hit(&intersections);

        h.is_some() && h.unwrap().t() < distance
    }

    pub fn reflected_color(&self, comps: &Computations, remaining: usize) -> Color {
        if equal(comps.object.material().reflective, 0.0) || remaining == 0 {
            return Color::black();
        }
        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(&reflect_ray, remaining - 1);
        color * comps.object.material().reflective
    }

    pub fn refracted_color(&self, comps: &Computations, remaining: usize) -> Color {
        if equal(comps.object.material().transparency, 0.0) || remaining == 0 {
            return Color::black();
        }

        let n_ratio = comps.n1 / comps.n2;
        let cos_i = dot(comps.eyev, comps.normalv);
        let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);

        if sin2_t > 1.0 {
            return Color::black();
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
        let refract_ray = Ray::new(comps.under_point, direction);

        self.color_at(&refract_ray, remaining - 1) * comps.object.material().transparency
    }
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Point::new(-10, 10, -10), Color::new(1.0, 1.0, 1.0));
        let mut s1 = Sphere::default();
        s1.get_base_mut().material.color = Color::new(0.8, 1.0, 0.6);
        s1.get_base_mut().material.diffuse = 0.7;
        s1.get_base_mut().material.specular = 0.2;
        let mut s2 = Sphere::default();
        s2.set_transform(scaling(0.5, 0.5, 0.5));
        Self {
            objects: vec![Box::new(s1), Box::new(s2)],
            lights: vec![light],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        equal, geometry::shape::Plane, pattern::test_pattern, transform::translation,
        vector::Vector,
    };

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
        let mut s1 = Sphere::default();
        s1.get_base_mut().material.color = Color::new(0.8, 1.0, 0.6);
        s1.get_base_mut().material.diffuse = 0.7;
        s1.get_base_mut().material.specular = 0.2;
        let mut s2 = Sphere::default();
        s2.set_transform(scaling(0.5, 0.5, 0.5));

        let w = World::default();
        assert!(w.lights.contains(&light));
        assert!(w.objects.contains(&(Box::new(s1) as Box<dyn Shape>)));
        assert!(w.objects.contains(&(Box::new(s2) as Box<dyn Shape>)));
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
        let i = Intersection::new(4.0, shape.as_ref());
        let comps = i.prepare_computations(&r, &[i]);
        let c = w.shade_hit(&comps, MAX_RECURSION_DEPTH);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shade_intersection_inside() {
        let mut w = World::default();
        w.lights[0] = PointLight::new(Point::new(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        let r = Ray::new(Point::new(0, 0, 0), Vector::new(0, 0, 1));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape.as_ref());
        let comps = i.prepare_computations(&r, &[i]);
        let c = w.shade_hit(&comps, MAX_RECURSION_DEPTH);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_ray_miss() {
        let w = World::default();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 1, 0));
        let c = w.color_at(&r, MAX_RECURSION_DEPTH);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn color_ray_hit() {
        let w = World::default();
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let c = w.color_at(&r, MAX_RECURSION_DEPTH);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    // fn color_intersection_behind_ray() {
    //     let mut w = World::default();
    //     let outer = w.objects[0].as_ref();
    //     outer.get_base_mut().material.ambient = 1.0;
    //     let inner = &mut w.objects[1];
    //     inner.get_base_mut().material.ambient = 1.0;
    //     let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0, 0, -1));
    //     let c = w.color_at(&r, MAX_RECURSION_DEPTH);
    //     let inner = &w.objects[1];
    //     assert_eq!(c, inner.get_base().material.color);
    // }
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
        let s1 = Sphere::default();
        w.add_object(s1);
        let mut s2 = Sphere::default();
        s2.set_transform(translation(0, 0, 10));
        w.add_object(s2);
        let r = Ray::new(Point::new(0, 0, 5), Vector::new(0, 0, 1));
        let i = Intersection::new(4.0, w.objects[1].as_ref());
        let comps = i.prepare_computations(&r, &[i]);
        let c = w.shade_hit(&comps, MAX_RECURSION_DEPTH);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn reflected_color_of_non_reflective_surface() {
        let mut w = World::default();
        let r = Ray::new(Point::origin(), Vector::new(0, 0, 1));
        let mut shape = Sphere::default();
        shape.set_transform(scaling(0.5, 0.5, 0.5));
        shape.get_base_mut().material.ambient = 1.0;
        w.objects[1] = Box::new(shape);

        let shape = &w.objects[1];
        let i = Intersection::new(1.0, shape.as_ref());
        let comps = i.prepare_computations(&r, &[i]);
        let color = w.reflected_color(&comps, MAX_RECURSION_DEPTH);
        assert_eq!(color, Color::black());
    }

    #[test]
    fn reflected_color_of_reflective_surface() {
        let mut w = World::default();
        let mut shape = Plane::default();
        shape.get_base_mut().material.reflective = 0.5;
        shape.set_transform(translation(0, -1, 0));
        w.add_object(shape);
        let shape = &w.objects[2];
        println!("{:?}", shape);
        let r = Ray::new(
            Point::new(0, 0, -3),
            Vector::new(0.0, -(2.0f64.sqrt() / 2.0), 2.0f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0f64.sqrt(), shape.as_ref());
        let comps = i.prepare_computations(&r, &[i]);
        let color = w.reflected_color(&comps, MAX_RECURSION_DEPTH);
        assert_eq!(color, Color::new(0.19033, 0.23791, 0.14274));
    }

    #[test]
    fn shade_hit_with_reflective_surface() {
        let mut w = World::default();
        let mut shape = Plane::default();
        shape.get_base_mut().material.reflective = 0.5;
        shape.set_transform(translation(0, -1, 0));
        w.add_object(shape);
        let shape = &w.objects[2];
        println!("{:?}", shape);
        let r = Ray::new(
            Point::new(0, 0, -3),
            Vector::new(0.0, -(2.0f64.sqrt() / 2.0), 2.0f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0f64.sqrt(), shape.as_ref());
        let comps = i.prepare_computations(&r, &[i]);
        let color = w.shade_hit(&comps, MAX_RECURSION_DEPTH);
        assert_eq!(color, Color::new(0.87676, 0.92435, 0.82918));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();
        w.add_light(PointLight::new(Point::origin(), Color::white()));

        let mut lower = Plane::default();
        lower.get_base_mut().material.reflective = 1.0;
        lower.set_transform(translation(0, -1, 0));
        w.add_object(lower);

        let mut upper = Plane::default();
        upper.get_base_mut().material.reflective = 1.0;
        upper.set_transform(translation(0, 1, 0));
        w.add_object(upper);

        let r = Ray::new(Point::origin(), Vector::new(0, 1, 0));
        w.color_at(&r, MAX_RECURSION_DEPTH);
    }

    #[test]
    fn reflected_color_maximum_recursive_depth() {
        let mut w = World::default();
        let mut shape = Plane::default();
        shape.get_base_mut().material.reflective = 0.5;
        shape.set_transform(translation(0, -1, 0));
        w.add_object(shape);
        let r = Ray::new(
            Point::new(0, 0, -3),
            Vector::new(0.0, -(2.0f64.sqrt() / 2.0), 2.0f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0f64.sqrt(), w.objects[0].as_ref());
        let comps = i.prepare_computations(&r, &[i]);
        let color = w.reflected_color(&comps, 0);
        assert_eq!(color, Color::black());
    }

    #[test]
    fn refracted_color_opaque_surface() {
        let w = World::default();
        let shape = &w.objects[0];
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let xs = intersections(&[
            Intersection::new(4.0, shape.as_ref()),
            Intersection::new(6.0, shape.as_ref()),
        ]);
        let comps = xs[0].prepare_computations(&r, &xs);
        let c = w.refracted_color(&comps, MAX_RECURSION_DEPTH);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn refracted_color_at_max_recursive_depth() {
        let mut w = World::default();
        let shape = &mut w.objects[0];
        shape.get_base_mut().material.transparency = 1.0;
        shape.get_base_mut().material.refractive_index = 1.5;

        let r = Ray::new(Point::new(0, 0, 5), Vector::new(0, 0, 1));

        let shape = &w.objects[0];
        let xs = intersections(&[
            Intersection::new(4.0, shape.as_ref()),
            Intersection::new(6.0, shape.as_ref()),
        ]);
        let comps = xs[0].prepare_computations(&r, &xs);
        let c = w.refracted_color(&comps, 0);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let mut w = World::default();
        let shape = &mut w.objects[0];
        shape.get_base_mut().material.transparency = 1.0;
        shape.get_base_mut().material.refractive_index = 1.5;

        let r = Ray::new(
            Point::new(0.0, 0.0, 2.0f64.sqrt() / 2.0),
            Vector::new(0, 1, 0),
        );

        let shape = &w.objects[0];
        let xs = intersections(&[
            Intersection::new(-(2.0f64.sqrt() / 2.0), shape.as_ref()),
            Intersection::new(2.0f64.sqrt() / 2.0, shape.as_ref()),
        ]);
        let comps = xs[1].prepare_computations(&r, &xs);
        let c = w.refracted_color(&comps, MAX_RECURSION_DEPTH);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn refracted_color_with_refracted_ray() {
        let mut world = World::default();
        let a = &mut world.objects[0];
        a.get_base_mut().material.ambient = 1.0;
        a.get_base_mut().material.set_pattern(test_pattern());

        let b = &mut world.objects[1];
        b.get_base_mut().material.transparency = 1.0;
        b.get_base_mut().material.refractive_index = 1.5;

        let r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0, 1, 0));
        let a = &world.objects[0];
        let b = &world.objects[1];
        let xs = intersections(&[
            Intersection::new(-0.9899, a.as_ref()),
            Intersection::new(-0.4899, b.as_ref()),
            Intersection::new(0.4899, b.as_ref()),
            Intersection::new(0.9899, a.as_ref()),
        ]);
        let comps = xs[2].prepare_computations(&r, &xs);
        let c = world.refracted_color(&comps, MAX_RECURSION_DEPTH);
        assert_eq!(c, Color::new(0.0, 0.99887, 0.04722));
    }

    #[test]
    fn shade_hit_with_transparent_material() {
        let mut w = World::default();

        let mut floor = Plane::default();
        floor.set_transform(translation(0, -1, 0));
        floor.get_base_mut().material.transparency = 0.5;
        floor.get_base_mut().material.refractive_index = 1.5;
        w.add_object(floor);

        let mut ball = Sphere::default();
        ball.get_base_mut().material.color = Color::new(1.0, 0.0, 0.0);
        ball.get_base_mut().material.ambient = 0.5;
        ball.set_transform(translation(0.0, -3.5, -0.5));
        w.add_object(ball);

        let r = Ray::new(
            Point::new(0, 0, -3),
            Vector::new(0.0, -(2.0f64.sqrt() / 2.0), 2.0f64.sqrt() / 2.0),
        );

        let floor = &w.objects[2];
        let xs = intersections(&[Intersection::new(2.0f64.sqrt(), floor.as_ref())]);
        let comps = xs[0].prepare_computations(&r, &xs);
        let color = w.shade_hit(&comps, MAX_RECURSION_DEPTH);
        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn shade_hit_with_reflective_transparent_material() {
        let mut w = World::default();

        let mut floor = Plane::default();
        floor.set_transform(translation(0, -1, 0));
        floor.get_base_mut().material.reflective = 0.5;
        floor.get_base_mut().material.transparency = 0.5;
        floor.get_base_mut().material.refractive_index = 1.5;
        w.add_object(floor);

        let mut ball = Sphere::default();
        ball.get_base_mut().material.color = Color::new(1.0, 0.0, 0.0);
        ball.get_base_mut().material.ambient = 0.5;
        ball.set_transform(translation(0.0, -3.5, -0.5));
        w.add_object(ball);

        let r = Ray::new(
            Point::new(0, 0, -3),
            Vector::new(0.0, -(2.0f64.sqrt() / 2.0), 2.0f64.sqrt() / 2.0),
        );

        let floor = &w.objects[2];
        let xs = intersections(&[Intersection::new(2.0f64.sqrt(), floor.as_ref())]);
        let comps = xs[0].prepare_computations(&r, &xs);
        let color = w.shade_hit(&comps, MAX_RECURSION_DEPTH);
        assert_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
    }
}
