use crate::mat::Matrix;

/// Defines a value where `x * T::identity() == x`.
pub trait Identity {
    fn identity() -> Self;
}

impl<T: Default+Copy+Identity, const M: usize, const LEN: usize> Identity for Matrix<T, M, M, LEN> {
    /// # Panics
    /// * If it fails to make an empty matrix
    #[must_use]
    fn identity() -> Self {
        Self::diag(T::identity())
    }
}

macro_rules! id {
    ($T:ty, $VAL:expr) => {
        impl Identity for $T {
            #[must_use]
            fn identity() -> Self {
                $VAL
            }
        }
    };
}

/*
 * Numeric primitives
 */
macro_rules! intid {
    ($T:ty) => {
        id!($T, 1);
    };
}

// Signed integers
intid!(i8);
intid!(i16);
intid!(i32);
intid!(i64);
intid!(i128);
intid!(isize);

// Unsigned integers
intid!(u8);
intid!(u16);
intid!(u32);
intid!(u64);
intid!(u128);
intid!(usize);

// Floating point
id!(f32, 1.0);
id!(f64, 1.0);