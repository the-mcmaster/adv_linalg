use std::ops::Add;
use crate::Vector;
use crate::vectors::MutVector;
use crate::macros::add::vector_unsliced_immut_add;

impl<T> Add<MutVector<T>> for Vector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(MutVector<T>);
}

impl<T> Add<MutVector<T>> for &Vector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(MutVector<T>);
}

impl<T> Add<&MutVector<T>> for Vector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(&MutVector<T>);
}

impl<T> Add<&MutVector<T>> for &Vector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(&MutVector<T>);
}

impl<'r, T> Add<&'r mut MutVector<T>> for Vector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(&'r mut MutVector<T>);
}

impl<'r, T> Add<&'r mut MutVector<T>> for &Vector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(&'r mut MutVector<T>);
}