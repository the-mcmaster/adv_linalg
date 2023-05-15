use std::ops::Sub;
use crate::Matrix;
use crate::macros::sub::matrix_unsliced_immut_sub;

impl<T> Sub<Matrix<T>> for Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(Matrix<T>);
}

impl<T> Sub<Matrix<T>> for &Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(Matrix<T>);
}

impl<T> Sub<&Matrix<T>> for Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(&Matrix<T>);
}

impl<T> Sub<&Matrix<T>> for &Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(&Matrix<T>);
}