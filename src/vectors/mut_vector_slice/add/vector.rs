use std::ops::Add;
use crate::Vector;
use crate::vectors::MutVectorSlice;
use crate::macros::vector_left_sliced_immut_add;

impl<'l, T> Add<Vector<T>> for MutVectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_left_sliced_immut_add!(Vector<T>);
}

impl<'l, T> Add<Vector<T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_left_sliced_immut_add!(Vector<T>);
}

impl<'l, T> Add<&Vector<T>> for MutVectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_left_sliced_immut_add!(&Vector<T>);
}

impl<'l, T> Add<&Vector<T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Add<Output = T>
{
    vector_left_sliced_immut_add!(&Vector<T>);
}