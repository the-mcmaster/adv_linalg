use std::ops::{Mul, Add};

use crate::{vectors::MutVector, macros::mul::dot_product_unsliced, Vector};

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