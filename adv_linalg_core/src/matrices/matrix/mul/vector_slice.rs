use std::ops::{Mul, Add};
use crate::{Vector, Matrix};
use crate::vectors::VectorSlice;
use crate::macros::mul::matrix_vector_right_sliced_immut_mul;

impl<'r, T> Mul<VectorSlice<'r, T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_right_sliced_immut_mul!(VectorSlice<'r, T>);
}

impl<'r, T> Mul<VectorSlice<'r, T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_right_sliced_immut_mul!(VectorSlice<'r, T>);
}

impl<'r, T> Mul<&VectorSlice<'r, T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_right_sliced_immut_mul!(&VectorSlice<'r, T>);
}

impl<'r, T> Mul<&VectorSlice<'r, T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_right_sliced_immut_mul!(&VectorSlice<'r, T>);
}