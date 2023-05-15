use std::ops::Sub;
use crate::Matrix;
use crate::matrices::MutMatrix;
use crate::macros::sub::{matrix_unsliced_immut_sub, matrix_unsliced_mut_sub};

// -----Mut Subtractions-----
impl<'r, T> Sub<Matrix<T>> for &'r mut MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_mut_sub!(Matrix<T>);
}

impl<'r, T> Sub<&Matrix<T>> for &'r mut MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_mut_sub!(&Matrix<T>);
}

// -----Immut Subtractions-----
impl<T> Sub<Matrix<T>> for MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(Matrix<T>);
}

impl<T> Sub<Matrix<T>> for &MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(Matrix<T>);
}

impl<T> Sub<&Matrix<T>> for MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(&Matrix<T>);
}

impl<T> Sub<&Matrix<T>> for &MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(&Matrix<T>);
}