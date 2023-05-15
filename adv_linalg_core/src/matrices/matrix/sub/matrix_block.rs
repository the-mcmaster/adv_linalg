use std::ops::Sub;
use crate::Matrix;
use crate::matrices::MatrixBlock;
use crate::macros::sub::matrix_right_sliced_immut_sub;

impl<'r, T> Sub<MatrixBlock<'r, T>> for Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(MatrixBlock<'r, T>);
}

impl<'r, T> Sub<MatrixBlock<'r, T>> for &Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(MatrixBlock<'r, T>);
}

impl<'r, T> Sub<&MatrixBlock<'r, T>> for Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(&MatrixBlock<'r, T>);
}

impl<'r, T> Sub<&MatrixBlock<'r, T>> for &Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(&MatrixBlock<'r, T>);
}