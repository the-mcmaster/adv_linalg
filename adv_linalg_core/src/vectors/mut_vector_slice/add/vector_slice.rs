use std::ops::Add;
use crate::Vector;
use crate::vectors::{VectorSlice, MutVectorSlice};
use crate::macros::add::vector_both_sliced_immut_add;

impl<'l, 'r, T> Add<VectorSlice<'r, T>> for MutVectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_both_sliced_immut_add!(VectorSlice<'r, T>);
}

impl<'l, 'r, T> Add<VectorSlice<'r, T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_both_sliced_immut_add!(VectorSlice<'r, T>);
}

impl<'l, 'r, T> Add<&VectorSlice<'r, T>> for MutVectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_both_sliced_immut_add!(&VectorSlice<'r, T>);
}

impl<'l, 'r, T> Add<&VectorSlice<'r, T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_both_sliced_immut_add!(&VectorSlice<'r, T>);
}