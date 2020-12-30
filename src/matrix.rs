use std::{
    ops::{Index, IndexMut, Mul},
    vec,
};

#[derive(Debug)]
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
            .all(|(&l, &r)| l == r)
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
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

#[cfg(test)]
mod tests {
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
        assert_eq!(a * b, expected);
    }
}
