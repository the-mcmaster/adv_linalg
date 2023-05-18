use std::ops::{Mul, Add};
use crate::{Vector, Matrix};
use crate::vectors::MutVectorSlice;
use crate::macros::mul::matrix_vector_right_sliced_immut_mul;

impl<'r, T> Mul<MutVectorSlice<'r, T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_right_sliced_immut_mul!(MutVectorSlice<'r, T>);
}

impl<'r, T> Mul<MutVectorSlice<'r, T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_right_sliced_immut_mul!(MutVectorSlice<'r, T>);
}

impl<'r, T> Mul<&MutVectorSlice<'r, T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_right_sliced_immut_mul!(&MutVectorSlice<'r, T>);
}

impl<'r, T> Mul<&MutVectorSlice<'r, T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_right_sliced_immut_mul!(&MutVectorSlice<'r, T>);
}