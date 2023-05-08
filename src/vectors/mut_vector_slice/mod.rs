use super::{MutVectorSlice, MutVector};

mod add;
mod mul;
mod sub;
mod lambda;
mod map;

impl<'a, T> MutVectorSlice<'a, T> {
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

impl<'a, T> From<&'a MutVector<T>> for MutVectorSlice<'a, T> {
    fn from(mut_vector: &'a MutVector<T>) -> Self {
        MutVectorSlice {
            vector : mut_vector,
            slice_range: 0..mut_vector.len()
        }
    }
}