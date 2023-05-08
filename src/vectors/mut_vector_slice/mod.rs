use super::MutVectorSlice;

mod add;
mod mul;
mod sub;
mod lambda;
mod map;

impl<'a, T> MutVectorSlice<'a, T> {
    pub fn start(&self) -> usize {
        self.slice_range.start
    }

    pub fn end(&self) -> usize {
        self.slice_range.end
    }
}