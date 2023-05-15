use std::ops::Mul;
use crate::Matrix;
use crate::matrices::MatrixBlock;
use crate::macros::mul::matrix_sliced_immut_scaled;

impl<'l, T> Mul<T> for MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T>
{
    matrix_sliced_immut_scaled!(T);
}

impl<'l, T> Mul<T> for &MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T>
{
    matrix_sliced_immut_scaled!(T);
}

impl<'l, T> Mul<&T> for MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T>
{
    matrix_sliced_immut_scaled!(&T);
}

impl<'l, T> Mul<&T> for &MatrixBlock<'l, T>
where
    T: Clone + Mul<Output = T>
{
    matrix_sliced_immut_scaled!(&T);
}