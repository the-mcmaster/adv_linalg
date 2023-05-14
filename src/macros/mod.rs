pub(crate) mod add;
pub(crate) mod mul;
pub(crate) mod sub;

#[macro_export]
macro_rules! vector {
    //no arguments case
    () => {
        adv_linalg_core::vector_impl::Vector::from(vec![])
    };

    //repeat some elements some n times
    ($x:expr; $n:expr) => {
        adv_linalg_core::vectors::Vector::from(vec![$x; $n])
    };
    
    //match each comma-separated argument
    //and allow the last comma to be ignored
    ($($x:expr),*) => {
        adv_linalg_core::vectors::Vector::from(vec![$($x),*])
    };

    //match each comma-separated argument
    //but an unneccesary comma was used at the end
    ($($x:expr,)*) => {
        adv_linalg_core::vectors::Vector::from(vec![$($x),*])
    }
}

#[macro_export]
macro_rules! matrix {
    //no arguments case
    () => {
        adv_linalg_core::matrices::Matrix::from(vec![])
    };

    //repeat some list of elements some n times
    ($x:tt; $n:expr) => {
        adv_linalg_core::matrices::Matrix::from(vec![vec!$x; $n])
    };

    //match each comma-separated argument
    //and allow the last comma to be ignored
    ($($x:tt),*) => {
        adv_linalg_core::matrices::Matrix::from(
            vec![
                $(vec!$x),*
            ]
        )
    };

    //match each comma-separated argument
    //but an unneccesary comma was used at the end
    ($($x:tt,)*) => {
        adv_linalg_core::matrices::Matrix::from(
            vec![
                $(vec!$x),*
            ]
        )
    }
}