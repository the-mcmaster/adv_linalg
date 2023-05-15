use std::ops::Sub;
use crate::Matrix;
use crate::matrices::{MatrixBlock, MutMatrix};
use crate::macros::sub::{matrix_right_sliced_immut_sub, matrix_sliced_mut_sub};

// -----Mut Subtractions-----
impl<'r, 'l, T> Sub<MatrixBlock<'r, T>> for &'l mut MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_sliced_mut_sub!(MatrixBlock<'r, T>);
}

impl<'r, 'l, T> Sub<&MatrixBlock<'r, T>> for &'l mut MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_sliced_mut_sub!(&MatrixBlock<'r, T>);
}

// -----Immut Subtractions-----
impl<'r, T> Sub<MatrixBlock<'r, T>> for MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(MatrixBlock<'r, T>);
}

impl<'r, T> Sub<MatrixBlock<'r, T>> for &MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(MatrixBlock<'r, T>);
}

impl<'r, T> Sub<&MatrixBlock<'r, T>> for MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(&MatrixBlock<'r, T>);
}

impl<'r, T> Sub<&MatrixBlock<'r, T>> for &MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(&MatrixBlock<'r, T>);
}