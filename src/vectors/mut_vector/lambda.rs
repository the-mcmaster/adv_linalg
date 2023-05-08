use crate::Vector;
use crate::vectors::MutVector;
use crate::traits::{Lambda, LambdaMut};

impl<T> Lambda for MutVector<T> {
    type Output = Vector<T>;

    type Inner = T;

    type Index = usize;

    fn lambda<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner) -> Self::Inner
    {
        let mut params = Vec::with_capacity(self.len());
        
        for idx in 0..self.len() {
            params.push(funct(&self.list[idx]))
        }

        Vector::from(params)
    }

    fn lambda_index<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index) -> Self::Inner
    {
        let mut params = Vec::with_capacity(self.len());

        for idx in 0..self.len() {
            params.push(funct(idx))
        }

        Vector::from(params)
    }

    fn lambda_enumerate<F>(&self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner) -> Self::Inner
    {
        let mut params = Vec::with_capacity(self.len());

        for idx in 0..self.len() {
            params.push(funct(idx, &self.list[idx]))
        }

        Vector::from(params)
    }
}

impl<'x, T: 'x> LambdaMut<'x> for MutVector<T> {
    type Output = &'x mut Self;

    type Inner = T;

    type Index = usize;

    fn lambda_mut<F>(&'x mut self, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner) -> Self::Inner
    {
        for idx in 0..self.len() {
            self.list[idx] = funct(&self.list[idx])
        }

        self
    }

    fn lambda_index_mut<F>(&'x mut self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index) -> Self::Inner
    {
        for idx in 0..self.len() {
            self.list[idx] = funct(idx)
        }

        self
    }

    fn lambda_enumerate_mut<F>(&'x mut self, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner) -> Self::Inner
    {
        for idx in 0..self.len() {
            self.list[idx] = funct(idx, &self.list[idx])
        }

        self
    }
}