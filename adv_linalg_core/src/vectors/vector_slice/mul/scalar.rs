use std::ops::Mul;
use crate::Vector;
use crate::vectors::VectorSlice;
use crate::macros::mul::vector_sliced_immut_scaled;

impl<'l, T> Mul<T> for VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T>
{
    vector_sliced_immut_scaled!(T);
}

impl<'l, T> Mul<T> for &VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T>
{
    vector_sliced_immut_scaled!(T);
}

impl<'l, T> Mul<&T> for VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T>
{
    vector_sliced_immut_scaled!(&T);
}

impl<'l, T> Mul<&T> for &VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T>
{
    vector_sliced_immut_scaled!(&T);
}