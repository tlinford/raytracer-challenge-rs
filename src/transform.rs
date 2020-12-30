use crate::matrix::Matrix;

pub fn translation<T: Into<f64> + Copy>(x: T, y: T, z: T) -> Matrix {
    let mut t = Matrix::identity(4, 4);

    t[(0, 3)] = x.into();
    t[(1, 3)] = y.into();
    t[(2, 3)] = z.into();

    t
}

pub fn scaling<T: Into<f64> + Copy>(x: T, y: T, z: T) -> Matrix {
    let mut s = Matrix::identity(4, 4);

    s[(0, 0)] = x.into();
    s[(1, 1)] = y.into();
    s[(2, 2)] = z.into();

    s
}

pub fn rotation_x(radians: f64) -> Matrix {
    let mut r = Matrix::identity(4, 4);

    r[(1, 1)] = radians.cos();
    r[(1, 2)] = -radians.sin();
    r[(2, 1)] = radians.sin();
    r[(2, 2)] = radians.cos();

    r
}

pub fn rotation_y(radians: f64) -> Matrix {
    let mut r = Matrix::identity(4, 4);

    r[(0, 0)] = radians.cos();
    r[(0, 2)] = radians.sin();
    r[(2, 0)] = -radians.sin();
    r[(2, 2)] = radians.cos();

    r
}

pub fn rotation_z(radians: f64) -> Matrix {
    let mut r = Matrix::identity(4, 4);

    r[(0, 0)] = radians.cos();
    r[(0, 1)] = -radians.sin();
    r[(1, 0)] = radians.sin();
    r[(1, 1)] = radians.cos();

    r
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{point::Point, vector::Vector};

    use super::*;

    #[test]
    fn translate_point() {
        let transform = translation(5, -3, 2);
        let p = Point::new(-3, 4, 5);
        let expected = Point::new(2, 1, 7);
        assert_eq!(&transform * p, expected);
    }

    #[test]
    fn inverse_translate_point() {
        let transform = translation(5, -3, 2);
        let inv = transform.inverse();
        let p = Point::new(-3, 4, 5);
        let expected = Point::new(-8, 7, 3);
        assert_eq!(&inv * p, expected);
    }

    #[test]
    fn translate_does_not_affect_vector() {
        let transform = translation(5, -3, 2);
        let v = Vector::new(-3, 4, 5);
        assert_eq!(&transform * v, v);
    }

    #[test]
    fn scale_point() {
        let transform = scaling(2, 3, 4);
        let p = Point::new(-4, 6, 8);
        let expected = Point::new(-8, 18, 32);
        assert_eq!(&transform * p, expected);
    }

    #[test]
    fn scale_vector() {
        let transform = scaling(2, 3, 4);
        let v = Vector::new(-4, 6, 8);
        let expected = Vector::new(-8, 18, 32);
        assert_eq!(&transform * v, expected);
    }

    #[test]
    fn inverse_scaling() {
        let transform = scaling(2, 3, 4);
        let inv = transform.inverse();
        let v = Vector::new(-4, 6, 8);
        let expected = Vector::new(-2, 2, 2);
        assert_eq!(&inv * v, expected);
    }

    #[test]
    fn reflection_is_negative_scaling() {
        let transform = scaling(-1, 1, 1);
        let p = Point::new(2, 3, 4);
        let expected = Point::new(-2, 3, 4);
        assert_eq!(&transform * p, expected);
    }

    #[test]
    fn rotate_point_x_axis() {
        let p = Point::new(0, 1, 0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);
        assert_eq!(
            &half_quarter * p,
            Point::new(0.0, 2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0)
        );
        assert_eq!(&full_quarter * p, Point::new(0, 0, 1));
    }

    #[test]
    fn inverse_rotate_point_x_axis() {
        let p = Point::new(0, 1, 0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();
        assert_eq!(
            &inv * p,
            Point::new(0.0, 2.0f64.sqrt() / 2.0, -(2.0f64.sqrt() / 2.0))
        );
    }

    #[test]
    fn rotate_point_y_axis() {
        let p = Point::new(0, 0, 1);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);
        assert_eq!(
            &half_quarter * p,
            Point::new(2.0f64.sqrt() / 2.0, 0.0, 2.0f64.sqrt() / 2.0)
        );
        assert_eq!(&full_quarter * p, Point::new(1, 0, 0));
    }

    #[test]
    fn rotate_point_z_axis() {
        let p = Point::new(0, 1, 0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);
        assert_eq!(
            &half_quarter * p,
            Point::new(-(2.0f64.sqrt() / 2.0), 2.0f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(&full_quarter * p, Point::new(-1, 0, 0));
    }
}
