use std::ops::Sub;
use crate::Vector;
use crate::vectors::{MutVectorSlice, VectorSlice};
use crate::macros::sub::vector_both_sliced_immut_sub;

impl<'l, 'r, T> Sub<MutVectorSlice<'r, T>> for VectorSlice<'l, T>
where
    T: Clone + Sub<Output = T>
{
    vector_both_sliced_immut_sub!(MutVectorSlice<'r, T>);
}

impl<'l, 'r, T> Sub<MutVectorSlice<'r, T>> for &VectorSlice<'l, T>
where
    T: Clone + Sub<Output = T>
{
    vector_both_sliced_immut_sub!(MutVectorSlice<'r, T>);
}

impl<'l, 'r, T> Sub<&MutVectorSlice<'r, T>> for VectorSlice<'l, T>
where
    T: Clone + Sub<Output = T>
{
    vector_both_sliced_immut_sub!(&MutVectorSlice<'r, T>);
}

impl<'l, 'r, T> Sub<&MutVectorSlice<'r, T>> for &VectorSlice<'l, T>
where
    T: Clone + Sub<Output = T>
{
    vector_both_sliced_immut_sub!(&MutVectorSlice<'r, T>);
}