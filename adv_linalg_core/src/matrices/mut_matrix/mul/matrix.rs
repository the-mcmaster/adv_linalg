use std::ops::{Add, Mul};
use crate::Matrix;
use crate::matrices::MutMatrix;
use crate::macros::mul::{matrix_matrix_unsliced_immut_mul, matrix_matrix_unsliced_mut_mul};

// -----Mutable Multiplication-----
impl<'l, T> Mul<Matrix<T>> for &'l mut MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_unsliced_mut_mul!(Matrix<T>);
}

impl<'l, T> Mul<&Matrix<T>> for &'l mut MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_unsliced_mut_mul!(&Matrix<T>);
}

// -----Immutable Multiplication-----
impl<T> Mul<Matrix<T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(Matrix<T>);
}

impl<T> Mul<Matrix<T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(Matrix<T>);
}

impl<T> Mul<&Matrix<T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(&Matrix<T>);
}

impl<T> Mul<&Matrix<T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(&Matrix<T>);
}