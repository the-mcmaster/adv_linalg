use std::ops::{Mul, Add};
use crate::Matrix;
use crate::macros::mul::matrix_matrix_unsliced_immut_mul;

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(Matrix<T>);
}

impl<T> Mul<Matrix<T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(Matrix<T>);
}

impl<T> Mul<&Matrix<T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(&Matrix<T>);
}

impl<T> Mul<&Matrix<T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(&Matrix<T>);
}