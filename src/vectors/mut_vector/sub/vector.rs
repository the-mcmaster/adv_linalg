use std::ops::Sub;
use crate::Vector;
use crate::vectors::MutVector;
use crate::macros::sub::{vector_unsliced_immut_sub, vector_unsliced_mut_sub};

// -----Mut Subtractions-----
impl<'r, T> Sub<Vector<T>> for &'r mut MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_mut_sub!(Vector<T>);
}

impl<'r, T> Sub<&Vector<T>> for &'r mut MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_mut_sub!(&Vector<T>);
}

// -----Immut Subtractions-----
impl<T> Sub<Vector<T>> for MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(Vector<T>);
}

impl<T> Sub<Vector<T>> for &MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(Vector<T>);
}

impl<T> Sub<&Vector<T>> for MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(&Vector<T>);
}

impl<T> Sub<&Vector<T>> for &MutVector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(&Vector<T>);
}