pub enum Error {
    MatrixOversize(&'static str),
    RangeOverflow(&'static str),
    IntoMatrix(&'static str)
}