use std::ops::{Add, Mul};
use crate::Matrix;
use crate::matrices::MutMatrix;
use crate::macros::mul::{matrix_matrix_unsliced_immut_mul, matrix_matrix_unsliced_mut_mul};

// -----Mutable Multiplication-----
impl<'l, T> Mul<MutMatrix<T>> for &'l mut MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_unsliced_mut_mul!(MutMatrix<T>);
}

impl<'l, T> Mul<&MutMatrix<T>> for &'l mut MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_unsliced_mut_mul!(&MutMatrix<T>);
}

impl<'l, T> Mul<&mut MutMatrix<T>> for &'l mut MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_unsliced_mut_mul!(&mut MutMatrix<T>);
}

// -----Immutable Multiplication-----
impl<T> Mul<MutMatrix<T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(MutMatrix<T>);
}

impl<T> Mul<MutMatrix<T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(MutMatrix<T>);
}

impl<T> Mul<&MutMatrix<T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(&MutMatrix<T>);
}

impl<T> Mul<&MutMatrix<T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(&MutMatrix<T>);
}

impl<T> Mul<&mut MutMatrix<T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(&mut MutMatrix<T>);
}

impl<T> Mul<&mut MutMatrix<T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + Default
{
    matrix_matrix_unsliced_immut_mul!(&mut MutMatrix<T>);
}