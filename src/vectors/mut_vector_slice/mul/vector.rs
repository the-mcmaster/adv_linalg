use std::ops::{Mul, Add};

use crate::{vectors::MutVectorSlice, macros::mul::dot_product_left_sliced, Vector};

impl<'l, T> Mul<Vector<T>> for MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(Vector<T>);
}

impl<'l, T> Mul<Vector<T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(Vector<T>);
}

impl<'l, T> Mul<&Vector<T>> for MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(&Vector<T>);
}

impl<'l, T> Mul<&Vector<T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(&Vector<T>);
}