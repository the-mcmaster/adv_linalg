use std::ops::Sub;
use crate::Vector;
use crate::macros::sub::vector_unsliced_immut_sub;

impl<T> Sub<Vector<T>> for Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(Vector<T>);
}

impl<T> Sub<Vector<T>> for &Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(Vector<T>);
}

impl<T> Sub<&Vector<T>> for Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(&Vector<T>);
}

impl<T> Sub<&Vector<T>> for &Vector<T>
where
    T: Clone + Sub<Output = T>
{
    vector_unsliced_immut_sub!(&Vector<T>);
}