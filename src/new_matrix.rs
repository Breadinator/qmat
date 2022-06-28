#![warn(clippy::all, clippy::pedantic)]

/// Creates a matrix of shape (`$M`, `$N`) with flat data `$data`.
/// 
/// # Examples
/// ```rust
/// let data = [0, 1, 2, 3];
/// let mat = qmat::matrix!(2, 2, data);
/// assert_eq!(mat[[1, 0]], 2);
/// ```
#[macro_export]
macro_rules! matrix {
    ($M:expr, $N:expr, $data:expr) => {
        $crate::mat::Matrix::<_, $M, $N, {$M*$N}>::new($data).unwrap()
    };
}

#[macro_export]
macro_rules! identity {
    ($T:ty, $M:expr) => {
        {
            use $crate::identities::Identity;
            $crate::mat::Matrix::<$T, $M, $M, {$M*$M}>::identity()
        }
    };
}