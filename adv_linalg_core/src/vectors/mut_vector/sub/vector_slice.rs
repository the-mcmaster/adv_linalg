use std::ops::Sub;
use crate::Vector;
use crate::vectors::{VectorSlice, MutVector};
use crate::macros::sub::{vector_right_sliced_immut_sub, vector_sliced_mut_sub};

// -----Mut Subtractions-----
impl<'r, 'l, T> Sub<VectorSlice<'r, T>> for &'l mut MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_sliced_mut_sub!(VectorSlice<'r, T>);
}

impl<'r, 'l, T> Sub<&VectorSlice<'r, T>> for &'l mut MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_sliced_mut_sub!(&VectorSlice<'r, T>);
}

// -----Immut Subtractions-----
impl<'r, T> Sub<VectorSlice<'r, T>> for MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(VectorSlice<'r, T>);
}

impl<'r, T> Sub<VectorSlice<'r, T>> for &MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(VectorSlice<'r, T>);
}

impl<'r, T> Sub<&VectorSlice<'r, T>> for MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(&VectorSlice<'r, T>);
}

impl<'r, T> Sub<&VectorSlice<'r, T>> for &MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(&VectorSlice<'r, T>);
}