use std::ops::Sub;
use crate::Vector;
use crate::vectors::MutVector;
use crate::macros::sub::vector_unsliced_immut_sub;

impl<T> Sub<MutVector<T>> for Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(MutVector<T>);
}

impl<T> Sub<MutVector<T>> for &Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(MutVector<T>);
}

impl<T> Sub<&MutVector<T>> for Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(&MutVector<T>);
}

impl<T> Sub<&MutVector<T>> for &Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(&MutVector<T>);
}

impl<'r, T> Sub<&'r mut MutVector<T>> for Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(&'r mut MutVector<T>);
}

impl<'r, T> Sub<&'r mut MutVector<T>> for &Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(&'r mut MutVector<T>);
}