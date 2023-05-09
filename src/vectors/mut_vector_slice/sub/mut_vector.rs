use std::ops::Sub;
use crate::Vector;
use crate::vectors::{MutVector, MutVectorSlice};
use crate::macros::sub::vector_left_sliced_immut_sub;

impl<'l, T> Sub<MutVector<T>> for MutVectorSlice<'l, T>
where
    T: Clone + Sub<Output = T>
{
    vector_left_sliced_immut_sub!(MutVector<T>);
}

impl<'l, T> Sub<MutVector<T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Sub<Output = T>
{
    vector_left_sliced_immut_sub!(MutVector<T>);
}

impl<'l, T> Sub<&MutVector<T>> for MutVectorSlice<'l, T>
where
    T: Clone + Sub<Output = T>
{
    vector_left_sliced_immut_sub!(&MutVector<T>);
}

impl<'l, T> Sub<&MutVector<T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Sub<Output = T>
{
    vector_left_sliced_immut_sub!(&MutVector<T>);
}

impl<'l, 'r, T> Sub<&'r mut MutVector<T>> for MutVectorSlice<'l, T>
where
    T: Clone + Sub<Output = T>
{
    vector_left_sliced_immut_sub!(&'r mut MutVector<T>);
}

impl<'l, 'r, T> Sub<&'r mut MutVector<T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Sub<Output = T>
{
    vector_left_sliced_immut_sub!(&'r mut MutVector<T>);
}