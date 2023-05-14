use std::ops::Sub;
use crate::Vector;
use crate::vectors::{MutVectorSlice, MutVector};
use crate::macros::sub::{vector_right_sliced_immut_sub, vector_sliced_mut_sub};

// -----Mut Subtractions-----
impl<'l, 'r, T> Sub<MutVectorSlice<'r, T>> for &'l mut MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_sliced_mut_sub!(MutVectorSlice<'r, T>);
}

impl<'l, 'r, T> Sub<&MutVectorSlice<'r, T>> for &'l mut MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_sliced_mut_sub!(&MutVectorSlice<'r, T>);
}

// -----Immut Subtractions-----
impl<'r, T> Sub<MutVectorSlice<'r, T>> for MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(MutVectorSlice<'r, T>);
}

impl<'r, T> Sub<MutVectorSlice<'r, T>> for &MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(MutVectorSlice<'r, T>);
}

impl<'r, T> Sub<&MutVectorSlice<'r, T>> for MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(&MutVectorSlice<'r, T>);
}

impl<'r, T> Sub<&MutVectorSlice<'r, T>> for &MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(&MutVectorSlice<'r, T>);
}