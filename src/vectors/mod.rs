mod vector;
mod mut_vector;
mod vector_slice;
mod mut_vector_slice;

use std::ops::Range;

pub struct Vector<T> {
    pub(crate) list : Vec<T>
}

pub struct VectorSlice<'a, T> {
    pub(crate) vector : &'a Vector<T>,
    slice_range : Range<usize>
}

pub struct MutVector<T> {
    pub(crate) list : Vec<T>
}

pub struct MutVectorSlice<'a, T> {
    pub(crate) vector : &'a MutVector<T>,
    slice_range : Range<usize>
}