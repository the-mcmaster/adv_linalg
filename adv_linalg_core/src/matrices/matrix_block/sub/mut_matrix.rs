use std::ops::Sub;
use crate::Matrix;
use crate::matrices::{MutMatrix, MatrixBlock};
use crate::macros::sub::matrix_left_sliced_immut_sub;

impl<'l, T> Sub<MutMatrix<T>> for MatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_left_sliced_immut_sub!(MutMatrix<T>);
}

impl<'l, T> Sub<MutMatrix<T>> for &MatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_left_sliced_immut_sub!(MutMatrix<T>);
}

impl<'l, T> Sub<&MutMatrix<T>> for MatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_left_sliced_immut_sub!(&MutMatrix<T>);
}

impl<'l, T> Sub<&MutMatrix<T>> for &MatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_left_sliced_immut_sub!(&MutMatrix<T>);
}

impl<'l, 'r, T> Sub<&'r mut MutMatrix<T>> for MatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_left_sliced_immut_sub!(&'r mut MutMatrix<T>);
}

impl<'l, 'r, T> Sub<&'r mut MutMatrix<T>> for &MatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_left_sliced_immut_sub!(&'r mut MutMatrix<T>);
}