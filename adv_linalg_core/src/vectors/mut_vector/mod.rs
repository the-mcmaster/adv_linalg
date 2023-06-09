use crate::matrices::{MatrixBlock, MutMatrix, MutMatrixBlock};
use crate::{Vector, Matrix};
use crate::error::Error;

use super::{MutVector, MutVectorSlice, VectorSlice};

mod add;
mod mul;
mod sub;
mod lambda;
mod map;

impl<T> MutVector<T> {
    pub fn len(&self) -> usize {
        self.list.len()
    }
}

impl<T> From<Vec<T>> for MutVector<T> {
    fn from(list: Vec<T>) -> Self {
        MutVector {
            list
        }
    }
}

impl<T> From<&Vec<T>> for MutVector<T>
where
    T: Clone
{
    fn from(list: &Vec<T>) -> Self {
        MutVector {
            list : list.clone()
        }
    }
}

impl<T> From<Vector<T>> for MutVector<T> {
    fn from(mut_vector: Vector<T>) -> Self
    {
        MutVector {
            list: mut_vector.list
        }
    }
}

impl<T> From<&Vector<T>> for MutVector<T>
where
    T: Clone
{
    fn from(mut_vector: &Vector<T>) -> Self
    {
        MutVector {
            list: mut_vector.list.clone()
        }
    }
}

impl<'a, T> From<VectorSlice<'a, T>> for MutVector<T>
where
    T: Clone
{
    fn from(vector_slice: VectorSlice<'a, T>) -> Self {
        let params = vector_slice.vector.list
            .split_at(vector_slice.start()).1
            .split_at(vector_slice.end()).0
            .into();

        MutVector {
            list : params
        }
    }
}

impl<'a, T> From<&VectorSlice<'a, T>> for MutVector<T>
where
    T: Clone
{
    fn from(vector_slice: &VectorSlice<'a, T>) -> Self {
        let params = vector_slice.vector.list
            .split_at(vector_slice.start()).1
            .split_at(vector_slice.end()).0
            .into();

        MutVector {
            list : params
        }
    }
}

impl<'a, T> From<MutVectorSlice<'a, T>> for MutVector<T>
where
    T: Clone
{
    fn from(mut_vector_slice: MutVectorSlice<'a, T>) -> Self
    {
        let params = mut_vector_slice.vector.list
            .split_at(mut_vector_slice.start()).1
            .split_at(mut_vector_slice.end()).0
            .into();

        MutVector {
            list: params
        }
    }
}

impl<'a, T> From<&MutVectorSlice<'a, T>> for MutVector<T>
where
    T: Clone
{
    fn from(mut_vector_slice: &MutVectorSlice<'a, T>) -> Self
    {
        let params = mut_vector_slice.vector.list
            .split_at(mut_vector_slice.start()).1
            .split_at(mut_vector_slice.end()).0
            .into();

        MutVector {
            list: params
        }
    }
}

impl<T> TryFrom<Matrix<T>> for MutVector<T>
where
    T: Clone 
{
    type Error = Error;

    fn try_from(matrix: Matrix<T>) -> Result<Self, Self::Error> {
        if matrix.rows == 1 {

            return Ok(
                MutVector {
                    list: matrix.matrix[0].clone()
                }
            );

        } else if matrix.cols == 1 {

            let mut params = Vec::with_capacity(matrix.rows);

            for idx in 0..matrix.rows {
                params.push(matrix.matrix[idx][0].clone())
            }

            return Ok(MutVector { list: params })

        } else {

            return Err(Error::MatrixOversize("Cannot convert a matrix into a vector with no singular row or column."));

        }
    }
}

impl<T> TryFrom<&Matrix<T>> for MutVector<T>
where
    T: Clone 
{
    type Error = Error;

    fn try_from(matrix: &Matrix<T>) -> Result<Self, Self::Error> {
        if matrix.rows == 1 {

            return Ok(
                MutVector {
                    list: matrix.matrix[0].clone()
                }
            );

        } else if matrix.cols == 1 {

            let mut params = Vec::with_capacity(matrix.rows);

            for idx in 0..matrix.rows {
                params.push(matrix.matrix[idx][0].clone())
            }

            return Ok(MutVector { list: params })

        } else {

            return Err(Error::MatrixOversize("Cannot convert a matrix into a vector with no singular row or column."));

        }
    }
}

impl<'a, T> TryFrom<MatrixBlock<'a, T>> for MutVector<T>
where
    T: Clone {
    type Error = Error;

    fn try_from(matrix_block: MatrixBlock<'a, T>) -> Result<Self, Self::Error>
    {
        if matrix_block.rows() == 1 {

            let params = matrix_block.matrix.matrix[0]
                .split_at(matrix_block.col_start()).1
                .split_at(matrix_block.col_end()).0
                .into();

            return Ok(
                MutVector {
                    list: params
                }
            );

        } else if matrix_block.cols() == 1 {

            let mut params = Vec::with_capacity(matrix_block.rows());

            for idx in matrix_block.row_start()..matrix_block.row_end() {
                params.push(matrix_block.matrix.matrix[matrix_block.col_start()][idx].clone())
            }

            return Ok(MutVector { list: params })

        } else {

            return Err(Error::MatrixOversize("Cannot convert a matrix into a vector with no singular row or column."));

        }
    }
}

impl<'a, T> TryFrom<&MatrixBlock<'a, T>> for MutVector<T>
where
    T: Clone {
    type Error = Error;

    fn try_from(matrix_block: &MatrixBlock<'a, T>) -> Result<Self, Self::Error>
    {
        if matrix_block.rows() == 1 {

            let params = matrix_block.matrix.matrix[0]
                .split_at(matrix_block.col_start()).1
                .split_at(matrix_block.col_end()).0
                .into();

            return Ok(
                MutVector {
                    list: params
                }
            );

        } else if matrix_block.cols() == 1 {

            let mut params = Vec::with_capacity(matrix_block.rows());

            for idx in matrix_block.row_start()..matrix_block.row_end() {
                params.push(matrix_block.matrix.matrix[matrix_block.col_start()][idx].clone())
            }

            return Ok(MutVector { list: params })

        } else {

            return Err(Error::MatrixOversize("Cannot convert a matrix into a vector with no singular row or column."));

        }
    }
}

impl<T> TryFrom<MutMatrix<T>> for MutVector<T>
where
    T: Clone
{
    type Error = Error;

    fn try_from(mut_matrix: MutMatrix<T>) -> Result<Self, Self::Error> {
        if mut_matrix.rows == 1 {

            return Ok(
                MutVector {
                    list: mut_matrix.matrix[0].clone()
                }
            );

        } else if mut_matrix.cols == 1 {

            let mut params = Vec::with_capacity(mut_matrix.rows);

            for idx in 0..mut_matrix.rows {
                params.push(mut_matrix.matrix[idx][0].clone())
            }

            return Ok(MutVector { list: params })

        } else {

            return Err(Error::MatrixOversize("Cannot convert a matrix into a vector with no singular row or column."));

        }
    }
}

impl<T> TryFrom<&MutMatrix<T>> for MutVector<T>
where
    T: Clone
{
    type Error = Error;

    fn try_from(mut_matrix: &MutMatrix<T>) -> Result<Self, Self::Error> {
        if mut_matrix.rows == 1 {

            return Ok(
                MutVector {
                    list: mut_matrix.matrix[0].clone()
                }
            );

        } else if mut_matrix.cols == 1 {

            let mut params = Vec::with_capacity(mut_matrix.rows);

            for idx in 0..mut_matrix.rows {
                params.push(mut_matrix.matrix[idx][0].clone())
            }

            return Ok(MutVector { list: params })

        } else {

            return Err(Error::MatrixOversize("Cannot convert a matrix into a vector with no singular row or column."));

        }
    }
}

impl<'a, T> TryFrom<MutMatrixBlock<'a, T>> for MutVector<T>
where
    T: Clone {
    type Error = Error;

    fn try_from(matrix_block: MutMatrixBlock<'a, T>) -> Result<Self, Self::Error>
    {
        if matrix_block.rows() == 1 {

            let params = matrix_block.matrix.matrix[0]
                .split_at(matrix_block.col_start()).1
                .split_at(matrix_block.col_end()).0
                .into();

            return Ok(
                MutVector {
                    list: params
                }
            );

        } else if matrix_block.cols() == 1 {

            let mut params = Vec::with_capacity(matrix_block.rows());

            for idx in matrix_block.row_start()..matrix_block.row_end() {
                params.push(matrix_block.matrix.matrix[matrix_block.col_start()][idx].clone())
            }

            return Ok(MutVector { list: params })

        } else {

            return Err(Error::MatrixOversize("Cannot convert a matrix into a vector with no singular row or column."));

        }
    }
}

impl<'a, T> TryFrom<&MutMatrixBlock<'a, T>> for MutVector<T>
where
    T: Clone {
    type Error = Error;

    fn try_from(matrix_block: &MutMatrixBlock<'a, T>) -> Result<Self, Self::Error>
    {
        if matrix_block.rows() == 1 {

            let params = matrix_block.matrix.matrix[0]
                .split_at(matrix_block.col_start()).1
                .split_at(matrix_block.col_end()).0
                .into();

            return Ok(
                MutVector {
                    list: params
                }
            );

        } else if matrix_block.cols() == 1 {

            let mut params = Vec::with_capacity(matrix_block.rows());

            for idx in matrix_block.row_start()..matrix_block.row_end() {
                params.push(matrix_block.matrix.matrix[matrix_block.col_start()][idx].clone())
            }

            return Ok(MutVector { list: params })

        } else {

            return Err(Error::MatrixOversize("Cannot convert a matrix into a vector with no singular row or column."));

        }
    }
}