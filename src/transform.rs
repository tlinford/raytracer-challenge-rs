#[cfg(test)]
mod tests {
    use crate::{matrix::Matrix, point::Point, vector::Vector};

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
}
