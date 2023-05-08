use std::ops::Add;
use crate::Vector;
use crate::vectors::MutVector;
use crate::macros::{vector_unsliced_immut_add, vector_unsliced_mut_add};

// -----Mut Adds-----
impl<'r, T> Add<Vector<T>> for &'r mut MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_mut_add!(Vector<T>);
}

impl<'r, T> Add<&Vector<T>> for &'r mut MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_mut_add!(&Vector<T>);
}

// -----Immut Adds-----
impl<T> Add<Vector<T>> for MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(Vector<T>);
}

impl<T> Add<Vector<T>> for &MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(Vector<T>);
}

impl<T> Add<&Vector<T>> for MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(&Vector<T>);
}

impl<T> Add<&Vector<T>> for &MutVector<T>
where
    T: Clone + Add<Output = T>
{
    vector_unsliced_immut_add!(&Vector<T>);
}