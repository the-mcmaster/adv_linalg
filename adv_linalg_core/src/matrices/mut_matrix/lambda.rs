use crate::Matrix;
use crate::matrices::MutMatrix;
use crate::traits::{Lambda, LambdaMut};

impl<T> Lambda for MutMatrix<T>
{
    type Output = Matrix<T>;

    type Inner = T;

    type Index = (usize, usize);

    fn lambda<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner) -> Self::Inner
    {
        let mut params = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            
            let mut param_row = Vec::with_capacity(self.cols);
            
            for col_idx in 0..self.cols {
                param_row.push(funct(&self.matrix[row_idx][col_idx]))
            }
            
            params.push(param_row);
        
        }

        Matrix::from(params)
    }

    fn lambda_index<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index) -> Self::Inner
    {
        let mut params = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            
            let mut param_row = Vec::with_capacity(self.cols);
            
            for col_idx in 0..self.cols {
                param_row.push(funct((row_idx, col_idx)))
            }
            
            params.push(param_row);
        
        }

        Matrix::from(params)
    }

    fn lambda_enumerate<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner) -> Self::Inner
    {
        let mut params = Vec::with_capacity(self.rows);

        for row_idx in 0..self.rows {
            
            let mut param_row = Vec::with_capacity(self.cols);
            
            for col_idx in 0..self.cols {
                param_row.push(funct((row_idx, col_idx), &self.matrix[row_idx][col_idx]))
            }
            
            params.push(param_row);
        
        }

        Matrix::from(params)
    }
}

impl<'x, T: 'x> LambdaMut<'x> for MutMatrix<T> {
    type Output = &'x mut Self;

    type Inner = T;

    type Index = (usize, usize);

    fn lambda_mut<F>(&'x mut self, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner) -> Self::Inner
    {
        for row_idx in 0..self.rows {
            
            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = funct(&self.matrix[row_idx][col_idx])
            }
        
        }

        self
    }

    fn lambda_index_mut<F>(&'x mut self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index) -> Self::Inner
    {
        for row_idx in 0..self.rows {
            
            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = funct((row_idx, col_idx))
            }
        
        }

        self
    }

    fn lambda_enumerate_mut<F>(&'x mut self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner) -> Self::Inner
    {
        for row_idx in 0..self.rows {
            
            for col_idx in 0..self.cols {
                self.matrix[row_idx][col_idx] = funct((row_idx, col_idx), &self.matrix[row_idx][col_idx])
            }
        
        }

        self
    }
}