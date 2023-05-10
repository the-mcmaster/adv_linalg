use std::ops::Mul;
use crate::Vector;
use crate::macros::mul::vector_unsliced_immut_scaled;

impl<T> Mul<T> for Vector<T>
where
    T: Clone + Mul<Output = T>
{
    vector_unsliced_immut_scaled!(T);
}

impl<T> Mul<T> for &Vector<T>
where
    T: Clone + Mul<Output = T>
{
    vector_unsliced_immut_scaled!(T);
}

impl<T> Mul<&T> for Vector<T>
where
    T: Clone + Mul<Output = T>
{
    vector_unsliced_immut_scaled!(&T);
}

impl<T> Mul<&T> for &Vector<T>
where
    T: Clone + Mul<Output = T>
{
    vector_unsliced_immut_scaled!(&T);
}