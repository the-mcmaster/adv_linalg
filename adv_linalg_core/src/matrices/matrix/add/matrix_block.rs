use std::ops::Add;
use crate::Matrix;
use crate::matrices::MatrixBlock;
use crate::macros::add::matrix_right_sliced_immut_add;

impl<'r, T> Add<MatrixBlock<'r, T>> for Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(MatrixBlock<'r, T>);
}

impl<'r, T> Add<MatrixBlock<'r, T>> for &Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(MatrixBlock<'r, T>);
}

impl<'r, T> Add<&MatrixBlock<'r, T>> for Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(&MatrixBlock<'r, T>);
}

impl<'r, T> Add<&MatrixBlock<'r, T>> for &Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(&MatrixBlock<'r, T>);
}