use std::ops::Add;
use crate::Vector;
use crate::vectors::VectorSlice;
use crate::macros::vector_left_sliced_immut_add;

impl<'a, T> Add<Vector<T>> for VectorSlice<'a, T>
where
    T: Clone + Add<Output = T>
{
    vector_left_sliced_immut_add!(Vector<T>);
}

impl<'a, T> Add<Vector<T>> for &VectorSlice<'a, T>
where
    T: Clone + Add<Output = T>
{
    vector_left_sliced_immut_add!(Vector<T>);
}

impl<'a, T> Add<&Vector<T>> for VectorSlice<'a, T>
where
    T: Clone + Add<Output = T>
{
    vector_left_sliced_immut_add!(&Vector<T>);
}

impl<'a, T> Add<&Vector<T>> for &VectorSlice<'a, T>
where
    T: Clone + Add<Output = T>
{
    vector_left_sliced_immut_add!(&Vector<T>);
}