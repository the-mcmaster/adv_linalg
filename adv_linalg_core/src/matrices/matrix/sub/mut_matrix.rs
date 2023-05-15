use std::ops::Sub;
use crate::Matrix;
use crate::matrices::MutMatrix;
use crate::macros::sub::matrix_unsliced_immut_sub;

impl<T> Sub<MutMatrix<T>> for Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(MutMatrix<T>);
}

impl<T> Sub<MutMatrix<T>> for &Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(MutMatrix<T>);
}

impl<T> Sub<&MutMatrix<T>> for Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(&MutMatrix<T>);
}

impl<T> Sub<&MutMatrix<T>> for &Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(&MutMatrix<T>);
}

impl<'r, T> Sub<&'r mut MutMatrix<T>> for Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(&'r mut MutMatrix<T>);
}

impl<'r, T> Sub<&'r mut MutMatrix<T>> for &Matrix<T>
where
    T: Clone + Sub<Output = T>
{
    matrix_unsliced_immut_sub!(&'r mut MutMatrix<T>);
}