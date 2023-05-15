use std::ops::Add;
use crate::Matrix;
use crate::matrices::{MutMatrix, MatrixBlock};
use crate::macros::add::matrix_left_sliced_immut_add;

impl<'l, T> Add<MutMatrix<T>> for MatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_left_sliced_immut_add!(MutMatrix<T>);
}

impl<'l, T> Add<MutMatrix<T>> for &MatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_left_sliced_immut_add!(MutMatrix<T>);
}

impl<'l, T> Add<&MutMatrix<T>> for MatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_left_sliced_immut_add!(&MutMatrix<T>);
}

impl<'l, T> Add<&MutMatrix<T>> for &MatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_left_sliced_immut_add!(&MutMatrix<T>);
}

impl<'l, 'r, T> Add<&'r mut MutMatrix<T>> for MatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_left_sliced_immut_add!(&'r mut MutMatrix<T>);
}

impl<'l, 'r, T> Add<&'r mut MutMatrix<T>> for &MatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_left_sliced_immut_add!(&'r mut MutMatrix<T>);
}