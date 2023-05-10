use std::ops::{Mul, Add};

use crate::{vectors::MutVectorSlice, macros::mul::dot_product_both_sliced};

impl<'l, 'r, T> Mul<MutVectorSlice<'r, T>> for MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_both_sliced!(MutVectorSlice<'r, T>);
}

impl<'l, 'r, T> Mul<MutVectorSlice<'r, T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_both_sliced!(MutVectorSlice<'r, T>);
}

impl<'l, 'r, T> Mul<&MutVectorSlice<'r, T>> for MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_both_sliced!(&MutVectorSlice<'r, T>);
}

impl<'l, 'r, T> Mul<&MutVectorSlice<'r, T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_both_sliced!(&MutVectorSlice<'r, T>);
}