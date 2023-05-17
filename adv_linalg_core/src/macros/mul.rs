macro_rules! dot_product_unsliced {
    ($rhs_type:ty) => {
        type Output = T;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
            if self.len() != rhs.len() {
                panic!("Cannot find dot product of two differently sized vectors.")
            }
    
            let mut product = T::default();
            
            for idx in 0..self.len() {
                product = product + self.list[idx].clone() * rhs.list[idx].clone()
            }
    
            product
        }
    }
}
pub(crate) use dot_product_unsliced;

macro_rules! dot_product_left_sliced {
    ($rhs_type:ty) => {
        type Output = T;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
            if self.len() != rhs.len() {
                panic!("Cannot find dot product of two differently sized vectors.")
            }
    
            let mut product = T::default();
            
            for idx in 0..self.len() {
                product = product + self.vector.list[idx + self.start()].clone() * rhs.list[idx].clone()
            }
    
            product
        }
    }
}
pub(crate) use dot_product_left_sliced;

macro_rules! dot_product_right_sliced {
    ($rhs_type:ty) => {
        type Output = T;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
            if self.len() != rhs.len() {
                panic!("Cannot find dot product of two differently sized vectors.")
            }
    
            let mut product = T::default();
            
            for idx in 0..self.len() {
                product = product + self.list[idx].clone() * rhs.vector.list[idx + rhs.start()].clone()
            }
    
            product
        }
    }
}
pub(crate) use dot_product_right_sliced;

macro_rules! dot_product_both_sliced {
    ($rhs_type:ty) => {
        type Output = T;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
            if self.len() != rhs.len() {
                panic!("Cannot find dot product of two differently sized vectors.")
            }
    
            let mut product = T::default();
            
            for idx in 0..self.len() {
                product = product + self.vector.list[idx + self.start()].clone() * rhs.vector.list[idx + rhs.start()].clone()
            }
    
            product
        }
    }
}
pub(crate) use dot_product_both_sliced;

macro_rules! vector_unsliced_immut_scaled {
    ($rhs_type:ty) => {
        type Output = Vector<T>;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
    
            let mut params = Vec::with_capacity(self.len());
            
            for idx in 0..self.len() {
                params.push(self.list[idx].clone() * rhs.clone())
            }
    
            Vector::from(params)
        }
    }
}
pub(crate) use vector_unsliced_immut_scaled;

macro_rules! vector_sliced_immut_scaled {
    ($rhs_type:ty) => {
        type Output = Vector<T>;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
    
            let mut params = Vec::with_capacity(self.len());
            
            for idx in self.start()..self.end() {
                params.push(self.vector.list[idx].clone() * rhs.clone())
            }
    
            Vector::from(params)
        }
    }
}
pub(crate) use vector_sliced_immut_scaled;

macro_rules! vector_mut_scaled {
    ($rhs_type:ty) => {
        type Output = Self;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
            
            for idx in 0..self.len() {
                self.list[idx] = self.list[idx].clone() * rhs.clone()
            }
    
            self
        }
    }
}
pub(crate) use vector_mut_scaled;

macro_rules! matrix_unsliced_immut_scaled {
    ($rhs_type:ty) => {
        type Output = Matrix<T>;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
            let mut params = Vec::with_capacity(self.rows);
    
            for row_idx in 0..self.rows {
                let mut params_row = Vec::with_capacity(self.cols);
                
                for col_idx in 0..self.cols {
                    params_row.push(self.matrix[row_idx][col_idx].clone() * rhs.clone())
                }
                
                params.push(params_row)
            }
            
            Matrix::from(params)
        }
    }
}
pub(crate) use matrix_unsliced_immut_scaled;

macro_rules! matrix_sliced_immut_scaled {
    ($rhs_type:ty) => {
        type Output = Matrix<T>;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
            let mut params = Vec::with_capacity(self.rows());
    
            for row_idx in self.row_start()..self.row_end() {
                let mut params_row = Vec::with_capacity(self.cols());
                
                for col_idx in self.col_start()..self.col_end() {
                    params_row.push(self.matrix.matrix[row_idx][col_idx].clone() * rhs.clone())
                }
                
                params.push(params_row)
            }
            
            Matrix::from(params)
        }
    }
}
pub(crate) use matrix_sliced_immut_scaled;

macro_rules! matrix_mut_scaled {
    ($rhs_type:ty) => {
        type Output = Self;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
            
            for row_idx in 0..self.rows {
                
                for col_idx in 0..self.cols {
                    self.matrix[row_idx][col_idx] = self.matrix[row_idx][col_idx].clone() * rhs.clone()
                }
                
            }
    
            self
        }
    }
}
pub(crate) use matrix_mut_scaled;

macro_rules! matrix_matrix_unsliced_immut_mul {
    ($rhs_type:ty) => {
        type Output = Matrix<T>;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
            if self.cols != rhs.rows {
                panic!("The left matrix row count is not equal to the right matrix column count.")
            }
    
            let mut params: Vec<Vec<T>> = vec![Vec::with_capacity(rhs.cols); self.rows];
    
            for row_index in 0..self.rows {
                for col_index in 0..rhs.cols {
                    let mut param_buffer = T::default();
                    
                    for index in 0..self.cols {
                        let lhs_row_ptr = self.matrix[row_index].as_ptr();
                        let lhs_value: T;
                        // SAFTEY: The initial test guarentees that valid memory will be pointed to.
                        unsafe {
                            lhs_value = (*lhs_row_ptr.add(index)).clone()
                        }
                        
                        let rhs_row_ptr = rhs.matrix[index].as_ptr();
                        let rhs_value: T;
                        // SAFTEY: The initial test guarentees that valid memory will be pointed to.
                        unsafe {
                            rhs_value = (*rhs_row_ptr.add(col_index)).clone()
                        }
                        
                        param_buffer = param_buffer + lhs_value * rhs_value
                    }
    
                    params[row_index].push(param_buffer)
                }
            }
            Matrix::from(params)
        }
    }
}
pub(crate) use matrix_matrix_unsliced_immut_mul;

macro_rules! matrix_matrix_left_sliced_immut_mul {
    ($rhs_type:ty) => {
        type Output = Matrix<T>;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
            if self.cols() != rhs.rows {
                panic!("The left matrix row count is not equal to the right matrix column count.")
            }
    
            let mut params: Vec<Vec<T>> = vec![Vec::with_capacity(rhs.cols); self.rows()];
    
            for row_index in 0..self.rows() {
                for col_index in 0..rhs.cols {
                    let mut param_buffer = T::default();
                    
                    for index in 0..self.cols() {
                        let lhs_row_ptr = self.matrix.matrix[row_index + self.row_start()].as_ptr();
                        let lhs_value: T;
                        // SAFTEY: The initial test guarentees that valid memory will be pointed to.
                        unsafe {
                            lhs_value = (*lhs_row_ptr.add(index + self.col_start())).clone()
                        }
                        
                        let rhs_row_ptr = rhs.matrix[index].as_ptr();
                        let rhs_value: T;
                        // SAFTEY: The initial test guarentees that valid memory will be pointed to.
                        unsafe {
                            rhs_value = (*rhs_row_ptr.add(col_index)).clone()
                        }
                        
                        param_buffer = param_buffer + lhs_value * rhs_value
                    }
    
                    params[row_index].push(param_buffer)
                }
            }
            Matrix::from(params)
        }
    }
}
pub(crate) use matrix_matrix_left_sliced_immut_mul;

macro_rules! matrix_matrix_right_sliced_immut_mul {
    ($rhs_type:ty) => {
        type Output = Matrix<T>;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
            if self.cols != rhs.rows() {
                panic!("The left matrix row count is not equal to the right matrix column count.")
            }
    
            let mut params: Vec<Vec<T>> = vec![Vec::with_capacity(rhs.cols()); self.rows];
    
            for row_index in 0..self.rows {
                for col_index in 0..rhs.cols() {
                    let mut param_buffer = T::default();
                    
                    for index in 0..self.cols {
                        let lhs_row_ptr = self.matrix[row_index].as_ptr();
                        let lhs_value: T;
                        // SAFTEY: The initial test guarentees that valid memory will be pointed to.
                        unsafe {
                            lhs_value = (*lhs_row_ptr.add(index)).clone()
                        }
                        
                        let rhs_row_ptr = rhs.matrix.matrix[index + rhs.row_start()].as_ptr();
                        let rhs_value: T;
                        // SAFTEY: The initial test guarentees that valid memory will be pointed to.
                        unsafe {
                            rhs_value = (*rhs_row_ptr.add(col_index + rhs.col_start())).clone()
                        }
                        
                        param_buffer = param_buffer + lhs_value * rhs_value
                    }
    
                    params[row_index].push(param_buffer)
                }
            }
            Matrix::from(params)
        }
    }
}
pub(crate) use matrix_matrix_right_sliced_immut_mul;

macro_rules! matrix_matrix_both_sliced_immut_mul {
    ($rhs_type:ty) => {
        type Output = Matrix<T>;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
            if self.cols() != rhs.rows() {
                panic!("The left matrix row count is not equal to the right matrix column count.")
            }
    
            let mut params: Vec<Vec<T>> = vec![Vec::with_capacity(rhs.cols()); self.rows()];
    
            for row_index in 0..self.rows() {
                for col_index in 0..rhs.cols() {
                    let mut param_buffer = T::default();
                    
                    for index in 0..self.cols() {
                        let lhs_row_ptr = self.matrix.matrix[row_index + self.row_start()].as_ptr();
                        let lhs_value: T;
                        // SAFTEY: The initial test guarentees that valid memory will be pointed to.
                        unsafe {
                            lhs_value = (*lhs_row_ptr.add(index + self.col_start())).clone()
                        }
                        
                        let rhs_row_ptr = rhs.matrix.matrix[index + rhs.row_start()].as_ptr();
                        let rhs_value: T;
                        // SAFTEY: The initial test guarentees that valid memory will be pointed to.
                        unsafe {
                            rhs_value = (*rhs_row_ptr.add(col_index + rhs.col_start())).clone()
                        }
                        
                        param_buffer = param_buffer + lhs_value * rhs_value
                    }
    
                    params[row_index].push(param_buffer)
                }
            }
            Matrix::from(params)
        }
    }
}
pub(crate) use matrix_matrix_both_sliced_immut_mul;

macro_rules! matrix_matrix_sliced_mut_mul {
    ($rhs_type:ty) => {
        type Output = Self;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
            if self.cols != rhs.rows() {
                panic!("The left matrix row count is not equal to the right matrix column count.")
            }
    
            for row_index in 0..self.rows {
                for col_index in 0..rhs.cols() {
                    let mut param_buffer = T::default();
                    
                    for index in 0..self.cols {
                        let lhs_row_ptr = self.matrix[row_index].as_ptr();
                        let lhs_value: T;
                        // SAFTEY: The initial test guarentees that valid memory will be pointed to.
                        unsafe {
                            lhs_value = (*lhs_row_ptr.add(index)).clone()
                        }
                        
                        let rhs_row_ptr = rhs.matrix.matrix[index + rhs.row_start()].as_ptr();
                        let rhs_value: T;
                        // SAFTEY: The initial test guarentees that valid memory will be pointed to.
                        unsafe {
                            rhs_value = (*rhs_row_ptr.add(col_index + rhs.col_start())).clone()
                        }
                        
                        param_buffer = param_buffer + lhs_value * rhs_value
                    }
    
                    self.matrix[row_index][col_index] = param_buffer
                }
            }
            self
        }
    }
}
pub(crate) use matrix_matrix_sliced_mut_mul;

macro_rules! matrix_matrix_unsliced_mut_mul {
    ($rhs_type:ty) => {
        type Output = Self;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
            if self.cols != rhs.rows {
                panic!("The left matrix row count is not equal to the right matrix column count.")
            }
    
            for row_index in 0..self.rows {
                for col_index in 0..rhs.cols {
                    let mut param_buffer = T::default();
                    
                    for index in 0..self.cols {
                        let lhs_row_ptr = self.matrix[row_index].as_ptr();
                        let lhs_value: T;
                        // SAFTEY: The initial test guarentees that valid memory will be pointed to.
                        unsafe {
                            lhs_value = (*lhs_row_ptr.add(index)).clone()
                        }
                        
                        let rhs_row_ptr = rhs.matrix[index].as_ptr();
                        let rhs_value: T;
                        // SAFTEY: The initial test guarentees that valid memory will be pointed to.
                        unsafe {
                            rhs_value = (*rhs_row_ptr.add(col_index)).clone()
                        }
                        
                        param_buffer = param_buffer + lhs_value * rhs_value
                    }
    
                    self.matrix[row_index][col_index] = param_buffer
                }
            }
            self
        }
    }
}
pub(crate) use matrix_matrix_unsliced_mut_mul;
