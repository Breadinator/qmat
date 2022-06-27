#![warn(clippy::all, clippy::pedantic)]

use std::{ops::{Add, Index, Sub, Mul, IndexMut}, iter::Sum};

use super::errors::NewMatrixError;
use super::math::arr_dot;

/// A matrix of `M` rows and `N` columns. <br/>
/// `LEN` is the length of the internal array `data: [T; LEN]` that stores all the elements (i.e. `LEN` = `M` * `N`).
#[derive(Debug, Clone, Copy)]
pub struct Matrix<T, const M: usize, const N: usize, const LEN: usize> {
    data: [T; LEN]
}

impl<T, const M: usize, const N: usize, const LEN: usize> Matrix<T, M, N, LEN> {
    /// Creates a new Matrix from given dimensions and flat data.
    /// 
    /// # Errors
    /// * `NewMatrixError::IllegalGenerics` if `M * N != LEN`
    /// 
    /// # Examples
    /// ```rust
    /// use qmat::prelude::*;
    /// let mat = Matrix::<_, 3, 2, 6>::new([4, 2, 10, 5, 5, 6]);
    /// assert!(mat.is_ok());
    /// ```
    pub fn new(data: [T; LEN]) -> Result<Self, NewMatrixError> {
        if M*N != LEN {
            return Err(NewMatrixError::IllegalGenerics);
        }
        Ok(Matrix { data })
    }
}

impl<T: Default+Copy, const M: usize, const N: usize, const LEN: usize> Matrix<T, M, N, LEN> {
    /// # Errors
    /// * `NewMatrixError::` if `rows * cols != data.len()`
    /// * `NewMatrixError::IllegalGenerics` if `M * N != LEN`
    /// * `NewMatrixError::GenericArgumentMismatch` if `M != rows`, `N != cols`, `LEN != data.len()`
    /// 
    /// # Examples
    /// ```rust
    /// use qmat::prelude::*;
    /// let mat = Matrix::<i32, 5, 5, 25>::empty().unwrap();
    /// println!("{:?}", mat);
    /// ```
    pub fn empty() -> Result<Self, NewMatrixError> {
        Self::new([Default::default(); LEN])
    }

    /// Gets a specific row of the matrix.
    ///  
    /// # Panics
    /// * When it fails to make an empty matrix.
    /// 
    /// # Examples
    /// ```rust
    /// use qmat::prelude::*;
    /// let mat = Matrix::<_, 2, 2, 4>::new([0, 1, 2, 3]).unwrap(); // [[0, 1], [2, 3]]
    /// assert_eq!(mat.get_row(0)[[0, 0]], 0);
    /// assert_eq!(mat.get_row(0)[[0, 1]], 1);
    /// assert_eq!(mat.get_row(1)[[0, 0]], 2);
    /// assert_eq!(mat.get_row(1)[[0, 1]], 3);
    /// ```
    #[must_use]
    pub fn get_row(&self, row: usize) -> Matrix<T, N, 1, N> {
        let mut output = Matrix::empty().unwrap();
        for i in 0..N {
            output.data[i] = self.data[row*M+i];
        }
        output
    }

    /// Gets a specific column of the matrix.
    ///  
    /// # Panics
    /// * When it fails to make an empty matrix.
    /// 
    /// # Examples
    /// ```rust
    /// use qmat::prelude::*;
    /// let mat = Matrix::<_, 2, 2, 4>::new([0, 1, 2, 3]).unwrap(); // [[0, 1], [2, 3]]
    /// assert_eq!(mat.get_col(0)[[0, 0]], 0);
    /// assert_eq!(mat.get_col(0)[[0, 1]], 2);
    /// assert_eq!(mat.get_col(1)[[0, 0]], 1);
    /// assert_eq!(mat.get_col(1)[[0, 1]], 3);
    /// ```
    #[must_use]
    pub fn get_col(&self, col: usize) -> Matrix<T, M, 1, M> {
        let mut output = Matrix::empty().unwrap();
        for row in 0..N {
            output.data[row] = self[[row, col]];
        }
        output
    }
}

impl<T: Default+Copy+Mul+Add+std::iter::Sum<<T as std::ops::Mul>::Output>+std::iter::Sum<<T as std::ops::Add>::Output>, const M: usize, const N: usize, const LEN: usize> Matrix<T, M, N, LEN> {
    /// Turbofish `::<O, Q, RES_LEN>` where 
    /// * `O` is the number of columns in the other matrix, 
    /// * `Q` is the array length in the other matrix, 
    /// * `RES_LEN` is the number of elements in the resulting matrix (`M` * `O`) where `M` is rows in `self`
    /// 
    /// # Panics
    /// * When it fails to make an empty matrix.
    /// 
    /// # Examples
    /// ```rust
    /// use qmat::prelude::*;
    /// 
    /// let a = Matrix::<_, 2, 2, 4>::new([3, 4, 2, 1]).unwrap();
    /// let b = Matrix::<_, 2, 2, 4>::new([1, 5, 3, 7]).unwrap();
    /// let output = a.multiply::<2, 4, 4>(&b);
    /// 
    /// assert_eq!(output[[0, 0]], 15);
    /// assert_eq!(output[[0, 1]], 43);
    /// assert_eq!(output[[1, 0]], 5);
    /// assert_eq!(output[[1, 1]], 17);
    /// ```
    pub fn multiply<const O: usize, const Q: usize, const RES_LEN: usize>(self, other: &Matrix<T, N, O, Q>) -> Matrix<T, M, O, RES_LEN> {
        let mut out: Matrix<T, M, O, RES_LEN> = Matrix::empty().unwrap();

        for row in 0..N {
            for col in 0..O {
                out[[row, col]] = self.get_row(row).dot(&other.get_col(col));
            }
        }

        out
    }
}

impl<T, const M: usize, const N: usize, const LEN: usize> Index<[usize; 2]> for Matrix<T, M, N, LEN> {
    type Output = T;

    /// # Examples
    /// ```
    /// use qmat::prelude::*;
    /// let mat = Matrix::<_, 2, 3, 6>::new([0, 1, 2, 3, 4, 5]).unwrap();
    /// assert_eq!(mat[[0, 0]], 0); // [0, 0] => 0*3 + 0 = 0
    /// assert_eq!(mat[[0, 1]], 1); // [0, 1] => 0*3 + 1 = 1
    /// assert_eq!(mat[[0, 2]], 2); // [0, 2] => 0*3 + 2 = 2
    /// assert_eq!(mat[[1, 0]], 3); // [0, 0] => 1*3 + 0 = 3
    /// assert_eq!(mat[[1, 1]], 4); // [1, 1] => 1*3 + 1 = 4
    /// assert_eq!(mat[[1, 2]], 5); // [2, 2] => 1*3 + 2 = 5
    /// ```
    fn index(&self, pos: [usize; 2]) -> &Self::Output {
        // it should panic anyways, no need to add an extra check
        //assert!(pos[0] <= M);
        //assert!(pos[1] <= N);
        &self.data[pos[0]*N + pos[1]]
    }
}

impl<T, const M: usize, const N: usize, const LEN: usize> IndexMut<[usize; 2]> for Matrix<T, M, N, LEN> {
    /// # Examples
    /// ```
    /// use qmat::prelude::*;
    /// let mut mat = Matrix::<_, 2, 3, 6>::new([0, 1, 2, 3, 4, 5]).unwrap();
    /// mat[[0, 2]] = 12;
    /// assert_eq!(mat[[0, 2]], 12);
    /// ```
    fn index_mut(&mut self, pos: [usize; 2]) -> &mut Self::Output {
        // it should panic anyways, no need to add an extra check
        //assert!(pos[0] <= M);
        //assert!(pos[1] <= N);
        &mut self.data[pos[0]*N + pos[1]]
    }
}

impl<T: Add+Add<Output = T>+Default+Copy, const M: usize, const N: usize, const LEN: usize> Add for Matrix<T, M, N, LEN> {
    type Output = Self;

    /// # Examples
    /// ```rust
    /// use qmat::prelude::*;
    /// 
    /// let lhs = Matrix::<i32, 2, 2, 4>::new([3, 17, 128, 5]).unwrap();
    /// let rhs = Matrix::<i32, 2, 2, 4>::new([63, 12, 4, 3]).unwrap();
    /// let added = lhs + rhs;
    /// 
    /// assert_eq!(added[[0, 0]], 66); // 3 + 63
    /// assert_eq!(added[[0, 1]], 29); // 17 + 12
    /// assert_eq!(added[[1, 0]], 132); // 128 + 4
    /// assert_eq!(added[[1, 1]], 8); // 5 + 3
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        let mut added = Self::empty().unwrap();

        for i in 0..LEN {
            added.data[i] = self.data[i] + rhs.data[i]; 
        }

        added
    }
}

impl<T: Sub+Sub<Output = T>+Default+Copy, const M: usize, const N: usize, const LEN: usize> Sub for Matrix<T, M, N, LEN> {
    type Output = Self;

    /// # Examples
    /// ```rust
    /// use qmat::prelude::*;
    /// 
    /// let lhs = Matrix::<i32, 2, 2, 4>::new([3, 17, 128, 5]).unwrap();
    /// let rhs = Matrix::<i32, 2, 2, 4>::new([63, 12, 4, 3]).unwrap();
    /// let subbed = lhs - rhs;
    /// 
    /// assert_eq!(subbed[[0, 0]], -60);  // 3 - 63
    /// assert_eq!(subbed[[0, 1]], 5);  // 17 - 12
    /// assert_eq!(subbed[[1, 0]], 124); // 128 - 4
    /// assert_eq!(subbed[[1, 1]], 2);   // 5 - 3
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        let mut added = Self::empty().unwrap();
        for i in 0..LEN {
            added.data[i] = self.data[i] - rhs.data[i]; 
        }
        added
    }
}

impl<T: Add+Mul+Sum<<T as Add>::Output>+Copy+Default+Sum<<T as Mul>::Output>, const M: usize> Matrix<T, M, 1, M> {
    /// # Examples
    /// ```rust
    /// use qmat::prelude::*;
    ///
    /// let vec1 = Matrix::<i32, 3, 1, 3>::new([2, 4, 3]).unwrap();
    /// let vec2 = Matrix::<i32, 3, 1, 3>::new([1, 3, 3]).unwrap();
    /// 
    /// assert_eq!(vec1.dot(&vec2), 23);
    /// ```
    #[must_use]
    pub fn dot(&self, other: &Self) -> T {
        //self.data.iter().enumerate().map(|(i, x)| {*x * other.data[i]}).sum()
        arr_dot(self.data, other.data)
    }
}