pub(crate) mod add;

#[macro_export]
macro_rules! vector {
    //no arguments case
    () => {
        adv_linalg::vector_impl::Vector::from(vec![])
    };

    //repeat some elements some n times
    ($x:expr; $n:expr) => {
        adv_linalg::vectors::Vector::from(vec![$x; $n])
    };
    
    //match each comma-separated argument
    //and allow the last comma to be ignored
    ($($x:expr),*) => {
        adv_linalg::vectors::Vector::from(vec![$($x),*])
    };

    //match each comma-separated argument
    //but an unneccesary comma was used at the end
    ($($x:expr,)*) => {
        adv_linalg::vectors::Vector::from(vec![$($x),*])
    }
}

#[macro_export]
macro_rules! matrix {
    //no arguments case
    () => {
        adv_linalg::matrices::Matrix::from(vec![])
    };

    //repeat some list of elements some n times
    ($x:tt; $n:expr) => {
        adv_linalg::matrices::Matrix::from(vec![vec!$x; $n])
    };

    //match each comma-separated argument
    //and allow the last comma to be ignored
    ($($x:tt),*) => {
        adv_linalg::matrices::Matrix::from(
            vec![
                $(vec!$x),*
            ]
        )
    };

    //match each comma-separated argument
    //but an unneccesary comma was used at the end
    ($($x:tt,)*) => {
        adv_linalg::matrices::Matrix::from(
            vec![
                $(vec!$x),*
            ]
        )
    }
}

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