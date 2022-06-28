#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug)]
pub enum NewMatrixError {
    /// If the given arguments aren't internally consistent (i.e. if `rows*cols!=data.len()`)
    IllegalArguments,

    /// If the generics aren't internally consistent (i.e. if `M*N!=LEN`)
    IllegalGenerics,

    /// If the generics don't match the given arguments
    GenericArgumentMismatch,
}

#[derive(Debug)]
pub enum MatrixOperationError {
    /// If the determinant was invalid for the attempted operation.
    InvalidDeterminant,
}
