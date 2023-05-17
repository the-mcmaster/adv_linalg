use std::ops::{Add, Mul};
use crate::Matrix;
use crate::matrices::{MutMatrix, MutMatrixBlock};
use crate::macros::mul::matrix_matrix_left_sliced_immut_mul;

impl<'l, T> Mul<MutMatrix<T>> for MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(MutMatrix<T>);
}

impl<'l, T> Mul<MutMatrix<T>> for &MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(MutMatrix<T>);
}

impl<'l, T> Mul<&MutMatrix<T>> for MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(&MutMatrix<T>);
}

impl<'l, T> Mul<&MutMatrix<T>> for &MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(&MutMatrix<T>);
}

impl<'l, T> Mul<&mut MutMatrix<T>> for MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(&mut MutMatrix<T>);
}

impl<'l, T> Mul<&mut MutMatrix<T>> for &MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(&mut MutMatrix<T>);
}