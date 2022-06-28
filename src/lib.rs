#![warn(clippy::all, clippy::pedantic)]

/// Contains the `Matrix` struct and most associated methods.
pub mod mat;
mod serialization;
/// Contains methods for getting identity matricies and scalars. 
pub mod identities;
pub mod errors;

#[macro_use]
mod new_matrix;

mod math;

// Convenience re-export of common members 
pub mod prelude {
    pub use crate::matrix;
    pub use crate::vector;
    pub use crate::identity;
    pub use crate::mat::Matrix;
}