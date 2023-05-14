use crate::Matrix;
use crate::traits::Lambda;

impl<T> Lambda for Matrix<T>
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