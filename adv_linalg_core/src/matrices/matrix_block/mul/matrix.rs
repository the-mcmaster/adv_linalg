use std::ops::{Add, Mul};
use crate::Matrix;
use crate::matrices::MatrixBlock;
use crate::macros::mul::matrix_matrix_left_sliced_immut_mul;

impl<'l, T> Mul<Matrix<T>> for MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(Matrix<T>);
}

impl<'l, T> Mul<Matrix<T>> for &MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(Matrix<T>);
}

impl<'l, T> Mul<&Matrix<T>> for MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(&Matrix<T>);
}

impl<'l, T> Mul<&Matrix<T>> for &MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(&Matrix<T>);
}