use std::ops::Sub;
use crate::Matrix;
use crate::matrices::{MutMatrixBlock, MatrixBlock};
use crate::macros::sub::matrix_both_sliced_immut_sub;

impl<'l, 'r, T> Sub<MutMatrixBlock<'r, T>> for MatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_both_sliced_immut_sub!(MutMatrixBlock<'r, T>);
}

impl<'l, 'r, T> Sub<MutMatrixBlock<'r, T>> for &MatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_both_sliced_immut_sub!(MutMatrixBlock<'r, T>);
}

impl<'l, 'r, T> Sub<&MutMatrixBlock<'r, T>> for MatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_both_sliced_immut_sub!(&MutMatrixBlock<'r, T>);
}

impl<'l, 'r, T> Sub<&MutMatrixBlock<'r, T>> for &MatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_both_sliced_immut_sub!(&MutMatrixBlock<'r, T>);
}