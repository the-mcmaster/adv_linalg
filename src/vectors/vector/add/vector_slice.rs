use std::ops::Add;
use crate::Vector;
use crate::vectors::VectorSlice;
use crate::macros::add::vector_right_sliced_immut_add;

impl<'r, T> Add<VectorSlice<'r, T>> for Vector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(VectorSlice<'r, T>);
}

impl<'r, T> Add<VectorSlice<'r, T>> for &Vector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(VectorSlice<'r, T>);
}

impl<'r, T> Add<&VectorSlice<'r, T>> for Vector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(&VectorSlice<'r, T>);
}

impl<'r, T> Add<&VectorSlice<'r, T>> for &Vector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(&VectorSlice<'r, T>);
}