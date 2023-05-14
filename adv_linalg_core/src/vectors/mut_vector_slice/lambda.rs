use crate::Vector;
use crate::vectors::MutVectorSlice;
use crate::traits::Lambda;

impl<'a, T> Lambda for MutVectorSlice<'a, T> {
    type Output = Vector<T>;

    type Inner = T;

    type Index = usize;

    fn lambda<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner) -> Self::Inner
    {
        let mut params = Vec::with_capacity(self.len());
        
        for idx in self.start()..self.end() {
            params.push(funct(&self.vector.list[idx]))
        }

        Vector::from(params)
    }

    fn lambda_index<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index) -> Self::Inner
    {
        let mut params = Vec::with_capacity(self.len());

        for idx in self.start()..self.end() {
            params.push(funct(idx))
        }

        Vector::from(params)
    }

    fn lambda_enumerate<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner) -> Self::Inner
    {
        let mut params = Vec::with_capacity(self.len());

        for idx in self.start()..self.end() {
            params.push(funct(idx, &self.vector.list[idx]))
        }

        Vector::from(params)
    }
}