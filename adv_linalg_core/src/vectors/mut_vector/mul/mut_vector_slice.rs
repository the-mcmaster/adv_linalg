use std::ops::{Mul, Add};
use crate::vectors::{MutVectorSlice, MutVector};
use crate::macros::mul::dot_product_right_sliced;

impl<'r, T> Mul<MutVectorSlice<'r, T>> for MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(MutVectorSlice<'r, T>);
}

impl<'r, T> Mul<MutVectorSlice<'r, T>> for &MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(MutVectorSlice<'r, T>);
}

impl<'r, T> Mul<MutVectorSlice<'r, T>> for &mut MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(MutVectorSlice<'r, T>);
}

impl<'r, T> Mul<&MutVectorSlice<'r, T>> for MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(&MutVectorSlice<'r, T>);
}

impl<'r, T> Mul<&MutVectorSlice<'r, T>> for &MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(&MutVectorSlice<'r, T>);
}

impl<'r, T> Mul<&MutVectorSlice<'r, T>> for &mut MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(&MutVectorSlice<'r, T>);
}