use std::ops::Add;
use crate::Matrix;
use crate::matrices::{MatrixBlock, MutMatrix};
use crate::macros::add::{matrix_right_sliced_immut_add, matrix_sliced_mut_add};

// -----Mut Adds-----
impl<'r, 'l, T> Add<MatrixBlock<'r, T>> for &'l mut MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_sliced_mut_add!(MatrixBlock<'r, T>);
}

impl<'r, 'l, T> Add<&MatrixBlock<'r, T>> for &'l mut MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_sliced_mut_add!(&MatrixBlock<'r, T>);
}

// -----Immut Adds-----
impl<'r, T> Add<MatrixBlock<'r, T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(MatrixBlock<'r, T>);
}

impl<'r, T> Add<MatrixBlock<'r, T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(MatrixBlock<'r, T>);
}

impl<'r, T> Add<&MatrixBlock<'r, T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(&MatrixBlock<'r, T>);
}

impl<'r, T> Add<&MatrixBlock<'r, T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(&MatrixBlock<'r, T>);
}