use std::ops::Mul;
use crate::Matrix;
use crate::macros::mul::matrix_unsliced_immut_scaled;

impl<T> Mul<T> for Matrix<T>
where
    T: Clone + Mul<Output = T>
{
    matrix_unsliced_immut_scaled!(T);
}

impl<T> Mul<T> for &Matrix<T>
where
    T: Clone + Mul<Output = T>
{
    matrix_unsliced_immut_scaled!(T);
}

impl<T> Mul<&T> for Matrix<T>
where
    T: Clone + Mul<Output = T>
{
    matrix_unsliced_immut_scaled!(&T);
}

impl<T> Mul<&T> for &Matrix<T>
where
    T: Clone + Mul<Output = T>
{
    matrix_unsliced_immut_scaled!(&T);
}