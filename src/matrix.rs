use std::{
    ops::{Index, IndexMut},
    vec,
};

struct Matrix {
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

    pub fn from(rows: usize, columns: usize, slice: &[f64]) -> Self {
        assert_eq!(rows * columns, slice.len());
        Self {
            rows,
            columns,
            elements: slice.to_vec(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    fn create_matrix_wrong_dimensions() {
        #[rustfmt::skip]
        let _m = Matrix::from(
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
        let m = Matrix::from(
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
        let m = Matrix::from(2, 2, &[-3.0, 5.0, 1.0, -2.0]);

        assert_eq!(m.rows(), 2);
        assert_eq!(m.columns(), 2);

        assert!(crate::equal(m[(0, 0)], -3.0));
        assert!(crate::equal(m[(0, 1)], 5.0));
        assert!(crate::equal(m[(1, 0)], 1.0));
        assert!(crate::equal(m[(1, 1)], -2.0));
    }

    #[test]
    fn create_matrix_3x3() {
        let m = Matrix::from(3, 3, &[-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert_eq!(m.rows(), 3);
        assert_eq!(m.columns(), 3);

        assert!(crate::equal(m[(0, 0)], -3.0));
        assert!(crate::equal(m[(1, 1)], -2.0));
        assert!(crate::equal(m[(2, 2)], 1.0));
    }
}
