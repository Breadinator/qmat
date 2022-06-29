#![warn(clippy::all, clippy::pedantic)]

/// Creates a matrix of shape (`$M`, `$N`) with flat data `$data`.
///
/// # Examples
/// ## From dimensions and flat data
/// ```rust
/// let data = [0, 1, 2, 3];
/// let mat = qmat::matrix!(2, 2, data);
/// assert_eq!(mat[[1, 0]], 2);
/// ```
///
/// ## From nested rows
/// ```rust
/// let mat = qmat::matrix!([
///     [0, 1],
///     [2, 3]
/// ]);
/// assert_eq!(mat[[1, 1]], 3);
/// ```
#[macro_export]
macro_rules! matrix {
    ($M:expr, $N:expr, $data:expr) => {
        $crate::mat::Matrix::<_, $M, $N, { $M * $N }>::new($data).unwrap()
    };
    ($rows:tt) => {{
        const M: usize = { $rows.len() };
        if M == 0 {
            panic!("no data provided");
        }
        const N: usize = { $rows[0].len() };
        const LEN: usize = { M * N };
        $crate::mat::Matrix::<_, M, N, LEN>::from_rows($rows).unwrap()
    }};
}

/// Creates a `$M`x`$M` [identity matrix](https://en.wikipedia.org/wiki/Identity_matrix) of type `$T`.
///
/// # Examples
/// ```rust
/// let mat = qmat::identity!(i32, 3);
/// assert_eq!(mat[[0,0]], 1);
/// assert_eq!(mat[[1,1]], 1);
/// assert_eq!(mat[[2,2]], 1);
/// ```
#[macro_export]
macro_rules! identity {
    ($T:ty, $M:expr) => {{
        use $crate::identities::Identity;
        $crate::mat::Matrix::<$T, $M, $M, { $M * $M }>::identity()
    }};
}

/// Creates a 1-dimensional matrix, i.e. a vector.
///
/// # Examples
/// ```rust
/// let data = [0, 1, 2, 3];
/// let mat = qmat::vector!(4, data);
/// assert_eq!(mat[[0,0]], 0);
/// assert_eq!(mat[[0,1]], 1);
/// assert_eq!(mat[[0,2]], 2);
/// assert_eq!(mat[[0,3]], 3);
/// ```
#[macro_export]
macro_rules! vector {
    ($M:expr, $data:expr) => {
        $crate::mat::Matrix::<_, $M, 1, $M>::new($data).unwrap()
    };
}

#[macro_export]
macro_rules! empty {
    ($M:expr, $N:expr, $T:ty) => {
        $crate::mat::Matrix::<$T, $M, $N, { $M * $N }>::empty()
    };
}
