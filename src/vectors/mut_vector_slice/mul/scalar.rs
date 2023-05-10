use std::ops::Mul;

use crate::{Vector, vectors::MutVectorSlice, macros::mul::vector_sliced_immut_scaled};

impl<'l, T> Mul<T> for MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T>
{
    vector_sliced_immut_scaled!(T);
}

impl<'l, T> Mul<T> for &MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T>
{
    vector_sliced_immut_scaled!(T);
}

impl<'l, T> Mul<&T> for MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T>
{
    vector_sliced_immut_scaled!(&T);
}

impl<'l, T> Mul<&T> for &MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T>
{
    vector_sliced_immut_scaled!(&T);
}