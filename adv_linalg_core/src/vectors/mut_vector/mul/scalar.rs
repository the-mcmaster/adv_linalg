use std::ops::{Mul, Add};
use crate::Vector;
use crate::vectors::MutVector;
use crate::macros::mul::{vector_mut_scaled, vector_unsliced_immut_scaled};

// -----Mut Multiplications-----
impl<'r, T> Mul<T> for &'r mut MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    vector_mut_scaled!(T);
}

impl<'r, T> Mul<&T> for &'r mut MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    vector_mut_scaled!(&T);
}

// -----Immut Multiplications-----
impl<T> Mul<T> for MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    vector_unsliced_immut_scaled!(T);
}

impl<T> Mul<T> for &MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    vector_unsliced_immut_scaled!(T);
}

impl<T> Mul<&T> for MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    vector_unsliced_immut_scaled!(&T);
}

impl<T> Mul<&T> for &MutVector<T>
where
    T: Clone + Mul<Output = T> + Add<Output = T> + Default
{
    vector_unsliced_immut_scaled!(&T);
}