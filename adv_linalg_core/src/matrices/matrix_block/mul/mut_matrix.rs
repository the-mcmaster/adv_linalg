use std::ops::{Add, Mul};
use crate::Matrix;
use crate::matrices::{MutMatrix, MatrixBlock};
use crate::macros::mul::matrix_matrix_left_sliced_immut_mul;

impl<'l, T> Mul<MutMatrix<T>> for MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(MutMatrix<T>);
}

impl<'l, T> Mul<MutMatrix<T>> for &MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(MutMatrix<T>);
}

impl<'l, T> Mul<&MutMatrix<T>> for MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(&MutMatrix<T>);
}

impl<'l, T> Mul<&MutMatrix<T>> for &MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(&MutMatrix<T>);
}

impl<'l, T> Mul<&mut MutMatrix<T>> for MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(&mut MutMatrix<T>);
}

impl<'l, T> Mul<&mut MutMatrix<T>> for &MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_left_sliced_immut_mul!(&mut MutMatrix<T>);
}