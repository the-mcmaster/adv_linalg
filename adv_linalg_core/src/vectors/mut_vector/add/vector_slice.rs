use std::ops::Add;
use crate::Vector;
use crate::vectors::{VectorSlice, MutVector};
use crate::macros::add::{vector_right_sliced_immut_add, vector_sliced_mut_add};

// -----Mut Adds-----
impl<'r, 'l, T> Add<VectorSlice<'r, T>> for &'l mut MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_sliced_mut_add!(VectorSlice<'r, T>);
}

impl<'r, 'l, T> Add<&VectorSlice<'r, T>> for &'l mut MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_sliced_mut_add!(&VectorSlice<'r, T>);
}

// -----Immut Adds-----
impl<'r, T> Add<VectorSlice<'r, T>> for MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(VectorSlice<'r, T>);
}

impl<'r, T> Add<VectorSlice<'r, T>> for &MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(VectorSlice<'r, T>);
}

impl<'r, T> Add<&VectorSlice<'r, T>> for MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(&VectorSlice<'r, T>);
}

impl<'r, T> Add<&VectorSlice<'r, T>> for &MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(&VectorSlice<'r, T>);
}