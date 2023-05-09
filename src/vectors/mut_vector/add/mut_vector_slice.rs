use std::ops::Add;
use crate::Vector;
use crate::vectors::{MutVectorSlice, MutVector};
use crate::macros::add::{vector_right_sliced_immut_add, vector_sliced_mut_add};

// -----Mut Adds-----
impl<'l, 'r, T> Add<MutVectorSlice<'r, T>> for &'l mut MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_sliced_mut_add!(MutVectorSlice<'r, T>);
}

impl<'l, 'r, T> Add<&MutVectorSlice<'r, T>> for &'l mut MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_sliced_mut_add!(&MutVectorSlice<'r, T>);
}

// -----Immut Adds-----
impl<'r, T> Add<MutVectorSlice<'r, T>> for MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(MutVectorSlice<'r, T>);
}

impl<'r, T> Add<MutVectorSlice<'r, T>> for &MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(MutVectorSlice<'r, T>);
}

impl<'r, T> Add<&MutVectorSlice<'r, T>> for MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(&MutVectorSlice<'r, T>);
}

impl<'r, T> Add<&MutVectorSlice<'r, T>> for &MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(&MutVectorSlice<'r, T>);
}