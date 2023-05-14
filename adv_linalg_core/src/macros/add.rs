macro_rules! vector_unsliced_immut_add {
    ($rhs_type:ty) => {
        type Output = Vector<T>;

        fn add(self, rhs: $rhs_type) -> Self::Output {
            if self.len() != rhs.len() {
                panic!("Vectors with different sizes cannot be added together.")
            }
            
            let length = self.len();

            let mut params = Vec::with_capacity(length);
            for idx in 0..length {
                params.push(self.list[idx].clone() + rhs.list[idx].clone())
            }

            Vector::from(params)
        }
    }
}
pub(crate) use vector_unsliced_immut_add;

macro_rules! vector_right_sliced_immut_add {
    ($rhs_type:ty) => {
        type Output = Vector<T>;

        fn add(self, rhs: $rhs_type) -> Self::Output {
            if self.len() != rhs.len() {
                panic!("Vectors with different sizes cannot be added together.")
            }
            
            let length = self.len();
    
            let mut params = Vec::with_capacity(length);
            for idx in 0..length {
                params.push(self.list[idx].clone() + rhs.vector.list[idx + rhs.start()].clone())
            }
    
            Vector::from(params)
        }
    }
}
pub(crate) use vector_right_sliced_immut_add;

macro_rules! vector_left_sliced_immut_add {
    ($rhs_type:ty) => {
        type Output = Vector<T>;

        fn add(self, rhs: $rhs_type) -> Self::Output {
            if self.len() != rhs.len() {
                panic!("Vectors with different sizes cannot be added together.")
            }
            
            let length = rhs.len();
    
            let mut params = Vec::with_capacity(length);
            for idx in 0..length {
                params.push(self.vector.list[idx + self.start()].clone() + rhs.list[idx].clone())
            }
    
            Vector::from(params)
        }
    }
}
pub(crate) use vector_left_sliced_immut_add;

macro_rules! vector_both_sliced_immut_add {
    ($rhs_type:ty) => {
        type Output = Vector<T>;

        fn add(self, rhs: $rhs_type) -> Self::Output {
            if self.len() != rhs.len() {
                panic!("Vectors with different sizes cannot be added together.")
            }
            
            let length = self.len();
    
            let mut params = Vec::with_capacity(length);
            for idx in 0..length {
                params.push(self.vector.list[idx + rhs.start()].clone() + rhs.vector.list[idx + rhs.start()].clone())
            }
    
            Vector::from(params)
        }
    }
}
pub(crate) use vector_both_sliced_immut_add;

macro_rules! vector_sliced_mut_add {
    ($rhs_type:ty) => {
        type Output = Self;

        fn add(self, rhs: $rhs_type) -> Self::Output {
            if self.len() != rhs.len() {
                panic!("Vectors with different sizes cannot be added together.")
            }
    
            for idx in 0..self.len() {
                self.list[idx] = self.list[idx].clone() + rhs.vector.list[idx + rhs.start()].clone()
            }
    
            self
        }
    }
}
pub(crate) use vector_sliced_mut_add;

macro_rules! vector_unsliced_mut_add {
    ($rhs_type:ty) => {
        type Output = Self;

        fn add(self, rhs: $rhs_type) -> Self::Output {
            if self.len() != rhs.len() {
                panic!("Vectors with different sizes cannot be added together.")
            }
    
            for idx in 0..self.len() {
                self.list[idx] = self.list[idx].clone() + rhs.list[idx].clone()
            }
    
            self
        }
    }
}
pub(crate) use vector_unsliced_mut_add;

macro_rules! matrix_unsliced_immut_add {
    ($rhs_type:ty) => {
        type Output = Matrix<T>;

        fn add(self, rhs: $rhs_type) -> Self::Output {
            if self.rows != rhs.rows || self.cols != rhs.cols {
                panic!("Matrices with different sizes cannot be added together.")
            }

            let mut params = Vec::with_capacity(self.rows);
            
            for row_idx in 0..self.rows {
                let mut param_row = Vec::with_capacity(self.cols);

                for col_idx in 0..self.cols {
                    param_row.push(self.matrix[row_idx][col_idx].clone() + rhs.matrix[row_idx][col_idx].clone())
                }
                
                params.push(param_row)
            }

            Matrix::from(params)
        }
    }
}
pub(crate) use matrix_unsliced_immut_add;

macro_rules! matrix_right_sliced_immut_add {
    ($rhs_type:ty) => {
        type Output = Matrix<T>;

        fn add(self, rhs: $rhs_type) -> Self::Output {
            if self.rows != rhs.rows() || self.cols != rhs.cols() {
                panic!("Matrices with different sizes cannot be added together.")
            }
            
            let mut params = Vec::with_capacity(self.rows);
            
            for row_idx in 0..self.rows {
                let mut param_row = Vec::with_capacity(self.cols);

                for col_idx in 0..self.cols {
                    param_row.push(self.matrix[row_idx][col_idx].clone() + rhs.matrix.matrix[row_idx + rhs.row_start()][col_idx + rhs.col_start()].clone())
                }
                
                params.push(param_row)
            }

            Matrix::from(params)
        }
    }
}
pub(crate) use matrix_right_sliced_immut_add;

macro_rules! matrix_left_sliced_immut_add {
    ($rhs_type:ty) => {
        type Output = Matrix<T>;

        fn add(self, rhs: $rhs_type) -> Self::Output {
            if self.rows() != rhs.rows || self.cols() != rhs.cols {
                panic!("Matrices with different sizes cannot be added together.")
            }
            
            let mut params = Vec::with_capacity(rhs.rows);
            
            for row_idx in 0..rhs.rows {
                let mut param_row = Vec::with_capacity(rhs.cols);

                for col_idx in 0..rhs.cols {
                    param_row.push(self.matrix.matrix[row_idx + self.row_start()][col_idx + self.col_start()].clone() + rhs.matrix[row_idx][col_idx].clone())
                }
                
                params.push(param_row)
            }

            Matrix::from(params)
        }
    }
}
pub(crate) use matrix_left_sliced_immut_add;

macro_rules! matrix_both_sliced_immut_add {
    ($rhs_type:ty) => {
        type Output = Matrix<T>;

        fn add(self, rhs: $rhs_type) -> Self::Output {
            if self.rows() != rhs.rows() || self.cols() != rhs.cols() {
                panic!("Matrices with different sizes cannot be added together.")
            }
            
            let mut params = Vec::with_capacity(rhs.rows());
            
            for row_idx in 0..rhs.rows() {
                let mut param_row = Vec::with_capacity(rhs.cols());

                for col_idx in 0..self.cols() {
                    param_row.push(self.matrix.matrix[row_idx + self.row_start()][col_idx + self.col_start()].clone() + rhs.matrix.matrix[row_idx + rhs.row_start()][col_idx + rhs.col_start()].clone())
                }
                
                params.push(param_row)
            }

            Matrix::from(params)
        }
    }
}
pub(crate) use matrix_both_sliced_immut_add;

macro_rules! matrix_sliced_mut_add {
    ($rhs_type:ty) => {
        type Output = Self;

        fn add(self, rhs: $rhs_type) -> Self::Output {
            if self.rows != rhs.rows() || self.cols != rhs.cols() {
                panic!("Matrices with different sizes cannot be added together.")
            }
    
            for row_idx in 0..self.rows {
                for col_idx in 0..self.cols {
                    self.matrix[row_idx][col_idx] = self.matrix[row_idx][col_idx].clone() + rhs.matrix.matrix[row_idx + rhs.row_start()][col_idx + rhs.col_start()].clone()
                }
            }
    
            self
        }
    }
}
pub(crate) use matrix_sliced_mut_add;

macro_rules! matrix_unsliced_mut_add {
    ($rhs_type:ty) => {
        type Output = Self;

        fn add(self, rhs: $rhs_type) -> Self::Output {
            if self.rows != rhs.rows || self.cols != rhs.cols {
                panic!("Matrices with different sizes cannot be added together.")
            }
    
            for row_idx in 0..self.rows {
                for col_idx in 0..self.cols {
                    self.matrix[row_idx][col_idx] = self.matrix[row_idx][col_idx].clone() + rhs.matrix[row_idx][col_idx].clone()
                }
            }
    
            self
        }
    }
}
pub(crate) use matrix_unsliced_mut_add;