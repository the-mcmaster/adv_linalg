use std::ops::{Mul, Add};
use crate::Vector;
use crate::macros::mul::dot_product_unsliced;

impl<T> Mul<Vector<T>> for Vector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(Vector<T>);
}

impl<T> Mul<Vector<T>> for &Vector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(Vector<T>);
}

impl<T> Mul<&Vector<T>> for Vector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(&Vector<T>);
}

impl<T> Mul<&Vector<T>> for &Vector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    dot_product_unsliced!(&Vector<T>);
}