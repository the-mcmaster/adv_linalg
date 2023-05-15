use std::ops::Add;
use crate::Matrix;
use crate::matrices::{MutMatrixBlock, MatrixBlock};
use crate::macros::add::matrix_both_sliced_immut_add;

impl<'l, 'r, T> Add<MutMatrixBlock<'r, T>> for MatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_both_sliced_immut_add!(MutMatrixBlock<'r, T>);
}

impl<'l, 'r, T> Add<MutMatrixBlock<'r, T>> for &MatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_both_sliced_immut_add!(MutMatrixBlock<'r, T>);
}

impl<'l, 'r, T> Add<&MutMatrixBlock<'r, T>> for MatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_both_sliced_immut_add!(&MutMatrixBlock<'r, T>);
}

impl<'l, 'r, T> Add<&MutMatrixBlock<'r, T>> for &MatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_both_sliced_immut_add!(&MutMatrixBlock<'r, T>);
}