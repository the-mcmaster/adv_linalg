use std::ops::{Mul, Add};

use crate::{vectors::{VectorSlice, MutVector}, macros::mul::dot_product_right_sliced};

impl<'r, T> Mul<VectorSlice<'r, T>> for MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(VectorSlice<'r, T>);
}

impl<'r, T> Mul<VectorSlice<'r, T>> for &MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(VectorSlice<'r, T>);
}

impl<'r, T> Mul<VectorSlice<'r, T>> for &mut MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(VectorSlice<'r, T>);
}

impl<'r, T> Mul<&VectorSlice<'r, T>> for MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(&VectorSlice<'r, T>);
}

impl<'r, T> Mul<&VectorSlice<'r, T>> for &MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(&VectorSlice<'r, T>);
}

impl<'r, T> Mul<&VectorSlice<'r, T>> for &mut MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(&VectorSlice<'r, T>);
}