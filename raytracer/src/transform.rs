use crate::{
    matrix::Matrix,
    point::Point,
    vector::{cross, Vector},
};

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

pub fn shearing<T: Into<f64> + Copy>(xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Matrix {
    let mut s = Matrix::identity(4, 4);

    s[(0, 1)] = xy.into();
    s[(0, 2)] = xz.into();
    s[(1, 0)] = yx.into();
    s[(1, 2)] = yz.into();
    s[(2, 0)] = zx.into();
    s[(2, 1)] = zy.into();

    s
}

pub fn view_transform(from: Point, to: Point, up: Vector) -> Matrix {
    let forward = (to - from).normalize();
    let upn = up.normalize();
    let left = cross(forward, upn);
    let true_up = cross(left, forward);
    let orientation = Matrix::from_rows(
        4,
        4,
        &[
            &[left.x, left.y, left.z, 0.0],
            &[true_up.x, true_up.y, true_up.z, 0.0],
            &[-forward.x, -forward.y, -forward.z, 0.0],
            &[0.0, 0.0, 0.0, 1.0],
        ],
    );

    &orientation * &translation(-from.x, -from.y, -from.z)
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

    #[test]
    fn shearing_xy() {
        let transform = shearing(1, 0, 0, 0, 0, 0);
        let p = Point::new(2, 3, 4);
        let expected = Point::new(5, 3, 4);
        assert_eq!(&transform * p, expected);
    }

    #[test]
    fn shearing_xz() {
        let transform = shearing(0, 1, 0, 0, 0, 0);
        let p = Point::new(2, 3, 4);
        let expected = Point::new(6, 3, 4);
        assert_eq!(&transform * p, expected);
    }

    #[test]
    fn shearing_yx() {
        let transform = shearing(0, 0, 1, 0, 0, 0);
        let p = Point::new(2, 3, 4);
        let expected = Point::new(2, 5, 4);
        assert_eq!(&transform * p, expected);
    }

    #[test]
    fn shearing_yz() {
        let transform = shearing(0, 0, 0, 1, 0, 0);
        let p = Point::new(2, 3, 4);
        let expected = Point::new(2, 7, 4);
        assert_eq!(&transform * p, expected);
    }

    #[test]
    fn shearing_zx() {
        let transform = shearing(0, 0, 0, 0, 1, 0);
        let p = Point::new(2, 3, 4);
        let expected = Point::new(2, 3, 6);
        assert_eq!(&transform * p, expected);
    }

    #[test]
    fn shearing_zy() {
        let transform = shearing(0, 0, 0, 0, 0, 1);
        let p = Point::new(2, 3, 4);
        let expected = Point::new(2, 3, 7);
        assert_eq!(&transform * p, expected);
    }

    #[test]
    fn indvidual_transformations_sequence() {
        let p = Point::new(1, 0, 1);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5, 5, 5);
        let c = translation(10, 5, 7);

        let p2 = &a * p;
        assert_eq!(p2, Point::new(1, -1, 0));

        let p3 = &b * p2;
        assert_eq!(p3, Point::new(5, -5, 0));

        let p4 = &c * p3;
        assert_eq!(p4, Point::new(15, 0, 7));
    }

    #[test]
    #[allow(clippy::clippy::many_single_char_names)]
    fn chained_transformations_reverse_order() {
        let p = Point::new(1, 0, 1);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5, 5, 5);
        let c = translation(10, 5, 7);

        let t = &(&c * &b) * &a;
        assert_eq!(&t * p, Point::new(15, 0, 7));
    }

    #[test]
    fn transformation_matrix_default_orientation() {
        let from = Point::new(0, 0, 0);
        let to = Point::new(0, 0, -1);
        let up = Vector::new(0, 1, 0);
        let t = view_transform(from, to, up);
        assert_eq!(t, Matrix::identity(4, 4));
    }

    #[test]
    fn view_transformation_positive_z() {
        let from = Point::new(0, 0, 0);
        let to = Point::new(0, 0, 1);
        let up = Vector::new(0, 1, 0);
        let t = view_transform(from, to, up);
        assert_eq!(t, scaling(-1, 1, -1));
    }

    #[test]
    fn view_transformation_moves_world() {
        let from = Point::new(0, 0, 8);
        let to = Point::new(0, 0, 0);
        let up = Vector::new(0, 1, 0);
        let t = view_transform(from, to, up);
        assert_eq!(t, translation(0, 0, -8));
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = Point::new(1, 3, 2);
        let to = Point::new(4, -2, 8);
        let up = Vector::new(1, 1, 0);
        let t = view_transform(from, to, up);
        let expected = Matrix::from_rows(
            4,
            4,
            &[
                &[-0.50709, 0.50709, 0.67612, -2.36643],
                &[0.76772, 0.60609, 0.12122, -2.82843],
                &[-0.35857, 0.59761, -0.71714, 0.0],
                &[0.0, 0.0, 0.0, 1.0],
            ],
        );
        assert_eq!(t, expected);
    }
}
