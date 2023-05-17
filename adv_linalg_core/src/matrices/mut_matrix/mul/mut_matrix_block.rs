use std::ops::{Add, Mul};
use crate::Matrix;
use crate::matrices::{MutMatrix, MutMatrixBlock};
use crate::macros::mul::{matrix_matrix_right_sliced_immut_mul, matrix_matrix_sliced_mut_mul};

// -----Mutable Multiplication-----
impl<'l, 'r, T> Mul<MutMatrixBlock<'r, T>> for &'l mut MutMatrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_sliced_mut_mul!(MutMatrixBlock<'r, T>);
}

impl<'l, 'r, T> Mul<&MutMatrixBlock<'r, T>> for &'l mut MutMatrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_sliced_mut_mul!(&MutMatrixBlock<'r, T>);
}

// -----Immutable Multiplication
impl<'r, T> Mul<MutMatrixBlock<'r, T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_right_sliced_immut_mul!(MutMatrixBlock<'r, T>);
}

impl<'r, T> Mul<MutMatrixBlock<'r, T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_right_sliced_immut_mul!(MutMatrixBlock<'r, T>);
}

impl<'r, T> Mul<&MutMatrixBlock<'r, T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_right_sliced_immut_mul!(&MutMatrixBlock<'r, T>);
}

impl<'r, T> Mul<&MutMatrixBlock<'r, T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_right_sliced_immut_mul!(&MutMatrixBlock<'r, T>);
}