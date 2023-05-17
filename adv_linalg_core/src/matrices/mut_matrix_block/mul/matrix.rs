use std::ops::{Add, Mul};
use crate::Matrix;
use crate::matrices::MutMatrixBlock;
use crate::macros::mul::matrix_matrix_left_sliced_immut_mul;

impl<'l, T> Mul<Matrix<T>> for MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(Matrix<T>);
}

impl<'l, T> Mul<Matrix<T>> for &MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(Matrix<T>);
}

impl<'l, T> Mul<&Matrix<T>> for MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(&Matrix<T>);
}

impl<'l, T> Mul<&Matrix<T>> for &MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(&Matrix<T>);
}