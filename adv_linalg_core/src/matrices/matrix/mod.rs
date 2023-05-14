use crate::Matrix;
use crate::matrices::{MatrixBlock, MutMatrix, MutMatrixBlock};

mod add;
mod mul;
mod sub;
mod lambda;
mod map;

impl<T> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(matrix: Vec<Vec<T>>) -> Self {
        let rows = matrix.len();
        let cols = matrix[0].len();

        for row in 1..rows {
            if matrix[row].len() != cols {
                panic!("Input 2D Vec does not have same length for all rows")
            }
        }
        Matrix {
            matrix, 
            rows, 
            cols
        }
    }
}

impl<T> From<&Vec<Vec<T>>> for Matrix<T>
where
    T: Clone
{
    fn from(matrix: &Vec<Vec<T>>) -> Self {
        let rows = matrix.len();
        let cols = matrix[0].len();

        for row in 1..rows {
            if matrix[row].len() != cols {
                panic!("Input 2D Vec does not have same length for all rows")
            }
        }
        Matrix {
            matrix: matrix.clone(), 
            rows, 
            cols
        }
    }
}

impl<T> From<MatrixBlock<'_, T>> for Matrix<T>
where
    T: Clone
{
    fn from(matrix_block: MatrixBlock<T>) -> Self {
        let rows = matrix_block.rows();
        let cols = matrix_block.cols();

        let mut matrix = vec![Vec::with_capacity(matrix_block.cols()); matrix_block.rows()];

        for row in matrix_block.matrix.matrix
                            .split_at(matrix_block.row_start()).1
                            .split_at(matrix_block.row_end()).0 {
            matrix.push(row
                .split_at(matrix_block.col_start()).1
                .split_at(matrix_block.col_end()).0
                .into()
            );
        }

        Matrix {
            matrix, 
            rows, 
            cols
        }
    }
}

impl<T> From<&MatrixBlock<'_, T>> for Matrix<T>
where
    T: Clone
{
    fn from(matrix_block: &MatrixBlock<T>) -> Self {
        let rows = matrix_block.rows();
        let cols = matrix_block.cols();

        let mut matrix = vec![Vec::with_capacity(matrix_block.cols()); matrix_block.rows()];

        for row in matrix_block.matrix.matrix
                            .split_at(matrix_block.row_start()).1
                            .split_at(matrix_block.row_end()).0 {
            matrix.push(row
                .split_at(matrix_block.col_start()).1
                .split_at(matrix_block.col_end()).0
                .into()
            );
        }

        Matrix {
            matrix, 
            rows, 
            cols
        }
    }
}

impl<T> From<MutMatrixBlock<'_, T>> for Matrix<T>
where
    T: Clone
{
    fn from(matrix_block: MutMatrixBlock<T>) -> Self {
        let rows = matrix_block.rows();
        let cols = matrix_block.cols();

        let mut matrix = vec![Vec::with_capacity(matrix_block.cols()); matrix_block.rows()];

        for row in matrix_block.matrix.matrix
                            .split_at(matrix_block.row_start()).1
                            .split_at(matrix_block.row_end()).0 {
            matrix.push(row
                .split_at(matrix_block.col_start()).1
                .split_at(matrix_block.col_end()).0
                .into()
            );
        }

        Matrix {
            matrix, 
            rows, 
            cols
        }
    }
}

impl<T> From<&MutMatrixBlock<'_, T>> for Matrix<T>
where
    T: Clone
{
    fn from(matrix_block: &MutMatrixBlock<T>) -> Self {
        let rows = matrix_block.rows();
        let cols = matrix_block.cols();

        let mut matrix = vec![Vec::with_capacity(matrix_block.cols()); matrix_block.rows()];

        for row in matrix_block.matrix.matrix
                            .split_at(matrix_block.row_start()).1
                            .split_at(matrix_block.row_end()).0 {
            matrix.push(row
                .split_at(matrix_block.col_start()).1
                .split_at(matrix_block.col_end()).0
                .into()
            );
        }

        Matrix {
            matrix, 
            rows, 
            cols
        }
    }
}

impl<T> From<MutMatrix<T>> for Matrix<T> {
    fn from(matrix: MutMatrix<T>) -> Self {
        Matrix {
            matrix: matrix.matrix,
            rows: matrix.rows,
            cols: matrix.cols
        }
    }
}

impl<T> From<&MutMatrix<T>> for Matrix<T>
where
    T: Clone
{
    fn from(matrix: &MutMatrix<T>) -> Self {
        Matrix {
            matrix: matrix.matrix.clone(),
            rows: matrix.rows,
            cols: matrix.cols
        }
    }
}