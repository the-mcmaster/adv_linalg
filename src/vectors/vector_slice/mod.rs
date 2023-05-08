use super::VectorSlice;

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