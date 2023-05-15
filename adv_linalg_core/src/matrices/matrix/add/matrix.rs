use std::ops::Add;
use crate::Matrix;
use crate::macros::add::matrix_unsliced_immut_add;

impl<T> Add<Matrix<T>> for Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(Matrix<T>);
}

impl<T> Add<Matrix<T>> for &Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(Matrix<T>);
}

impl<T> Add<&Matrix<T>> for Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(&Matrix<T>);
}

impl<T> Add<&Matrix<T>> for &Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(&Matrix<T>);
}