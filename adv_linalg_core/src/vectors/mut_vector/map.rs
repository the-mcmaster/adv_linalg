use crate::Vector;
use crate::vectors::{VectorSlice, MutVector, MutVectorSlice};
use crate::traits::{Map, MapMut};

impl<T> Map<MutVector<T>> for MutVector<T> {
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

impl<'r, T> Map<MutVectorSlice<'r, T>> for MutVector<T> {
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

impl<T> Map<Vector<T>> for MutVector<T> {
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

impl<'r, T> Map<VectorSlice<'r, T>> for MutVector<T> {
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

impl<'l, T: 'l> MapMut<'l, Vector<T>> for MutVector<T> {
    type Output = &'l mut Self;

    type Inner = T;

    type Index = usize;

    fn map_mut<F>(&'l mut self, other: &Vector<T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        for idx in 0..self.len() {
            self.list[idx] = funct(&self.list[idx], &other.list[idx])
        }

        self
    }

    fn map_enumerate_mut<F>(&'l mut self, other: &Vector<T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        for idx in 0..self.len() {
            self.list[idx] = funct(idx, &self.list[idx], &other.list[idx])
        }

        self
    }
}

impl<'l, T: 'l> MapMut<'l, MutVector<T>> for MutVector<T> {
    type Output = &'l mut Self;

    type Inner = T;

    type Index = usize;

    fn map_mut<F>(&'l mut self, other: &MutVector<T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        for idx in 0..self.len() {
            self.list[idx] = funct(&self.list[idx], &other.list[idx])
        }

        self
    }

    fn map_enumerate_mut<F>(&'l mut self, other: &MutVector<T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        for idx in 0..self.len() {
            self.list[idx] = funct(idx, &self.list[idx], &other.list[idx])
        }

        self
    }
}

impl<'l, 'r, T: 'l> MapMut<'l, VectorSlice<'r, T>> for MutVector<T> {
    type Output = &'l mut Self;

    type Inner = T;

    type Index = usize;

    fn map_mut<F>(&'l mut self, other: &VectorSlice<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        for idx in 0..self.len() {
            self.list[idx] = funct(&self.list[idx], &other.vector.list[idx])
        }

        self
    }

    fn map_enumerate_mut<F>(&'l mut self, other: &VectorSlice<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        for idx in 0..self.len() {
            self.list[idx] = funct(idx, &self.list[idx], &other.vector.list[idx])
        }

        self
    }
}

impl<'l, 'r, T: 'l> MapMut<'l, MutVectorSlice<'r, T>> for MutVector<T> {
    type Output = &'l mut Self;

    type Inner = T;

    type Index = usize;

    fn map_mut<F>(&'l mut self, other: &MutVectorSlice<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(&Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        for idx in 0..self.len() {
            self.list[idx] = funct(&self.list[idx], &other.vector.list[idx])
        }

        self
    }

    fn map_enumerate_mut<F>(&'l mut self, other: &MutVectorSlice<'r, T>, funct: F) -> Self::Output
    where
        F: Fn(Self::Index, &Self::Inner, &Self::Inner) -> Self::Inner
    {
        if self.len() != other.len() { 
            panic!("Cannot map vectors of different lengths.")
        }

        for idx in 0..self.len() {
            self.list[idx] = funct(idx, &self.list[idx], &other.vector.list[idx])
        }

        self
    }
}