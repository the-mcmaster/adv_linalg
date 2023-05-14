use std::ops::Sub;
use crate::Vector;
use crate::vectors::VectorSlice;
use crate::macros::sub::vector_right_sliced_immut_sub;

impl<'r, T> Sub<VectorSlice<'r, T>> for Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(VectorSlice<'r, T>);
}

impl<'r, T> Sub<VectorSlice<'r, T>> for &Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(VectorSlice<'r, T>);
}

impl<'r, T> Sub<&VectorSlice<'r, T>> for Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(&VectorSlice<'r, T>);
}

impl<'r, T> Sub<&VectorSlice<'r, T>> for &Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_right_sliced_immut_sub!(&VectorSlice<'r, T>);
}