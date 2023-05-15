use std::ops::Sub;
use crate::Matrix;
use crate::matrices::{MatrixBlock, MutMatrixBlock};
use crate::macros::sub::matrix_both_sliced_immut_sub;

impl<'l, 'r, T> Sub<MatrixBlock<'r, T>> for MutMatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_both_sliced_immut_sub!(MatrixBlock<'r, T>);
}

impl<'l, 'r, T> Sub<MatrixBlock<'r, T>> for &MutMatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_both_sliced_immut_sub!(MatrixBlock<'r, T>);
}

impl<'l, 'r, T> Sub<&MatrixBlock<'r, T>> for MutMatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_both_sliced_immut_sub!(&MatrixBlock<'r, T>);
}

impl<'l, 'r, T> Sub<&MatrixBlock<'r, T>> for &MutMatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_both_sliced_immut_sub!(&MatrixBlock<'r, T>);
}