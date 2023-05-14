use crate::Matrix;
use crate::matrices::{MutMatrix, MatrixBlock, MutMatrixBlock};
use crate::traits::{Map, MapMut};

impl<T> Map<Matrix<T>> for MutMatrix<T> {
    type Output = Matrix<T>;

    type Inner = T;

    type Index = (usize, usize);

    fn map<F>(&self, other: &Matrix<T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols { 
            panic!("Cannot map matrices of different column lengths.")
        }

        let mut params = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            let mut param_row = Vec::with_capacity(self.cols);

            for col_idx in 0..self.cols {
                param_row.push(funct(&self.matrix[row_idx][col_idx], &other.matrix[row_idx][col_idx]))
            }

            params.push(param_row);
        }

        Matrix::from(params)
    }

    fn map_enumerate<F>(&self, other: &Matrix<T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols { 
            panic!("Cannot map matrices of different column lengths.")
        }

        let mut params = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            let mut param_row = Vec::with_capacity(self.cols);

            for col_idx in 0..self.cols {
                param_row.push(funct((row_idx, col_idx), &self.matrix[row_idx][col_idx], &other.matrix[row_idx][col_idx]))
            }

            params.push(param_row);
        }

        Matrix::from(params)
    }
}

impl<T> Map<MutMatrix<T>> for MutMatrix<T> {
    type Output = Matrix<T>;

    type Inner = T;

    type Index = (usize, usize);

    fn map<F>(&self, other: &MutMatrix<T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols { 
            panic!("Cannot map matrices of different column lengths.")
        }

        let mut params = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            let mut param_row = Vec::with_capacity(self.cols);

            for col_idx in 0..self.cols {
                param_row.push(funct(&self.matrix[row_idx][col_idx], &other.matrix[row_idx][col_idx]))
            }

            params.push(param_row);
        }

        Matrix::from(params)
    }

    fn map_enumerate<F>(&self, other: &MutMatrix<T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols { 
            panic!("Cannot map matrices of different column lengths.")
        }

        let mut params = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            let mut param_row = Vec::with_capacity(self.cols);

            for col_idx in 0..self.cols {
                param_row.push(funct((row_idx, col_idx), &self.matrix[row_idx][col_idx], &other.matrix[row_idx][col_idx]))
            }

            params.push(param_row);
        }

        Matrix::from(params)
    }
}

impl<'r, T> Map<MatrixBlock<'r, T>> for MutMatrix<T> {
    type Output = Matrix<T>;

    type Inner = T;

    type Index = (usize, usize);

    fn map<F>(&self, other: &MatrixBlock<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows() { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols() { 
            panic!("Cannot map matrices of different column lengths.")
        }

        let mut params = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            let mut param_row = Vec::with_capacity(self.cols);

            for col_idx in 0..self.cols {
                param_row.push(funct(&self.matrix[row_idx][col_idx], &other.matrix.matrix[row_idx + other.row_start()][col_idx + other.col_start()]))
            }

            params.push(param_row);
        }

        Matrix::from(params)
    }

    fn map_enumerate<F>(&self, other: &MatrixBlock<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows() { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols() { 
            panic!("Cannot map matrices of different column lengths.")
        }

        let mut params = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            let mut param_row = Vec::with_capacity(self.cols);

            for col_idx in 0..self.cols {
                param_row.push(funct((row_idx, col_idx), &self.matrix[row_idx][col_idx], &other.matrix.matrix[row_idx + other.row_start()][col_idx + other.col_start()]))
            }

            params.push(param_row);
        }

        Matrix::from(params)
    }
}

impl<'r, T> Map<MutMatrixBlock<'r, T>> for MutMatrix<T> {
    type Output = Matrix<T>;

    type Inner = T;

    type Index = (usize, usize);

    fn map<F>(&self, other: &MutMatrixBlock<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows() { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols() { 
            panic!("Cannot map matrices of different column lengths.")
        }

        let mut params = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            let mut param_row = Vec::with_capacity(self.cols);

            for col_idx in 0..self.cols {
                param_row.push(funct(&self.matrix[row_idx][col_idx], &other.matrix.matrix[row_idx + other.row_start()][col_idx + other.col_start()]))
            }

            params.push(param_row);
        }

        Matrix::from(params)
    }

    fn map_enumerate<F>(&self, other: &MutMatrixBlock<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows() { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols() { 
            panic!("Cannot map matrices of different column lengths.")
        }

        let mut params = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            let mut param_row = Vec::with_capacity(self.cols);

            for col_idx in 0..self.cols {
                param_row.push(funct((row_idx, col_idx), &self.matrix[row_idx][col_idx], &other.matrix.matrix[row_idx + other.row_start()][col_idx + other.col_start()]))
            }

            params.push(param_row);
        }

        Matrix::from(params)
    }
}

impl<'l, T: 'l> MapMut<'l, Matrix<T>> for MutMatrix<T> {
    type Output = &'l mut Self;

    type Inner = T;

    type Index = (usize, usize);

    fn map_mut<F>(&'l mut self, other: &Matrix<T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols { 
            panic!("Cannot map matrices of different column lengths.")
        }

        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = funct(&self.matrix[row_idx][col_idx], &other.matrix[row_idx][col_idx])
            }
        }

        self
    }

    fn map_enumerate_mut<F>(&'l mut self, other: &Matrix<T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols { 
            panic!("Cannot map matrices of different column lengths.")
        }

        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = funct((row_idx, col_idx), &self.matrix[row_idx][col_idx], &other.matrix[row_idx][col_idx])
            }
        }

        self
    }
}

impl<'l, T: 'l> MapMut<'l, MutMatrix<T>> for MutMatrix<T> {
    type Output = &'l mut Self;

    type Inner = T;

    type Index = (usize, usize);

    fn map_mut<F>(&'l mut self, other: &MutMatrix<T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols { 
            panic!("Cannot map matrices of different column lengths.")
        }

        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = funct(&self.matrix[row_idx][col_idx], &other.matrix[row_idx][col_idx])
            }
        }

        self
    }

    fn map_enumerate_mut<F>(&'l mut self, other: &MutMatrix<T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols { 
            panic!("Cannot map matrices of different column lengths.")
        }

        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = funct((row_idx, col_idx), &self.matrix[row_idx][col_idx], &other.matrix[row_idx][col_idx])
            }
        }

        self
    }
}

impl<'l, 'r, T: 'l> MapMut<'l, MatrixBlock<'r, T>> for MutMatrix<T> {
    type Output = &'l mut Self;

    type Inner = T;

    type Index = (usize, usize);

    fn map_mut<F>(&'l mut self, other: &MatrixBlock<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows() { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols() { 
            panic!("Cannot map matrices of different column lengths.")
        }

        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = funct(&self.matrix[row_idx][col_idx], &other.matrix.matrix[row_idx + other.row_start()][col_idx + other.col_start()])
            }
        }

        self
    }

    fn map_enumerate_mut<F>(&'l mut self, other: &MatrixBlock<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows() { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols() { 
            panic!("Cannot map matrices of different column lengths.")
        }

        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = funct((row_idx, col_idx), &self.matrix[row_idx][col_idx], &other.matrix.matrix[row_idx + other.row_start()][col_idx + other.col_start()])
            }
        }

        self
    }
}

impl<'l, 'r, T: 'l> MapMut<'l, MutMatrixBlock<'r, T>> for MutMatrix<T> {
    type Output = &'l mut Self;

    type Inner = T;

    type Index = (usize, usize);

    fn map_mut<F>(&'l mut self, other: &MutMatrixBlock<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows() { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols() { 
            panic!("Cannot map matrices of different column lengths.")
        }

        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = funct(&self.matrix[row_idx][col_idx], &other.matrix.matrix[row_idx + other.row_start()][col_idx + other.col_start()])
            }
        }

        self
    }

    fn map_enumerate_mut<F>(&'l mut self, other: &MutMatrixBlock<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.rows != other.rows() { 
            panic!("Cannot map matrices of different row lengths.")
        }

        if self.cols != other.cols() { 
            panic!("Cannot map matrices of different column lengths.")
        }

        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = funct((row_idx, col_idx), &self.matrix[row_idx][col_idx], &other.matrix.matrix[row_idx + other.row_start()][col_idx + other.col_start()])
            }
        }

        self
    }
}