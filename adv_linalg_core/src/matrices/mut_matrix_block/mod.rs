use std::ops::Range;

use crate::matrices::{MutMatrix, MutMatrixBlock};
use crate::error::Error;

mod add;
mod mul;
mod sub;
mod lambda;
mod map;

impl<'a, T> MutMatrixBlock<'a, T>
{
    pub fn new(matrix: &'a MutMatrix<T>, row_range: Range<usize>, col_range: Range<usize>) -> Result<Self, Error> {
        if row_range.is_empty() {
            return Err(Error::RangeUnderflow("Row range was empty"));
        } else if matrix.rows >= row_range.end {
            return Err(Error::RangeOverflow("Row range end exceeded the row length of the relative matrix"))
        }
        
        if col_range.is_empty() {
            return Err(Error::RangeUnderflow("Column range was empty"));
        } else if matrix.cols >= col_range.end {
            return Err(Error::RangeOverflow("Column range end exceeded the column length of the relative matrix"))
        }
        
        Ok(MutMatrixBlock {
            matrix,
            row_range,
            col_range
        })
    }

    pub fn row_range(&mut self, row_range: Range<usize>) -> Result<(), Error> {
        if row_range.is_empty() {
            
            return Err(Error::RangeUnderflow("Row range was empty"))
        
        } else if row_range.end >= self.matrix.rows {
            
            return Err(Error::RangeOverflow("Row range end exceeded the row length of the relative matrix"));
        
        } else {
            
            self.row_range = row_range;
            return Ok(());
        
        }
    }

    pub fn col_range(&mut self, col_range: Range<usize>) -> Result<(), Error> {
        if col_range.is_empty() {
            
            return Err(Error::RangeUnderflow("Column range was empty"))
        
        } else if col_range.end >= self.matrix.cols {
            
            return Err(Error::RangeOverflow("Column range end exceeded the column length of the relative matrix"));
        
        } else {
            
            self.col_range = col_range;
            return Ok(());
        
        }
    }

    pub fn rows(&self) -> usize {
        self.row_range.end - self.row_range.start
    }

    pub fn cols(&self) -> usize {
        self.col_range.end - self.col_range.start
    }

    pub fn col_start(&self) -> usize {
        self.col_range.start
    }

    pub fn col_end(&self) -> usize {
        self.col_range.end
    }

    pub fn row_start(&self) -> usize {
        self.row_range.start
    }

    pub fn row_end(&self) -> usize {
        self.row_range.end
    }
}

impl<'a, T> From<&'a MutMatrix<T>> for MutMatrixBlock<'a, T> {
    fn from(matrix: &'a MutMatrix<T>) -> Self {
        MutMatrixBlock {
            matrix,
            row_range: 0..matrix.rows,
            col_range: 0..matrix.cols
        }
    }
}