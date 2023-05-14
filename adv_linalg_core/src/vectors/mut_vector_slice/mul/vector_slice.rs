use std::ops::{Mul, Add};
use crate::vectors::{VectorSlice, MutVectorSlice};
use crate::macros::mul::dot_product_both_sliced;

impl<'l, 'r, T> Mul<VectorSlice<'r, T>> for MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_both_sliced!(VectorSlice<'r, T>);
}

impl<'l, 'r, T> Mul<VectorSlice<'r, T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_both_sliced!(VectorSlice<'r, T>);
}

impl<'l, 'r, T> Mul<&VectorSlice<'r, T>> for MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_both_sliced!(&VectorSlice<'r, T>);
}

impl<'l, 'r, T> Mul<&VectorSlice<'r, T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_both_sliced!(&VectorSlice<'r, T>);
}