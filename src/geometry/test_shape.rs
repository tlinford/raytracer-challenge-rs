use std::cell::RefCell;

use crate::{point::Point, ray::Ray, vector::Vector};

use super::shape::Shape;

#[derive(Debug)]
pub struct TestShape {
    saved_ray: RefCell<Ray>,
}

impl Default for TestShape {
    fn default() -> Self {
        Self {
            saved_ray: RefCell::new(Ray::new(Point::origin(), Vector::new(0, 0, 0))),
        }
    }
}

impl TestShape {
    pub fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        self.saved_ray
            .replace(Ray::new(ray.origin(), ray.direction()));
        vec![]
    }

    pub fn local_normal_at(&self, point: Point) -> Vector {
        Vector::new(point.x, point.y, point.z)
    }
}

impl PartialEq for TestShape {
    fn eq(&self, rhs: &TestShape) -> bool {
        *self.saved_ray.borrow() == *rhs.saved_ray.borrow()
    }
}

fn _test_shape() -> Shape {
    Shape::default()
}

#[cfg(test)]
mod tests {
    use std::{f32::consts::FRAC_1_SQRT_2, f64::consts::PI};

    use crate::{
        geometry::shape::Kind,
        material::Material,
        matrix::Matrix,
        transform::{rotation_y, scaling, translation},
    };

    use super::*;

    #[test]
    fn default_transformation() {
        let s = _test_shape();
        assert_eq!(s.transform(), &Matrix::identity(4, 4));
    }

    #[test]
    fn assign_transformation() {
        let mut s = _test_shape();
        let t = translation(2, 3, 4);
        s.set_transform(&t);
        assert_eq!(s.transform(), &t);
    }

    #[test]
    fn default_material() {
        let s = _test_shape();
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn assign_material() {
        let mut s = _test_shape();
        let mut m = Material::default();
        m.ambient = 1.0;
        s.material = m;

        let mut m = Material::default();
        m.ambient = 1.0;
        assert_eq!(s.material, m);
    }

    #[test]
    fn intersect_scaled_shape_with_ray() {
        let r = Ray::new(Point::new(0, 0, -5), Vector::new(0, 0, 1));
        let mut s = _test_shape();
        s.set_transform(&scaling(2, 2, 2));

        s.intersect(&r);

        if let Kind::TestShape(test_shape) = s.shape {
            let r = test_shape.saved_ray.borrow();
            assert_eq!(r.origin(), Point::new(0.0, 0.0, -2.5));
            assert_eq!(r.direction(), Vector::new(0.0, 0.0, 0.5));
        } else {
            unreachable!();
        }
    }

    #[test]
    fn normal_translated_shape() {
        let mut s = _test_shape();
        s.set_transform(&translation(0, 1, 0));
        let n = s.normal_at(Point::new(0.0, 1.70711, -FRAC_1_SQRT_2));
        assert_eq!(n, Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn normal_transformed_shape() {
        let mut s = _test_shape();
        let m = &scaling(1.0, 0.5, 1.0) * &rotation_y(PI / 5.0);
        s.set_transform(&m);
        let n = s.normal_at(Point::new(0.0, 2.0f64.sqrt() / 2.0, -(2.0f64.sqrt() / 2.0)));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }
}
