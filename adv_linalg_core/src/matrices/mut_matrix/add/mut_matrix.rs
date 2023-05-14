use std::ops::Add;
use crate::Matrix;
use crate::matrices::MutMatrix;
use crate::macros::add::{matrix_unsliced_immut_add, matrix_unsliced_mut_add};

// -----Mut Adds-----
impl<'l, T> Add<MutMatrix<T>> for &'l mut MutMatrix<T>
where
    T: Clone +  Add<Output = T>
{
    matrix_unsliced_mut_add!(MutMatrix<T>);
}

impl<'l, T> Add<&MutMatrix<T>> for &'l mut MutMatrix<T>
where
    T: Clone +  Add<Output = T>
{
    matrix_unsliced_mut_add!(&MutMatrix<T>);
}

impl<'l, 'r, T> Add<&'r mut MutMatrix<T>> for &'l mut MutMatrix<T>
where
    T: Clone +  Add<Output = T>
{
    matrix_unsliced_mut_add!(&'r mut MutMatrix<T>);
}

// -----Immut Adds-----
impl<T> Add<MutMatrix<T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(MutMatrix<T>);
}

impl<T> Add<MutMatrix<T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(MutMatrix<T>);
}

impl<T> Add<&MutMatrix<T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(&MutMatrix<T>);
}

impl<T> Add<&MutMatrix<T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(&MutMatrix<T>);
}

impl<'r, T> Add<&'r mut MutMatrix<T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(&'r mut MutMatrix<T>);
}

impl<'r, T> Add<&'r mut MutMatrix<T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(&'r mut MutMatrix<T>);
}