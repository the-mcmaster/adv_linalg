use std::ops::Range;

mod matrix;
mod mut_matrix;
mod matrix_block;
mod mut_matrix_block;

pub struct Matrix<T> {
    pub(crate) matrix : Vec<Vec<T>>
}

pub struct MatrixBlock<'a, T> {
    pub(crate) matrix : &'a Matrix<T>
}

pub struct MutMatrix<T> {
    pub(crate) matrix : Vec<Vec<T>>
}

pub struct MutMatrixBlock<'a, T> {
    pub(crate) matrix : &'a mut MutMatrix<T>,
    pub(crate) row_range : Range<usize>,
    pub(crate) col_range : Range<usize>
}