use std::ops::{Add, Mul};
use crate::Matrix;
use crate::matrices::{MutMatrix, MatrixBlock};
use crate::macros::mul::{matrix_matrix_right_sliced_immut_mul, matrix_matrix_sliced_mut_mul};

// -----Mutable Multiplication-----
impl<'l, 'r, T> Mul<MatrixBlock<'r, T>> for &'l mut MutMatrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_sliced_mut_mul!(MatrixBlock<'r, T>);
}

impl<'l, 'r, T> Mul<&MatrixBlock<'r, T>> for &'l mut MutMatrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_sliced_mut_mul!(&MatrixBlock<'r, T>);
}

// -----Immutable Multiplication
impl<'r, T> Mul<MatrixBlock<'r, T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_right_sliced_immut_mul!(MatrixBlock<'r, T>);
}

impl<'r, T> Mul<MatrixBlock<'r, T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_right_sliced_immut_mul!(MatrixBlock<'r, T>);
}

impl<'r, T> Mul<&MatrixBlock<'r, T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_right_sliced_immut_mul!(&MatrixBlock<'r, T>);
}

impl<'r, T> Mul<&MatrixBlock<'r, T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_right_sliced_immut_mul!(&MatrixBlock<'r, T>);
}