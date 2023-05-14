use std::ops::Sub;
use crate::Vector;
use crate::vectors::MutVectorSlice;
use crate::macros::sub::vector_left_sliced_immut_sub;

impl<'l, T> Sub<Vector<T>> for MutVectorSlice<'l, T>
where
    T: Clone + Sub<Output = T>
{
    vector_left_sliced_immut_sub!(Vector<T>);
}

impl<'l, T> Sub<Vector<T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Sub<Output = T>
{
    vector_left_sliced_immut_sub!(Vector<T>);
}

impl<'l, T> Sub<&Vector<T>> for MutVectorSlice<'l, T>
where
    T: Clone + Sub<Output = T>
{
    vector_left_sliced_immut_sub!(&Vector<T>);
}

impl<'l, T> Sub<&Vector<T>> for &MutVectorSlice<'l, T>
where
    T: Clone + Sub<Output = T>
{
    vector_left_sliced_immut_sub!(&Vector<T>);
}