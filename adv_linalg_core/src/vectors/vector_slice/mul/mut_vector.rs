use std::ops::{Mul, Add};
use crate::vectors::{MutVector, VectorSlice};
use crate::macros::mul::dot_product_left_sliced;

impl<'l, T> Mul<MutVector<T>> for VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(MutVector<T>);
}

impl<'l, T> Mul<MutVector<T>> for &VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(MutVector<T>);
}

impl<'l, T> Mul<&MutVector<T>> for VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(&MutVector<T>);
}

impl<'l, T> Mul<&MutVector<T>> for &VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(&MutVector<T>);
}

impl<'l, T> Mul<&mut MutVector<T>> for VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(&mut MutVector<T>);
}

impl<'l, T> Mul<&mut MutVector<T>> for &VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(&mut MutVector<T>);
}