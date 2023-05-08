use std::ops::Add;
use crate::Vector;
use crate::vectors::VectorSlice;
use crate::macros::vector_both_sliced_immut_add;

impl<'l, 'r, T> Add<VectorSlice<'r, T>> for VectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_both_sliced_immut_add!(VectorSlice<'r, T>);
}

impl<'l, 'r, T> Add<VectorSlice<'r, T>> for &VectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_both_sliced_immut_add!(VectorSlice<'r, T>);
}

impl<'l, 'r, T> Add<&VectorSlice<'r, T>> for VectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_both_sliced_immut_add!(&VectorSlice<'r, T>);
}

impl<'l, 'r, T> Add<&VectorSlice<'r, T>> for &VectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_both_sliced_immut_add!(&VectorSlice<'r, T>);
}