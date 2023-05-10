use std::ops::{Mul, Add};

use crate::{vectors::MutVectorSlice, Vector, macros::mul::dot_product_right_sliced};

impl<'r, T> Mul<MutVectorSlice<'r, T>> for Vector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(MutVectorSlice<'r, T>);
}

impl<'r, T> Mul<MutVectorSlice<'r, T>> for &Vector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(MutVectorSlice<'r, T>);
}

impl<'r, T> Mul<&MutVectorSlice<'r, T>> for Vector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(&MutVectorSlice<'r, T>);
}

impl<'r, T> Mul<&MutVectorSlice<'r, T>> for &Vector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(&MutVectorSlice<'r, T>);
}