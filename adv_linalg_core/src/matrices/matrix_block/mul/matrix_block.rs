use std::ops::{Add, Mul};
use crate::Matrix;
use crate::matrices::MatrixBlock;
use crate::macros::mul::matrix_matrix_both_sliced_immut_mul;

impl<'l, 'r, T> Mul<MatrixBlock<'r, T>> for MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_both_sliced_immut_mul!(MatrixBlock<'r, T>);
}

impl<'l, 'r, T> Mul<MatrixBlock<'r, T>> for &MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_both_sliced_immut_mul!(MatrixBlock<'r, T>);
}

impl<'l, 'r, T> Mul<&MatrixBlock<'r, T>> for MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_both_sliced_immut_mul!(&MatrixBlock<'r, T>);
}

impl<'l, 'r, T> Mul<&MatrixBlock<'r, T>> for &MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_both_sliced_immut_mul!(&MatrixBlock<'r, T>);
}