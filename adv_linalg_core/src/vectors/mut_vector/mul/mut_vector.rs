use std::ops::{Mul, Add};
use crate::vectors::MutVector;
use crate::macros::mul::dot_product_unsliced;

impl<T> Mul<MutVector<T>> for MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(MutVector<T>);
}

impl<T> Mul<MutVector<T>> for &MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(MutVector<T>);
}

impl<T> Mul<MutVector<T>> for &mut MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(MutVector<T>);
}

impl<T> Mul<&MutVector<T>> for MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(&MutVector<T>);
}

impl<T> Mul<&MutVector<T>> for &MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(&MutVector<T>);
}

impl<T> Mul<&MutVector<T>> for &mut MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(&MutVector<T>);
}

impl<T> Mul<&mut MutVector<T>> for MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(&mut MutVector<T>);
}

impl<T> Mul<&mut MutVector<T>> for &MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(&mut MutVector<T>);
}

impl<T> Mul<&mut MutVector<T>> for &mut MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(&mut MutVector<T>);
}