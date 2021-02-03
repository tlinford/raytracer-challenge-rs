use std::{
    ops::{Index, IndexMut, Mul},
    vec,
};

use crate::{
    point::Point,
    transform::{rotation_x, rotation_y, rotation_z, scaling, shearing, translation},
    vector::Vector,
};

#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    columns: usize,
    elements: Vec<f64>,
}

impl Matrix {
    pub fn zero(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            elements: vec![0.0; rows * columns],
        }
    }

    pub fn identity(rows: usize, columns: usize) -> Self {
        let mut id = Self::zero(rows, columns);

        for i in 0..columns {
            id[(i, i)] = 1.0;
        }

        id
    }

    pub fn from_slice<T: Into<f64> + Copy>(rows: usize, columns: usize, slice: &[T]) -> Self {
        assert_eq!(rows * columns, slice.len());
        Self {
            rows,
            columns,
            elements: slice.iter().map(|&n| n.into()).collect(),
        }
    }

    pub fn from_rows<T: Into<f64> + Copy>(
        rows: usize,
        columns: usize,
        row_slices: &[&[T]],
    ) -> Self {
        assert_eq!(row_slices.len(), rows);

        let mut elements = Vec::new();
        row_slices.iter().for_each(|s| {
            assert!(s.len() == columns);
            s.iter().map(|&n| n.into()).for_each(|f| elements.push(f));
        });

        Self {
            rows,
            columns,
            elements,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    fn idx(&self, i: usize, j: usize) -> usize {
        i * self.columns + j
    }

    pub fn transpose(&self) -> Self {
        let mut t = Matrix::zero(self.columns, self.rows);

        for i in 0..self.rows {
            for j in 0..self.columns {
                t[(j, i)] = self[(i, j)];
            }
        }

        t
    }

    pub fn determinant(&self) -> f64 {
        if self.rows == 2 && self.columns == 2 {
            self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
        } else {
            let mut det = 0.0;
            for column in 0..self.columns() {
                det += self[(0, column)] * self.cofactor(0, column);
            }

            det
        }
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Self {
        assert!(row < self.rows);
        assert!(self.rows > 1);
        assert!(column < self.columns);
        assert!(self.columns > 1);
        let mut sub = Self::zero(self.rows - 1, self.columns - 1);

        for i in 0..sub.rows {
            for j in 0..sub.columns {
                let ii = if i < row { i } else { i + 1 };
                let jj = if j < column { j } else { j + 1 };
                sub[(i, j)] = self[(ii, jj)];
            }
        }

        sub
    }

    pub fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        if (row + column) % 2 == 1 {
            -self.minor(row, column)
        } else {
            self.minor(row, column)
        }
    }

    pub fn is_invertible(&self) -> bool {
        !crate::equal(self.determinant(), 0.0)
    }

    pub fn inverse(&self) -> Self {
        assert!(self.is_invertible());
        assert!(self.rows == self.columns);

        let mut inv = Matrix::zero(self.rows, self.columns);
        let det = self.determinant();

        for i in 0..self.rows {
            for j in 0..self.columns {
                let c = self.cofactor(i, j);
                inv[(j, i)] = c / det;
            }
        }

        inv
    }

    pub fn translate<T: Into<f64> + Copy>(&self, x: T, y: T, z: T) -> Self {
        let t = translation(x, y, z);
        &t * self
    }

    pub fn scale<T: Into<f64> + Copy>(&self, x: T, y: T, z: T) -> Self {
        let s = scaling(x, y, z);
        &s * self
    }

    pub fn rotate_x(&self, radians: f64) -> Matrix {
        let r = rotation_x(radians);
        &r * self
    }

    pub fn rotate_y(&self, radians: f64) -> Matrix {
        let r = rotation_y(radians);
        &r * self
    }

    pub fn rotate_z(&self, radians: f64) -> Matrix {
        let r = rotation_z(radians);
        &r * self
    }

    pub fn shear<T: Into<f64> + Copy>(&self, xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Self {
        let s = shearing(xy, xz, yx, yz, zx, zy);
        &s * self
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (i, j): (usize, usize)) -> &f64 {
        &self.elements[self.idx(i, j)]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        let idx = self.idx(i, j);
        &mut self.elements[idx]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, rhs: &Self) -> bool {
        self.elements
            .iter()
            .zip(rhs.elements.iter())
            .all(|(&l, &r)| crate::equal(l, r))
    }
}

impl<'a, 'b> Mul<&'b Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &'b Matrix) -> Matrix {
        assert_eq!(self.columns, rhs.rows);

        let mut m = Matrix::zero(self.rows, rhs.columns);

        for row in 0..self.rows {
            for col in 0..rhs.columns {
                let mut c = 0.0;
                for i in 0..self.columns {
                    c += self[(row, i)] * rhs[(i, col)]
                }
                m[(row, col)] = c;
            }
        }

        m
    }
}

impl Mul<Point> for &Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        assert_eq!(self.rows, 4);
        assert_eq!(self.columns, 4);

        Point::new(
            self[(0, 0)] * rhs.x + self[(0, 1)] * rhs.y + self[(0, 2)] * rhs.z + self[(0, 3)],
            self[(1, 0)] * rhs.x + self[(1, 1)] * rhs.y + self[(1, 2)] * rhs.z + self[(1, 3)],
            self[(2, 0)] * rhs.x + self[(2, 1)] * rhs.y + self[(2, 2)] * rhs.z + self[(2, 3)],
        )
    }
}

impl Mul<Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        assert_eq!(self.columns, 4);
        assert_eq!(self.columns, 4);

        Vector::new(
            self[(0, 0)] * rhs.x + self[(0, 1)] * rhs.y + self[(0, 2)] * rhs.z,
            self[(1, 0)] * rhs.x + self[(1, 1)] * rhs.y + self[(1, 2)] * rhs.z,
            self[(2, 0)] * rhs.x + self[(2, 1)] * rhs.y + self[(2, 2)] * rhs.z,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[should_panic]
    #[test]
    fn create_matrix_wrong_dimensions() {
        #[rustfmt::skip]
        let _m = Matrix::from_slice(
            5,
            5,
            &[
                1.0, 2.0, 3.0, 4.0,
                5.5, 6.5, 7.5, 8.5,
                9.0, 10.0, 11.0, 12.0,
                13.5, 14.5, 15.5, 16.5,
            ],
        );
    }

    #[test]
    fn create_matrix_4x4() {
        #[rustfmt::skip]
        let m = Matrix::from_slice(
            4,
            4,
            &[
                1.0, 2.0, 3.0, 4.0,
                5.5, 6.5, 7.5, 8.5,
                9.0, 10.0, 11.0, 12.0,
                13.5, 14.5, 15.5, 16.5,
            ],
        );

        assert_eq!(m.rows(), 4);
        assert_eq!(m.columns(), 4);

        assert!(crate::equal(m[(0, 0)], 1.0));
        assert!(crate::equal(m[(0, 3)], 4.0));
        assert!(crate::equal(m[(1, 0)], 5.5));
        assert!(crate::equal(m[(1, 2)], 7.5));
        assert!(crate::equal(m[(2, 2)], 11.0));
        assert!(crate::equal(m[(3, 0)], 13.5));
        assert!(crate::equal(m[(3, 2)], 15.5));
    }

    #[test]
    fn create_matrix_2x2() {
        let m = Matrix::from_slice(2, 2, &[-3.0, 5.0, 1.0, -2.0]);

        assert_eq!(m.rows(), 2);
        assert_eq!(m.columns(), 2);

        assert!(crate::equal(m[(0, 0)], -3.0));
        assert!(crate::equal(m[(0, 1)], 5.0));
        assert!(crate::equal(m[(1, 0)], 1.0));
        assert!(crate::equal(m[(1, 1)], -2.0));
    }

    #[test]
    fn create_matrix_3x3() {
        let m = Matrix::from_slice(3, 3, &[-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert_eq!(m.rows(), 3);
        assert_eq!(m.columns(), 3);

        assert!(crate::equal(m[(0, 0)], -3.0));
        assert!(crate::equal(m[(1, 1)], -2.0));
        assert!(crate::equal(m[(2, 2)], 1.0));
    }

    #[test]
    fn identical_matrices_are_equal() {
        let a = Matrix::from_slice(
            4,
            4,
            &[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );
        let b = Matrix::from_rows(
            4,
            4,
            &[
                &[1.0, 2.0, 3.0, 4.0],
                &[5.0, 6.0, 7.0, 8.0],
                &[9.0, 8.0, 7.0, 6.0],
                &[5.0, 4.0, 3.0, 2.0],
            ],
        );
        assert_eq!(a, b);
    }

    #[test]
    fn different_matrices_are_not_equal() {
        let a = Matrix::from_slice(
            4,
            4,
            &[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );
        let b = Matrix::from_slice(
            4,
            4,
            &[
                2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
            ],
        );
        assert_ne!(a, b);
    }

    #[test]
    fn muliply_two_matrices() {
        let a = Matrix::from_slice(
            4,
            4,
            &[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );

        let b = Matrix::from_rows(
            4,
            4,
            &[&[-2, 1, 2, 3], &[3, 2, 1, -1], &[4, 3, 6, 5], &[1, 2, 7, 8]],
        );
        let expected = Matrix::from_rows(
            4,
            4,
            &[
                &[20, 22, 50, 48],
                &[44, 54, 114, 108],
                &[40, 58, 110, 102],
                &[16, 26, 46, 42],
            ],
        );
        assert_eq!(&a * &b, expected);
    }

    #[test]
    fn multiply_matrix_by_point() {
        let a = Matrix::from_rows(
            4,
            4,
            &[&[1, 2, 3, 4], &[2, 4, 4, 2], &[8, 6, 4, 1], &[0, 0, 0, 1]],
        );
        let p = Point::new(1.0, 2.0, 3.0);
        let expected = Point::new(18.0, 24.0, 33.0);
        assert_eq!(&a * p, expected);
    }

    #[test]
    fn multiply_matrix_by_vector() {
        let a = Matrix::from_rows(
            4,
            4,
            &[&[1, 2, 3, 4], &[2, 4, 4, 2], &[8, 6, 4, 1], &[0, 0, 0, 1]],
        );
        let v = Vector::new(1.0, 2.0, 3.0);
        let expected = Vector::new(14.0, 22.0, 32.0);
        assert_eq!(&a * v, expected);
    }

    #[test]
    fn multiply_matrix_by_identity() {
        let a = Matrix::from_rows(
            4,
            4,
            &[
                &[0, 1, 2, 4],
                &[1, 2, 4, 8],
                &[2, 4, 8, 16],
                &[4, 8, 16, 32],
            ],
        );

        assert_eq!(&a * &Matrix::identity(4, 4), a);
    }

    #[test]
    fn transpose_matrix() {
        let a = Matrix::from_rows(
            4,
            4,
            &[&[0, 9, 3, 0], &[9, 8, 0, 8], &[1, 8, 5, 3], &[0, 0, 5, 8]],
        );
        let expected = Matrix::from_rows(
            4,
            4,
            &[&[0, 9, 1, 0], &[9, 8, 8, 0], &[3, 0, 5, 5], &[0, 8, 3, 8]],
        );
        assert_eq!(a.transpose(), expected);
    }

    #[test]
    fn transpose_identity() {
        let id = Matrix::identity(4, 4);
        assert_eq!(id.transpose(), id);
    }

    #[test]
    fn determinant_2x2() {
        let a = Matrix::from_slice(2, 2, &[1, 5, -3, 2]);
        assert!(crate::equal(a.determinant(), 17.0));
    }

    #[test]
    fn submatrix_3x3_is_2x2() {
        let a = Matrix::from_rows(3, 3, &[&[1, 5, 0], &[-3, 2, 7], &[0, 6, 3]]);
        let sub = a.submatrix(0, 2);
        assert_eq!(sub.rows(), 2);
        assert_eq!(sub.columns(), 2);
        let expected = Matrix::from_slice(2, 2, &[-3, 2, 0, 6]);
        assert_eq!(sub, expected);
    }

    #[test]
    fn submatrix_4x4_is_3x3() {
        let a = Matrix::from_rows(
            4,
            4,
            &[
                &[-6, 1, 1, 6],
                &[-8, 5, 8, 6],
                &[-1, 0, 8, 2],
                &[-7, 1, -1, 1],
            ],
        );
        let sub = a.submatrix(2, 1);
        assert_eq!(sub.rows(), 3);
        assert_eq!(sub.columns(), 3);
        let expected = Matrix::from_slice(3, 3, &[-6, 1, 6, -8, 8, 6, -7, -1, 1]);
        assert_eq!(sub, expected);
    }

    #[test]
    fn minor_3x3() {
        let a = Matrix::from_rows(3, 3, &[&[3, 5, 0], &[2, -1, -7], &[6, -1, 5]]);
        let b = a.submatrix(1, 0);
        assert!(crate::equal(b.determinant(), 25.0));
        assert!(crate::equal(a.minor(1, 0), 25.0));
    }

    #[test]
    fn cofactor_3x3() {
        let a = Matrix::from_rows(3, 3, &[&[3, 5, 0], &[2, -1, -7], &[6, -1, 5]]);
        assert!(crate::equal(a.minor(0, 0), -12.0));
        assert!(crate::equal(a.cofactor(0, 0), -12.0));
        assert!(crate::equal(a.minor(1, 0), 25.0));
        assert!(crate::equal(a.cofactor(1, 0), -25.0))
    }

    #[test]
    fn determinant_3x3() {
        let a = Matrix::from_rows(3, 3, &[&[1, 2, 6], &[-5, 8, -4], &[2, 6, 4]]);
        assert!(crate::equal(a.cofactor(0, 0), 56.0));
        assert!(crate::equal(a.cofactor(0, 1), 12.0));
        assert!(crate::equal(a.cofactor(0, 2), -46.0));
        assert!(crate::equal(a.determinant(), -196.0));
    }

    #[test]
    fn determinant_4x4() {
        let a = Matrix::from_rows(
            4,
            4,
            &[
                &[-2, -8, 3, 5],
                &[-3, 1, 7, 3],
                &[1, 2, -9, 6],
                &[-6, 7, 7, -9],
            ],
        );
        assert!(crate::equal(a.cofactor(0, 0), 690.0));
        assert!(crate::equal(a.cofactor(0, 1), 447.0));
        assert!(crate::equal(a.cofactor(0, 2), 210.0));
        assert!(crate::equal(a.cofactor(0, 3), 51.0));
        assert!(crate::equal(a.determinant(), -4071.0));
    }

    #[test]
    fn invertible_matrix() {
        let a = Matrix::from_rows(
            4,
            4,
            &[
                &[6, 4, 4, 4],
                &[5, 5, 7, 6],
                &[4, -9, 3, -7],
                &[9, 1, 7, -6],
            ],
        );
        assert!(crate::equal(a.determinant(), -2120.0));
        assert!(a.is_invertible());
    }

    #[test]
    fn non_invertible_matrix() {
        let a = Matrix::from_rows(
            4,
            4,
            &[
                &[-4, 2, -2, -3],
                &[9, 6, 2, 6],
                &[0, -5, 1, -5],
                &[0, 0, 0, 0],
            ],
        );
        assert!(crate::equal(a.determinant(), 0.0));
        assert!(!a.is_invertible());
    }

    #[test]
    fn inverse_matrix1() {
        let a = Matrix::from_rows(
            4,
            4,
            &[
                &[-5, 2, 6, -8],
                &[1, -5, 1, 8],
                &[7, 7, -6, -7],
                &[1, -3, 7, 4],
            ],
        );
        let b = a.inverse();

        assert!(crate::equal(a.determinant(), 532.0));
        assert!(crate::equal(a.cofactor(2, 3), -160.0));
        assert!(crate::equal(b[(3, 2)], -160.0 / 532.0));
        assert!(crate::equal(a.cofactor(3, 2), 105.0));
        assert!(crate::equal(b[(2, 3)], 105.0 / 532.0));

        let expected = Matrix::from_rows(
            4,
            4,
            &[
                &[0.21805, 0.45113, 0.24060, -0.04511],
                &[-0.80827, -1.45677, -0.44361, 0.52068],
                &[-0.07895, -0.22368, -0.05263, 0.19737],
                &[-0.52256, -0.81391, -0.30075, 0.30639],
            ],
        );

        assert_eq!(b, expected);
    }

    #[test]
    fn inverse_matrix3() {
        let a = Matrix::from_rows(
            4,
            4,
            &[
                &[9, 3, 0, 9],
                &[-5, -2, -6, -3],
                &[-4, 9, 6, 4],
                &[-7, 6, 6, 2],
            ],
        );

        let expected = Matrix::from_rows(
            4,
            4,
            &[
                &[-0.04074, -0.07778, 0.14444, -0.22222],
                &[-0.07778, 0.03333, 0.36667, -0.33333],
                &[-0.02901, -0.14630, -0.10926, 0.12963],
                &[0.17778, 0.06667, -0.26667, 0.33333],
            ],
        );

        assert_eq!(a.inverse(), expected);
    }

    #[test]
    fn multiply_matrix_product_by_its_inverse() {
        let a = Matrix::from_rows(
            4,
            4,
            &[
                &[3, -9, 7, 3],
                &[3, -8, 2, -9],
                &[-4, 4, 4, 1],
                &[-6, 5, -1, 1],
            ],
        );
        let b = Matrix::from_rows(
            4,
            4,
            &[&[8, 2, 2, 2], &[3, -1, 7, 0], &[7, 0, 5, 4], &[6, -2, 0, 5]],
        );
        let c = &a * &b;
        assert_eq!(&c * &b.inverse(), a);
    }

    #[test]
    fn transformations_fluent_api_chaining() {
        let p = Point::new(1, 0, 1);
        let t = Matrix::identity(4, 4)
            .rotate_x(PI / 2.0)
            .scale(5, 5, 5)
            .translate(10, 5, 7);
        assert_eq!(&t * p, Point::new(15, 0, 7));
    }
}
