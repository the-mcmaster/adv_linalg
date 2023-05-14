use std::ops::Add;
use crate::Matrix;
use crate::matrices::MutMatrix;
use crate::macros::add::{matrix_unsliced_immut_add, matrix_unsliced_mut_add};

// -----Mut Adds-----
impl<'r, T> Add<Matrix<T>> for &'r mut MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_mut_add!(Matrix<T>);
}

impl<'r, T> Add<&Matrix<T>> for &'r mut MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_mut_add!(&Matrix<T>);
}

// -----Immut Adds-----
impl<T> Add<Matrix<T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(Matrix<T>);
}

impl<T> Add<Matrix<T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(Matrix<T>);
}

impl<T> Add<&Matrix<T>> for MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(&Matrix<T>);
}

impl<T> Add<&Matrix<T>> for &MutMatrix<T>
where
    T: Clone + Add<Output = T>
{
    matrix_unsliced_immut_add!(&Matrix<T>);
}