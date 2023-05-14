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