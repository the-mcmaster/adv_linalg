use std::ops::Sub;
use crate::Matrix;
use crate::matrices::MutMatrix;
use crate::macros::sub::{matrix_unsliced_immut_sub, matrix_unsliced_mut_sub};

// -----Mut Subtractions-----
impl<'l, T> Sub<MutMatrix<T>> for &'l mut MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_mut_sub!(MutMatrix<T>);
}

impl<'l, T> Sub<&MutMatrix<T>> for &'l mut MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_mut_sub!(&MutMatrix<T>);
}

impl<'l, 'r, T> Sub<&'r mut MutMatrix<T>> for &'l mut MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_mut_sub!(&'r mut MutMatrix<T>);
}

// -----Immut Subtractions-----
impl<T> Sub<MutMatrix<T>> for MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(MutMatrix<T>);
}

impl<T> Sub<MutMatrix<T>> for &MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(MutMatrix<T>);
}

impl<T> Sub<&MutMatrix<T>> for MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(&MutMatrix<T>);
}

impl<T> Sub<&MutMatrix<T>> for &MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(&MutMatrix<T>);
}

impl<'r, T> Sub<&'r mut MutMatrix<T>> for MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(&'r mut MutMatrix<T>);
}

impl<'r, T> Sub<&'r mut MutMatrix<T>> for &MutMatrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(&'r mut MutMatrix<T>);
}