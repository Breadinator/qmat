#![warn(clippy::all, clippy::pedantic)]

pub mod mat;
pub mod errors;

#[macro_use]
mod new_matrix;

mod math;

pub mod prelude {
    pub use crate::matrix;
    pub use crate::mat::Matrix;
}