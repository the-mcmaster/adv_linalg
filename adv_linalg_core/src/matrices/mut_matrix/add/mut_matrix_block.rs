use std::ops::Add;
use crate::Matrix;
use crate::matrices::{MutMatrixBlock, MutMatrix};
use crate::macros::add::{matrix_right_sliced_immut_add, matrix_sliced_mut_add};

// -----Mut Adds-----
impl<'l, 'r, T> Add<MutMatrixBlock<'r, T>> for &'l mut MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_sliced_mut_add!(MutMatrixBlock<'r, T>);
}

impl<'l, 'r, T> Add<&MutMatrixBlock<'r, T>> for &'l mut MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_sliced_mut_add!(&MutMatrixBlock<'r, T>);
}

// -----Immut Adds-----
impl<'r, T> Add<MutMatrixBlock<'r, T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(MutMatrixBlock<'r, T>);
}

impl<'r, T> Add<MutMatrixBlock<'r, T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(MutMatrixBlock<'r, T>);
}

impl<'r, T> Add<&MutMatrixBlock<'r, T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(&MutMatrixBlock<'r, T>);
}

impl<'r, T> Add<&MutMatrixBlock<'r, T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_right_sliced_immut_add!(&MutMatrixBlock<'r, T>);
}