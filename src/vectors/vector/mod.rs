use crate::Vector;

mod add;
mod mul;
mod sub;
mod lambda;
mod map;

impl<T> Vector<T> {
    pub fn len(&self) -> usize {
        self.list.len()
    }
}