use std::ops::Add;
use crate::Matrix;
use crate::matrices::{MatrixBlock, MutMatrixBlock};
use crate::macros::add::matrix_both_sliced_immut_add;

impl<'l, 'r, T> Add<MatrixBlock<'r, T>> for MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_both_sliced_immut_add!(MatrixBlock<'r, T>);
}

impl<'l, 'r, T> Add<MatrixBlock<'r, T>> for &MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_both_sliced_immut_add!(MatrixBlock<'r, T>);
}

impl<'l, 'r, T> Add<&MatrixBlock<'r, T>> for MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_both_sliced_immut_add!(&MatrixBlock<'r, T>);
}

impl<'l, 'r, T> Add<&MatrixBlock<'r, T>> for &MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_both_sliced_immut_add!(&MatrixBlock<'r, T>);
}