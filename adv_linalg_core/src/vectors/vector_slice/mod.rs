use std::ops::Range;

use crate::Vector;

use crate::error::Error;
use crate::vectors::VectorSlice;

mod add;
mod mul;
mod sub;
mod lambda;
mod map;

impl<'a, T> VectorSlice<'a, T> {
    pub fn new(vector: &'a Vector<T>, slice_range: Range<usize>) -> Result<Self, Error> {
        if slice_range.is_empty() {
            
            return Err(Error::RangeUnderflow("Slice range was empty"))
        
        } else if vector.len() >= slice_range.end {
            
            return Err(Error::RangeOverflow("Slice range end exceeded the length of the relative vector"));
        
        } else {
            
            return Ok(VectorSlice {
                vector,
                slice_range
            });
        
        }
    }

    pub fn slice_range(&mut self, slice_range: Range<usize>) -> Result<(), Error> {
        if slice_range.is_empty() {
            
            return Err(Error::RangeUnderflow("Slice range was empty"))
        
        } else if slice_range.end >= self.vector.len() {
            
            return Err(Error::RangeOverflow("Slice range end exceeded the length of the relative vector"));
        
        } else {
            
            self.slice_range = slice_range;
            return Ok(());
        
        }
    }

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