//! Module hosting all available traits, especially
//! [Lambda][crate::traits::Lambda] and [Map][crate::traits::Map].
//! 
//! [Lambda][crate::traits::Lambda] and [Map][crate::traits::Map]
//! provide [second-order functionality] between types.
//! 
//! [Lambda][crate::traits::Lambda] provides methods to a vector or matrix
//! to produce another vector or matrix by passing each element together through
//! a user-provided function.
//! 
//! [Map][crate::traits::Map] provides methods to transform two vectors or
//! matricies into one vector or matrix by mapping each corresponding elements 
//! together through a user-provided function.
//! 
//! [second-order functionality]: <https://en.wikipedia.org/wiki/Higher-order_function>

/// Second-order functionality for transforming an immutable type element-wise.
pub trait Lambda {
    type Output;
    type Inner;
    type Index;

    fn lambda<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner) -> Self::Inner;

    fn lambda_index<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index) -> Self::Inner;

    fn lambda_enumerate<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner) -> Self::Inner;
}

/// Second-order functionality for transforming an mutable type element-wise.
pub trait LambdaMut<'x> {
    type Output;
    type Inner;
    type Index;

    fn lambda_mut<F>(&'x mut self, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner) -> Self::Inner;

    fn lambda_index_mut<F>(&'x mut self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index) -> Self::Inner;

    fn lambda_enumerate_mut<F>(&'x mut self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner) -> Self::Inner;
}

/// Second-order functionality for combining two immutable types element-wise.
pub trait Map<Rhs> {
    type Output;
    type Inner;
    type Index;

    fn map<F>(&self, other: &Rhs, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner;

    fn map_enumerate<F>(&self, other: &Rhs, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner;
}

/// Second-order functionality for combining two types element-wise.
/// 
/// The left hand side element is mutable.
pub trait MapMut<'x, Rhs> {
    type Output;
    type Inner;
    type Index;

    fn map_mut<F>(&'x mut self, other: &Rhs, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner;

    fn map_enumerate_mut<F>(&'x mut self, other: &Rhs, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner;
}