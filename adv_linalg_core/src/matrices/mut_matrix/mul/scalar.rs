use std::ops::Mul;
use crate::Matrix;
use crate::matrices::MutMatrix;
use crate::macros::mul::{matrix_mut_scaled, matrix_unsliced_immut_scaled};

// -----Mutable Scalar-----
impl<'l, T> Mul<T> for &'l mut MutMatrix<T>
where
    T: Clone + Mul<Output = T>
{
    matrix_mut_scaled!(T);
}

impl<'l, T> Mul<&T> for &'l mut MutMatrix<T>
where
    T: Clone + Mul<Output = T>
{
    matrix_mut_scaled!(&T);
}

// -----Immutable Scalar-----
impl<T> Mul<T> for MutMatrix<T>
where
    T: Clone + Mul<Output = T>
{
    matrix_unsliced_immut_scaled!(T);
}

impl<T> Mul<T> for &MutMatrix<T>
where
    T: Clone + Mul<Output = T>
{
    matrix_unsliced_immut_scaled!(T);
}

impl<T> Mul<&T> for MutMatrix<T>
where
    T: Clone + Mul<Output = T>
{
    matrix_unsliced_immut_scaled!(&T);
}

impl<T> Mul<&T> for &MutMatrix<T>
where
    T: Clone + Mul<Output = T>
{
    matrix_unsliced_immut_scaled!(&T);
}