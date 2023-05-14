pub enum Error {
    MatrixOversize(&'static str),
    RangeOverflow(&'static str),
    RangeUnderflow(&'static str)
}