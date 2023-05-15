use std::ops::Add;
use crate::Matrix;
use crate::matrices::MutMatrix;
use crate::macros::add::matrix_unsliced_immut_add;

impl<T> Add<MutMatrix<T>> for Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(MutMatrix<T>);
}

impl<T> Add<MutMatrix<T>> for &Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(MutMatrix<T>);
}

impl<T> Add<&MutMatrix<T>> for Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(&MutMatrix<T>);
}

impl<T> Add<&MutMatrix<T>> for &Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(&MutMatrix<T>);
}

impl<'r, T> Add<&'r mut MutMatrix<T>> for Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(&'r mut MutMatrix<T>);
}

impl<'r, T> Add<&'r mut MutMatrix<T>> for &Matrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(&'r mut MutMatrix<T>);
}