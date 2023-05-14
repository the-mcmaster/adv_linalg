use std::ops::Add;
use crate::Vector;
use crate::vectors::MutVector;
use crate::macros::add::{vector_unsliced_immut_add, vector_unsliced_mut_add};

// -----Mut Adds-----
impl<'l, T> Add<MutVector<T>> for &'l mut MutVector<T>
where
    T: Clone +  Add<Output = T>
{
    vector_unsliced_mut_add!(MutVector<T>);
}

impl<'l, T> Add<&MutVector<T>> for &'l mut MutVector<T>
where
    T: Clone +  Add<Output = T>
{
    vector_unsliced_mut_add!(&MutVector<T>);
}

impl<'l, 'r, T> Add<&'r mut MutVector<T>> for &'l mut MutVector<T>
where
    T: Clone +  Add<Output = T>
{
    vector_unsliced_mut_add!(&'r mut MutVector<T>);
}

// -----Immut Adds-----
impl<T> Add<MutVector<T>> for MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(MutVector<T>);
}

impl<T> Add<MutVector<T>> for &MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(MutVector<T>);
}

impl<T> Add<&MutVector<T>> for MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(&MutVector<T>);
}

impl<T> Add<&MutVector<T>> for &MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(&MutVector<T>);
}

impl<'r, T> Add<&'r mut MutVector<T>> for MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(&'r mut MutVector<T>);
}

impl<'r, T> Add<&'r mut MutVector<T>> for &MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(&'r mut MutVector<T>);
}