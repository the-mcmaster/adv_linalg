macro_rules! vector_unsliced_immut_mul {
    ($rhs_type:ty) => {
        type Output = T;

        fn mul(self, rhs: $rhs_type) -> Self::Output {
            if self.len() != rhs.len() {
                panic!("Cannot find dot product of two differently sized vectors.")
            }
    
            let mut product = T::default();
            
            for idx in 0..self.len() {
                product += self.list[idx] * rhs.list[idx]
            }
    
            product
        }
    }
}
pub(crate) use vector_unsliced_immut_mul;