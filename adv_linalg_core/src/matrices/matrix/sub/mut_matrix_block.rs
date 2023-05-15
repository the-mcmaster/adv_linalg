use std::ops::Sub;
use crate::Matrix;
use crate::matrices::MutMatrixBlock;
use crate::macros::sub::matrix_right_sliced_immut_sub;

impl<'r, T> Sub<MutMatrixBlock<'r, T>> for Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(MutMatrixBlock<'r, T>);
}

impl<'r, T> Sub<MutMatrixBlock<'r, T>> for &Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(MutMatrixBlock<'r, T>);
}

impl<'r, T> Sub<&MutMatrixBlock<'r, T>> for Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(&MutMatrixBlock<'r, T>);
}

impl<'r, T> Sub<&MutMatrixBlock<'r, T>> for &Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(&MutMatrixBlock<'r, T>);
}