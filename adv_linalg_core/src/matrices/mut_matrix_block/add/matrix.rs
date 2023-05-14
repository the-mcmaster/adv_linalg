use std::ops::Add;
use crate::Matrix;
use crate::matrices::MutMatrixBlock;
use crate::macros::add::matrix_left_sliced_immut_add;

impl<'l, T> Add<Matrix<T>> for MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_left_sliced_immut_add!(Matrix<T>);
}

impl<'l, T> Add<Matrix<T>> for &MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_left_sliced_immut_add!(Matrix<T>);
}

impl<'l, T> Add<&Matrix<T>> for MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_left_sliced_immut_add!(&Matrix<T>);
}

impl<'l, T> Add<&Matrix<T>> for &MutMatrixBlock<'l, T>
where
    T: Clone + Add<Output = T>
{
    matrix_left_sliced_immut_add!(&Matrix<T>);
}