use std::ops::Range;

mod matrix;
mod mut_matrix;
mod matrix_block;
mod mut_matrix_block;

pub struct Matrix<T> {
    pub(crate) matrix : Vec<Vec<T>>,
    pub(crate) rows : usize,
    pub(crate) cols : usize
}

pub struct MatrixBlock<'a, T> {
    pub(crate) matrix : &'a Matrix<T>,
    row_range : Range<usize>,
    col_range : Range<usize>
}

pub struct MutMatrix<T> {
    pub(crate) matrix : Vec<Vec<T>>,
    pub(crate) rows : usize,
    pub(crate) cols : usize
}

pub struct MutMatrixBlock<'a, T> {
    pub(crate) matrix : &'a mut MutMatrix<T>,
    row_range : Range<usize>,
    col_range : Range<usize>
}