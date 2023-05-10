use std::ops::{Mul, Add};

use crate::vectors::VectorSlice;
use crate::macros::mul::dot_product_both_sliced;

impl<'l, 'r, T> Mul<VectorSlice<'r, T>> for VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_both_sliced!(VectorSlice<'r, T>);
}

impl<'l, 'r, T> Mul<VectorSlice<'r, T>> for &VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_both_sliced!(VectorSlice<'r, T>);
}

impl<'l, 'r, T> Mul<&VectorSlice<'r, T>> for VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_both_sliced!(&VectorSlice<'r, T>);
}

impl<'l, 'r, T> Mul<&VectorSlice<'r, T>> for &VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_both_sliced!(&VectorSlice<'r, T>);
}