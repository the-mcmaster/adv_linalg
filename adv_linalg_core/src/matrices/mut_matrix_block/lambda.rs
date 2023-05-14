use crate::Matrix;
use crate::matrices::MutMatrixBlock;
use crate::traits::Lambda;

impl<'a, T> Lambda for MutMatrixBlock<'a, T>
{
    type Output = Matrix<T>;

    type Inner = T;

    type Index = (usize, usize);

    fn lambda<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner) -> Self::Inner
    {
        let mut params = Vec::with_capacity(self.rows());

        for row_idx in self.row_start()..self.row_end() {
            
            let mut param_row = Vec::with_capacity(self.cols());
            
            for col_idx in self.col_start()..self.col_end() {
                param_row.push(funct(&self.matrix.matrix[row_idx][col_idx]))
            }
            
            params.push(param_row);
        
        }

        Matrix::from(params)
    }

    fn lambda_index<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index) -> Self::Inner
    {
        let mut params = Vec::with_capacity(self.rows());

        for row_idx in self.row_start()..self.row_end() {
            
            let mut param_row = Vec::with_capacity(self.cols());
            
            for col_idx in self.col_start()..self.col_end() {
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
        let mut params = Vec::with_capacity(self.rows());

        for row_idx in self.row_start()..self.row_end() {
            
            let mut param_row = Vec::with_capacity(self.cols());
            
            for col_idx in self.col_start()..self.col_end() {
                param_row.push(funct((row_idx, col_idx), &self.matrix.matrix[row_idx][col_idx]))
            }
            
            params.push(param_row);
        
        }

        Matrix::from(params)
    }
}