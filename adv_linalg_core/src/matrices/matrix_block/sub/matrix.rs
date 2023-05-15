use std::ops::Sub;
use crate::Matrix;
use crate::matrices::MatrixBlock;
use crate::macros::sub::matrix_left_sliced_immut_sub;

impl<'l, T> Sub<Matrix<T>> for MatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_left_sliced_immut_sub!(Matrix<T>);
}

impl<'l, T> Sub<Matrix<T>> for &MatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_left_sliced_immut_sub!(Matrix<T>);
}

impl<'l, T> Sub<&Matrix<T>> for MatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_left_sliced_immut_sub!(&Matrix<T>);
}

impl<'l, T> Sub<&Matrix<T>> for &MatrixBlock<'l, T>
where
    T: Clone + Sub<Output = T>
{
    matrix_left_sliced_immut_sub!(&Matrix<T>);
}