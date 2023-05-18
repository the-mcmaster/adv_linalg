use std::ops::{Mul, Add};
use crate::{Vector, Matrix};
use crate::macros::mul::matrix_vector_unsliced_immut_mul;

impl<T> Mul<Vector<T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_unsliced_immut_mul!(Vector<T>);
}

impl<T> Mul<Vector<T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_unsliced_immut_mul!(Vector<T>);
}

impl<T> Mul<&Vector<T>> for Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_unsliced_immut_mul!(&Vector<T>);
}

impl<T> Mul<&Vector<T>> for &Matrix<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    matrix_vector_unsliced_immut_mul!(&Vector<T>);
}