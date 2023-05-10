use std::ops::{Mul, Add};
use crate::Vector;
use crate::vectors::VectorSlice;
use crate::macros::mul::dot_product_right_sliced;

impl<'r, T> Mul<VectorSlice<'r, T>> for Vector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(VectorSlice<'r, T>);
}

impl<'r, T> Mul<VectorSlice<'r, T>> for &Vector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(VectorSlice<'r, T>);
}

impl<'r, T> Mul<&VectorSlice<'r, T>> for Vector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(&VectorSlice<'r, T>);
}

impl<'r, T> Mul<&VectorSlice<'r, T>> for &Vector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_right_sliced!(&VectorSlice<'r, T>);
}