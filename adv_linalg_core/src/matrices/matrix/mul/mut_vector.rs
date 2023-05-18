use std::ops::{Mul, Add};
use crate::{Vector, Matrix};
use crate::vectors::MutVector;
use crate::macros::mul::{matrix_vector_unsliced_immut_mul, matrix_vector_unsliced_mut_mul};

// -----Mutable Multiplication-----
impl<'r, T> Mul<&'r mut MutVector<T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_unsliced_mut_mul!(&'r mut MutVector<T>);
}

impl<'r, T> Mul<&'r mut MutVector<T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_unsliced_mut_mul!(&'r mut MutVector<T>);
}

// -----Immutable Multiplication-----
impl<T> Mul<MutVector<T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_unsliced_immut_mul!(MutVector<T>);
}

impl<T> Mul<MutVector<T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_unsliced_immut_mul!(MutVector<T>);
}

impl<T> Mul<&MutVector<T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_unsliced_immut_mul!(&MutVector<T>);
}

impl<T> Mul<&MutVector<T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_unsliced_immut_mul!(&MutVector<T>);
}