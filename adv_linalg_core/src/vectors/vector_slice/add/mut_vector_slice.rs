use std::ops::Add;
use crate::Vector;
use crate::vectors::{MutVectorSlice, VectorSlice};
use crate::macros::add::vector_both_sliced_immut_add;

impl<'l, 'r, T> Add<MutVectorSlice<'r, T>> for VectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_both_sliced_immut_add!(MutVectorSlice<'r, T>);
}

impl<'l, 'r, T> Add<MutVectorSlice<'r, T>> for &VectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_both_sliced_immut_add!(MutVectorSlice<'r, T>);
}

impl<'l, 'r, T> Add<&MutVectorSlice<'r, T>> for VectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_both_sliced_immut_add!(&MutVectorSlice<'r, T>);
}

impl<'l, 'r, T> Add<&MutVectorSlice<'r, T>> for &VectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_both_sliced_immut_add!(&MutVectorSlice<'r, T>);
}