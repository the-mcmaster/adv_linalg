use std::ops::Sub;
use crate::Vector;
use crate::vectors::MutVectorSlice;
use crate::macros::sub::vector_right_sliced_immut_sub;

impl<'r, T> Sub<MutVectorSlice<'r, T>> for Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(MutVectorSlice<'r, T>);
}

impl<'r, T> Sub<MutVectorSlice<'r, T>> for &Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(MutVectorSlice<'r, T>);
}

impl<'r, T> Sub<&MutVectorSlice<'r, T>> for Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(&MutVectorSlice<'r, T>);
}

impl<'r, T> Sub<&MutVectorSlice<'r, T>> for &Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(&MutVectorSlice<'r, T>);
}