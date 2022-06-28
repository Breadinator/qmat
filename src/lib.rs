#![warn(clippy::all, clippy::pedantic)]

pub mod errors;
/// Contains methods for getting identity matricies and scalars.
pub mod identities;
/// Contains the `Matrix` struct and most associated methods.
pub mod mat;
/// Contains the `Position` struct.
pub mod position;
mod serialization;

#[macro_use]
mod new_matrix;

mod math;

// Convenience re-export of common members
pub mod prelude {
    pub use crate::identity;
    pub use crate::mat::Matrix;
    pub use crate::matrix;
    pub use crate::vector;
}
