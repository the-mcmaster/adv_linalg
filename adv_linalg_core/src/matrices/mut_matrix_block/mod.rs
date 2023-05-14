use super::MutMatrixBlock;

mod add;
mod mul;
mod sub;
mod lambda;
mod map;

impl<'a, T> MutMatrixBlock<'a, T> {
    pub fn rows(&self) -> usize {
        self.row_range.end - self.row_range.start
    }

    pub fn cols(&self) -> usize {
        self.col_range.end - self.col_range.start
    }

    pub fn row_start(&self) -> usize {
        self.row_range.start
    }

    pub fn row_end(&self) -> usize {
        self.row_range.end
    }

    pub fn col_start(&self) -> usize {
        self.col_range.start
    }

    pub fn col_end(&self) -> usize {
        self.col_range.end
    }
}