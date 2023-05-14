use std::ops::Sub;
use crate::Vector;
use crate::vectors::MutVector;
use crate::macros::sub::{vector_unsliced_immut_sub, vector_unsliced_mut_sub};

// -----Mut Subtractions-----
impl<'l, T> Sub<MutVector<T>> for &'l mut MutVector<T>
where
    T: Clone +  Sub<Output = T>
{
    vector_unsliced_mut_sub!(MutVector<T>);
}

impl<'l, T> Sub<&MutVector<T>> for &'l mut MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_mut_sub!(&MutVector<T>);
}

impl<'l, 'r, T> Sub<&'r mut MutVector<T>> for &'l mut MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_mut_sub!(&'r mut MutVector<T>);
}

// -----Immut Subtractions-----
impl<T> Sub<MutVector<T>> for MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(MutVector<T>);
}

impl<T> Sub<MutVector<T>> for &MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(MutVector<T>);
}

impl<T> Sub<&MutVector<T>> for MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(&MutVector<T>);
}

impl<T> Sub<&MutVector<T>> for &MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(&MutVector<T>);
}

impl<'r, T> Sub<&'r mut MutVector<T>> for MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(&'r mut MutVector<T>);
}

impl<'r, T> Sub<&'r mut MutVector<T>> for &MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(&'r mut MutVector<T>);
}