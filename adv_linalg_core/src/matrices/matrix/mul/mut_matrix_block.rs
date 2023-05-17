use std::ops::{Mul, Add};
use crate::Matrix;
use crate::matrices::MutMatrixBlock;
use crate::macros::mul::matrix_matrix_right_sliced_immut_mul;

impl<'r, T> Mul<MutMatrixBlock<'r, T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_right_sliced_immut_mul!(MutMatrixBlock<'r, T>);
}

impl<'r, T> Mul<MutMatrixBlock<'r, T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_right_sliced_immut_mul!(MutMatrixBlock<'r, T>);
}

impl<'r, T> Mul<&MutMatrixBlock<'r, T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_right_sliced_immut_mul!(&MutMatrixBlock<'r, T>);
}

impl<'r, T> Mul<&MutMatrixBlock<'r, T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_right_sliced_immut_mul!(&MutMatrixBlock<'r, T>);
}