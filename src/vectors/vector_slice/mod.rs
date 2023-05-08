use crate::Vector;

use crate::vectors::VectorSlice;

mod add;
mod mul;
mod sub;
mod lambda;
mod map;

impl<'a, T> VectorSlice<'a, T> {
    pub fn len(&self) -> usize {
        self.slice_range.end - self.slice_range.start
    }

    pub fn start(&self) -> usize {
        self.slice_range.start
    }

    pub fn end(&self) -> usize {
        self.slice_range.end
    }
}

impl<'a, T> From<&'a Vector<T>> for VectorSlice<'a, T> {
    fn from(vector: &'a Vector<T>) -> Self {
        VectorSlice {
            vector : &vector, 
            slice_range: 0..vector.len()
        }
    }
}