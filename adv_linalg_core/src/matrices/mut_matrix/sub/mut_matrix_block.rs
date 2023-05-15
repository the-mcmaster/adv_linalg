use std::ops::Sub;
use crate::Matrix;
use crate::matrices::{MutMatrixBlock, MutMatrix};
use crate::macros::sub::{matrix_right_sliced_immut_sub, matrix_sliced_mut_sub};

// -----Mut Subtractions-----
impl<'l, 'r, T> Sub<MutMatrixBlock<'r, T>> for &'l mut MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_sliced_mut_sub!(MutMatrixBlock<'r, T>);
}

impl<'l, 'r, T> Sub<&MutMatrixBlock<'r, T>> for &'l mut MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_sliced_mut_sub!(&MutMatrixBlock<'r, T>);
}

// -----Immut Subtractions-----
impl<'r, T> Sub<MutMatrixBlock<'r, T>> for MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(MutMatrixBlock<'r, T>);
}

impl<'r, T> Sub<MutMatrixBlock<'r, T>> for &MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(MutMatrixBlock<'r, T>);
}

impl<'r, T> Sub<&MutMatrixBlock<'r, T>> for MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(&MutMatrixBlock<'r, T>);
}

impl<'r, T> Sub<&MutMatrixBlock<'r, T>> for &MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_right_sliced_immut_sub!(&MutMatrixBlock<'r, T>);
}