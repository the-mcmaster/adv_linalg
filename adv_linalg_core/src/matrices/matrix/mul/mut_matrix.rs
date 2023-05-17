use std::ops::{Mul, Add};
use crate::Matrix;
use crate::matrices::MutMatrix;
use crate::macros::mul::matrix_matrix_unsliced_immut_mul;

impl<T> Mul<MutMatrix<T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(MutMatrix<T>);
}

impl<T> Mul<MutMatrix<T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(MutMatrix<T>);
}

impl<T> Mul<&MutMatrix<T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(&MutMatrix<T>);
}

impl<T> Mul<&MutMatrix<T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(&MutMatrix<T>);
}

impl<T> Mul<&mut MutMatrix<T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(&mut MutMatrix<T>);
}

impl<T> Mul<&mut MutMatrix<T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(&mut MutMatrix<T>);
}