use std::ops::{Mul, Add};

use crate::{vectors::{MutVector, MutVectorSlice}, macros::mul::dot_product_left_sliced};

impl<'l, T> Mul<MutVector<T>> for MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(MutVector<T>);
}

impl<'l, T> Mul<MutVector<T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(MutVector<T>);
}

impl<'l, T> Mul<&MutVector<T>> for MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(&MutVector<T>);
}

impl<'l, T> Mul<&MutVector<T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(&MutVector<T>);
}

impl<'l, 'r, T> Mul<&'r mut MutVector<T>> for MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(&'r mut MutVector<T>);
}

impl<'l, 'r, T> Mul<&'r mut MutVector<T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(&'r mut MutVector<T>);
}