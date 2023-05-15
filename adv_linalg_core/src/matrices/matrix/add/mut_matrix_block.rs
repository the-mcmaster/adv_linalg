use std::ops::Add;
use crate::Matrix;
use crate::matrices::MutMatrixBlock;
use crate::macros::add::matrix_right_sliced_immut_add;

impl<'r, T> Add<MutMatrixBlock<'r, T>> for Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(MutMatrixBlock<'r, T>);
}

impl<'r, T> Add<MutMatrixBlock<'r, T>> for &Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(MutMatrixBlock<'r, T>);
}

impl<'r, T> Add<&MutMatrixBlock<'r, T>> for Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(&MutMatrixBlock<'r, T>);
}

impl<'r, T> Add<&MutMatrixBlock<'r, T>> for &Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(&MutMatrixBlock<'r, T>);
}