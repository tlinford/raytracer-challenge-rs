use std::{any::Any, cell::RefCell};

use crate::{
    geometry::{intersection::Intersection, BaseShape, Shape},
    point::Point,
    ray::Ray,
    vector::Vector,
};

#[derive(Debug, PartialEq)]
struct TestShape {
    base: BaseShape,
    saved_ray: RefCell<Ray>,
}

impl Default for TestShape {
    fn default() -> Self {
        Self {
            base: BaseShape::default(),
            saved_ray: RefCell::new(Ray::new(Point::origin(), Vector::new(0, 0, 0))),
        }
    }
}

impl Shape for TestShape {
    fn get_base(&self) -> &BaseShape {
        &self.base
    }

    fn get_base_mut(&mut self) -> &mut BaseShape {
        &mut self.base
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        self.saved_ray
            .replace(Ray::new(ray.origin(), ray.direction()));
        vec![]
    }

    fn local_normal_at(&self, point: Point, _intersection: &Intersection) -> Vector {
        Vector::new(point.x, point.y, point.z)
    }
}

#[cfg(test)]
mod tests {
    use std::{f32::consts::FRAC_1_SQRT_2, f64::consts::PI};

    use crate::{
        material::Material,
        matrix::Matrix,
        transform::{rotation_y, scaling, translation},
    };

    use super::*;

    #[test]
    fn default_transformation() {
        let s = TestShape::default();
        assert_eq!(s.transform(), &Matrix::identity(4, 4));
    }

    #[test]
    fn assign_transformation() {
        let mut s = TestShape::default();
        let t = translation(2, 3, 4);
        s.set_transform(t.clone());
        assert_eq!(s.transform(), &t);
    }

    #[test]
    fn default_material() {
        let s = TestShape::default();
        assert_eq!(s.material(), &Material::default());
    }

    #[test]
    fn assign_material() {
        let mut s = TestShape::default();
        let mut m = Material::default();
        m.ambient = 1.0;
        s.set_material(m);

        let mut m = Material::default();
        m.ambient = 1.0;
        assert_eq!(s.material(), &m);
    }

    #[test]
    fn intersect_scaled_shape_with_ray() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut s = TestShape::default();
        s.set_transform(scaling(2, 2, 2));

        s.intersect(&r);
        let r = s.saved_ray.borrow();
        assert_eq!(r.origin(), Point::new(0.0, 0.0, -2.5));
        assert_eq!(r.direction(), Vector::new(0.0, 0.0, 0.5));
    }

    #[test]
    fn intersect_translated_shape_with_ray() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut s = TestShape::default();
        s.set_transform(translation(5, 0, 0));

        s.intersect(&r);
        let r = s.saved_ray.borrow();
        assert_eq!(r.origin(), Point::new(-5, 0, -5));
        assert_eq!(r.direction(), Vector::new(0, 0, 1));
    }

    #[test]
    fn normal_translated_shape() {
        let mut s = TestShape::default();
        s.set_transform(translation(0, 1, 0));
        let n = s.normal_at(
            Point::new(0.0, 1.70711, -FRAC_1_SQRT_2),
            &Intersection::new(-100.0, &s),
        );
        assert_eq!(n, Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn normal_transformed_shape() {
        let mut s = TestShape::default();
        s.set_transform(&scaling(1.0, 0.5, 1.0) * &rotation_y(PI / 5.0));
        let n = s.normal_at(
            Point::new(0.0, 2.0f64.sqrt() / 2.0, -(2.0f64.sqrt() / 2.0)),
            &Intersection::new(-100.0, &s),
        );
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
