use std::ops::Add;
use crate::Vector;
use crate::vectors::{MutVector, MutVectorSlice};
use crate::macros::add::vector_left_sliced_immut_add;

impl<'l, T> Add<MutVector<T>> for MutVectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_left_sliced_immut_add!(MutVector<T>);
}

impl<'l, T> Add<MutVector<T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_left_sliced_immut_add!(MutVector<T>);
}

impl<'l, T> Add<&MutVector<T>> for MutVectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_left_sliced_immut_add!(&MutVector<T>);
}

impl<'l, T> Add<&MutVector<T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_left_sliced_immut_add!(&MutVector<T>);
}

impl<'l, 'r, T> Add<&'r mut MutVector<T>> for MutVectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_left_sliced_immut_add!(&'r mut MutVector<T>);
}

impl<'l, 'r, T> Add<&'r mut MutVector<T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_left_sliced_immut_add!(&'r mut MutVector<T>);
}