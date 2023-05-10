use std::ops::{Mul, Add};

use crate::{vectors::VectorSlice, Vector};
use crate::macros::mul::dot_product_left_sliced;

impl<'l, T> Mul<Vector<T>> for VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(Vector<T>);
}

impl<'l, T> Mul<Vector<T>> for &VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(Vector<T>);
}

impl<'l, T> Mul<&Vector<T>> for VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(&Vector<T>);
}

impl<'l, T> Mul<&Vector<T>> for &VectorSlice<'l, T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_left_sliced!(&Vector<T>);
}