use super::MutVector;

mod add;
mod mul;
mod sub;
mod lambda;
mod map;

impl<T> MutVector<T> {
    pub fn len(&self) -> usize {
        self.list.len()
    }
}