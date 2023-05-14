use std::ops::{Mul, Add};
use crate::Vector;
use crate::vectors::MutVector;
use crate::macros::mul::dot_product_unsliced;

impl<T> Mul<Vector<T>> for MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(Vector<T>);
}

impl<T> Mul<Vector<T>> for &MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(Vector<T>);
}

impl<T> Mul<&Vector<T>> for MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(&Vector<T>);
}

impl<T> Mul<&Vector<T>> for &MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(&Vector<T>);
}

impl<T> Mul<Vector<T>> for &mut MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(Vector<T>);
}

impl<T> Mul<&Vector<T>> for &mut MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(&Vector<T>);
}