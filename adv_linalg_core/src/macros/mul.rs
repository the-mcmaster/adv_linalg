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