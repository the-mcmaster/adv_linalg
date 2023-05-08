use std::ops::Add;
use crate::Vector;
use crate::vectors::MutVector;
use crate::macros::vector_unsliced_immut_add;

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