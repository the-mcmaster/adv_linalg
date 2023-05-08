use std::ops::Add;
use crate::Vector;
use crate::vectors::MutVectorSlice;
use crate::macros::vector_right_sliced_immut_add;

impl<'r, T> Add<MutVectorSlice<'r, T>> for Vector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(MutVectorSlice<'r, T>);
}

impl<'r, T> Add<MutVectorSlice<'r, T>> for &Vector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(MutVectorSlice<'r, T>);
}

impl<'r, T> Add<&MutVectorSlice<'r, T>> for Vector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(&MutVectorSlice<'r, T>);
}

impl<'r, T> Add<&MutVectorSlice<'r, T>> for &Vector<T>
where
    T: Clone + Add<Output = T>
{
    vector_right_sliced_immut_add!(&MutVectorSlice<'r, T>);
}