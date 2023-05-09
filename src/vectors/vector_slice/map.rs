use crate::Vector;
use crate::vectors::{VectorSlice, MutVector, MutVectorSlice};
use crate::traits::Map;

impl<T> Map<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    type Inner = T;

    type Index = usize;

    fn map<F>(&self, other: &Vector<T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        let mut params = Vec::with_capacity(self.len());

        for idx in 0..self.len() {
            params.push(funct(&self.list[idx], &other.list[idx]))
        }

        Vector::from(params)
    }

    fn map_enumerate<F>(&self, other: &Vector<T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        let mut params = Vec::with_capacity(self.len());

        for idx in 0..self.len() {
            params.push(funct(idx, &self.list[idx], &other.list[idx]))
        }

        Vector::from(params)
    }
}

impl<'r, T> Map<VectorSlice<'r, T>> for Vector<T> {
    type Output = Vector<T>;

    type Inner = T;

    type Index = usize;

    fn map<F>(&self, other: &VectorSlice<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        let mut params = Vec::with_capacity(self.len());

        for idx in 0..self.len() {
            params.push(funct(&self.list[idx], &other.vector.list[idx]))
        }

        Vector::from(params)
    }

    fn map_enumerate<F>(&self, other: &VectorSlice<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        let mut params = Vec::with_capacity(self.len());

        for idx in 0..self.len() {
            params.push(funct(idx, &self.list[idx], &other.vector.list[idx]))
        }

        Vector::from(params)
    }
}

impl<T> Map<MutVector<T>> for Vector<T> {
    type Output = Vector<T>;

    type Inner = T;

    type Index = usize;

    fn map<F>(&self, other: &MutVector<T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        let mut params = Vec::with_capacity(self.len());

        for idx in 0..self.len() {
            params.push(funct(&self.list[idx], &other.list[idx]))
        }

        Vector::from(params)
    }

    fn map_enumerate<F>(&self, other: &MutVector<T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        let mut params = Vec::with_capacity(self.len());

        for idx in 0..self.len() {
            params.push(funct(idx, &self.list[idx], &other.list[idx]))
        }

        Vector::from(params)
    }
}

impl<'r, T> Map<MutVectorSlice<'r, T>> for Vector<T> {
    type Output = Vector<T>;
    
    type Inner = T;
    
    type Index = usize;
    
    fn map<F>(&self, other: &MutVectorSlice<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        let mut params = Vec::with_capacity(self.len());

        for idx in 0..self.len() {
            params.push(funct(&self.list[idx], &other.vector.list[idx]))
        }

        Vector::from(params)
    }
        
    fn map_enumerate<F>(&self, other: &MutVectorSlice<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        let mut params = Vec::with_capacity(self.len());

        for idx in 0..self.len() {
            params.push(funct(idx, &self.list[idx], &other.vector.list[idx]))
        }

        Vector::from(params)
    }
}