#![warn(clippy::all, clippy::pedantic)]

use std::{
    iter::Sum,
    ops::{Add, Index, IndexMut, Mul, Sub},
};

use crate::{
    errors::{MatrixOperationError, NewMatrixError},
    identities::Identity,
    math::arr_dot,
    position::Position,
};

/// A matrix of `M` rows and `N` columns. <br/>
/// `LEN` is the length of the internal array `data: [T; LEN]` that stores all the elements (i.e. `LEN` = `M` * `N`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize, const LEN: usize> {
    data: [T; LEN],
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
        if M * N != LEN {
            return Err(NewMatrixError::IllegalGenerics);
        }
        Ok(Matrix { data })
    }

    /// Returns an immutable reference to the underlying 1-dimensional data.
    ///
    /// Flattened such that the matrix <br/>
    /// &nbsp;&nbsp;&nbsp;&nbsp;`[a, b, c]` <br/>
    /// &nbsp;&nbsp;&nbsp;&nbsp;`[d, e, f]`<br/>
    /// becomes `[a, b, c, d, e, f]`.
    #[must_use]
    pub fn as_flat_array(&self) -> &[T; LEN] {
        &self.data
    }

    /// Returns the constant number of rows, `M`.
    #[must_use]
    #[allow(clippy::unused_self)] // so you can call someMatrix.rows()
    pub fn rows(&self) -> usize {
        M
    }

    /// Returns the constant number of columns, `N`.
    #[must_use]
    #[allow(clippy::unused_self)] // so you can call someMatrix.cols()
    pub fn cols(&self) -> usize {
        N
    }

    /// The number of elements in the matrix (i.e. the number of rows times the number of cols).
    #[must_use]
    #[allow(clippy::unused_self)] // so you can call someMatrix.vol()
    pub fn vol(&self) -> usize {
        LEN
    }

    /// Iterates over immutable references to all of the elements of a matrix.
    pub fn iter(&self) -> Iter<T, M, N, LEN> {
        Iter {
            data: &self.data,
            i: 0,
        }
    }

    /// TODO: implement myself.
    /// Currently just passes the `iter_mut` call to the underlying array.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }

    /// Creates an iterator for the matrix's rows by moving matrix ownership.
    pub fn into_iter_row(self) -> IntoIterRow<T, M, N, LEN> {
        IntoIterRow {
            i: 0,
            data: self.data,
        }
    }

    /// Iterates over rows, using immutable references to the original array's data.
    pub fn iter_row(&self) -> IterRow<'_, T, M, N, LEN> {
        IterRow {
            i: 0,
            data: &self.data,
        }
    }

    /// Unimplemented method.
    /// Iterates over rows, using mutable references to the original array's data.
    #[allow(clippy::unused_self)]
    pub fn iter_row_mut(&mut self) {
        unimplemented!();
    }

    /// Creates an iterator for the matrix's columns by moving matrix ownership.
    pub fn into_iter_col(self) -> IntoIterCol<T, M, N, LEN> {
        IntoIterCol {
            i: 0,
            data: self.data,
        }
    }

    /// Iterates over columns, using immutable references to the original array's data.
    pub fn iter_col(&self) -> IterCol<'_, T, M, N, LEN> {
        IterCol {
            i: 0,
            data: &self.data,
        }
    }

    /// Iterates over columns, using mutable references to the original array's data.
    /// Unimplemented method.
    #[allow(clippy::unused_self)]
    pub fn iter_col_mut(&mut self) {
        unimplemented!();
    }
}

impl<T, const M: usize, const N: usize, const LEN: usize> Matrix<T, M, N, LEN>
where
    T: Default + Copy,
{
    /// # Errors
    /// * `NewMatrixError::IllegalGenerics` if `M * N != LEN`
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

    /// Errors
    /// * Same as `Matrix::new`
    #[allow(clippy::missing_errors_doc)] // idk why clippy can't read
    pub fn from_rows(data: [[T; N]; M]) -> Result<Self, NewMatrixError> {
        let mut flat_data: [T; LEN] = [Default::default(); LEN];
        let mut i: usize = 0;
        for row in data {
            for elem in row {
                flat_data[i] = elem;
                i += 1;
            }
        }
        Self::new(flat_data)
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
            output.data[i] = self.data[row * N + i];
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
        for row in 0..M {
            output.data[row] = self[[row, col]];
        }
        output
    }
}

impl<T, const M: usize, const N: usize, const LEN: usize> Matrix<T, M, N, LEN>
where
    T: Default + Copy + Mul + Sum<<T as Mul>::Output>,
{
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
    pub fn multiply<const O: usize, const Q: usize, const RES_LEN: usize>(
        &self,
        other: &Matrix<T, N, O, Q>,
    ) -> Matrix<T, M, O, RES_LEN> {
        let mut out: Matrix<T, M, O, RES_LEN> = Matrix::empty().unwrap();

        for row in 0..N {
            for col in 0..O {
                out[[row, col]] = self.get_row(row).dot(&other.get_col(col));
            }
        }

        out
    }
}

impl<T, const M: usize, const N: usize, const LEN: usize> Matrix<T, M, N, LEN>
where
    T: Default + Copy + Mul<Output = T>,
{
    /// Multiplies a matrix with and scalar value.
    /// Iterates over all elements in the matrix and multiplies it by the given scalar.
    ///
    /// # Examples
    /// ```rust
    /// use qmat::prelude::*;
    /// let mat = matrix!{[[0, 1, 2]]};
    /// let res = mat.mul_scalar(3);
    /// assert_eq!(res[[0, 0]], 0);
    /// assert_eq!(res[[0, 1]], 3);
    /// assert_eq!(res[[0, 2]], 6);
    /// ```
    ///
    /// # Panics
    /// * When it fails to make an empty matrix.
    #[must_use]
    pub fn mul_scalar(&self, scalar: T) -> Self {
        let mut out = Matrix::empty().unwrap();
        for i in 0..LEN {
            out.data[i] = self.data[i] * scalar;
        }
        out
    }
}

impl<T, const M: usize, const N: usize, const LEN: usize> Index<Position> for Matrix<T, M, N, LEN> {
    type Output = T;
    fn index(&self, pos: Position) -> &Self::Output {
        &self.data[pos.0 * N + pos.1]
    }
}

impl<T, const M: usize, const N: usize, const LEN: usize> IndexMut<Position>
    for Matrix<T, M, N, LEN>
{
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self.data[pos.0 * N + pos.1]
    }
}

impl<T, const M: usize, const N: usize, const LEN: usize> Index<[usize; 2]>
    for Matrix<T, M, N, LEN>
{
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
        &self.data[pos[0] * N + pos[1]]
    }
}

impl<T, const M: usize, const N: usize, const LEN: usize> IndexMut<[usize; 2]>
    for Matrix<T, M, N, LEN>
{
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
        &mut self.data[pos[0] * N + pos[1]]
    }
}

impl<T, const M: usize, const N: usize, const LEN: usize> Add for Matrix<T, M, N, LEN>
where
    T: Add<Output = T> + Default + Copy,
{
    type Output = Self;

    /// Returns a matrix where element `i` is `lhs[i] + rhs[i]`.
    ///
    /// # Examples
    /// ```rust
    /// use qmat::prelude::*;
    ///
    /// let lhs = matrix!([[3, 17], [128, 5]]);
    /// let rhs = matrix!([[63, 12], [4, 3]]);
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

impl<T, const M: usize> Matrix<T, M, 1, M>
where
    T: Copy + Mul + Sum<<T as Mul>::Output>,
{
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
        arr_dot(*self.as_flat_array(), *other.as_flat_array())
    }
}

impl<T, const M: usize, const N: usize, const LEN: usize> Sub for Matrix<T, M, N, LEN>
where
    T: Sub<Output = T> + Default + Copy,
{
    type Output = Self;

    /// # Examples
    /// ```rust
    /// use qmat::prelude::*;
    ///
    /// let lhs = matrix!([[3, 17], [128, 5]]);
    /// let rhs = matrix!([[63, 12], [4, 3]]);
    /// let subbed = lhs - rhs;
    ///
    /// assert_eq!(subbed[[0, 0]], -60);  // 3 - 63
    /// assert_eq!(subbed[[0, 1]], 5);  // 17 - 12
    /// assert_eq!(subbed[[1, 0]], 124); // 128 - 4
    /// assert_eq!(subbed[[1, 1]], 2);   // 5 - 3
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        let mut subbed = Self::empty().unwrap();
        for i in 0..LEN {
            subbed.data[i] = self.data[i] - rhs.data[i];
        }
        subbed
    }
}

impl<T, const M: usize, const LEN: usize> Matrix<T, M, M, LEN>
where
    T: Default + Copy,
{
    /// Creates a new matrix such that every value in the diagonal from the top left (`[0, 0]`) to the bottom left (`[M, M]`) are equal to `val`.
    ///
    /// # Examples
    /// ```rust
    /// use qmat::prelude::Matrix;
    ///
    /// let mat: Matrix<i32, 3, 3, 9> = Matrix::diag(3);
    /// assert_eq!(mat[[0, 0]], 3);
    /// assert_eq!(mat[[1, 1]], 3);
    /// assert_eq!(mat[[2, 2]], 3);
    /// ```
    ///
    /// # Panics
    /// * If it fails to create an empty matrix.
    #[must_use]
    pub fn diag(val: T) -> Self {
        let mut mat = Self::empty().unwrap();

        for i in 0..M {
            mat[[i, i]] = val;
        }

        mat
    }
}

impl<T, const M: usize, const N: usize, const LEN: usize> Matrix<T, M, N, LEN>
where
    T: num_traits::Num + Copy,
{
    /// Returns the [determinant](https://en.wikipedia.org/wiki/Determinant) of a matrix.
    #[must_use]
    pub fn det(&self) -> T {
        let mat = self;
        let mut mat = *mat; // hopefully dereferences a copy of self?
        let mut temp = [T::zero(); N]; // temp array for row storage
        let mut total: T = T::one();
        let mut det: T = T::one(); // init res

        // loop for traversing diagonal elems
        for i in 0..N {
            let mut index = i;

            // finding non-zero value
            while index < N && self[[index, i]] == T::zero() {
                index += 1;
            }

            if index == N {
                // there is non-zero elem
                // det of matrix is 0
                continue;
            }
            if index != i {
                // loop for swapping the diagonal element row and index row
                for j in 0..N {
                    (mat[[index, j]], mat[[i, j]]) = (mat[[i, j]], mat[[index, j]]);
                }

                // det sign changes when row is shifted
                let exp = index - i;
                if exp % 2 == 1 {
                    det = det * (T::zero() - T::one());
                }
            }

            // storing diagonal row elems
            for j in 0..N {
                temp[j] = mat[[i, j]];
            }

            // traversing every col of row and mul to every row
            for j in (i + 1)..N {
                let num1 = temp[i]; // value of diagonal elem
                let num2 = mat[[j, i]]; // value of next row elem

                // traverse every column of row and mul to every row
                for k in 0..N {
                    // multiply to make the diagonal element and next row element equal
                    mat[[j, k]] = (num1 * mat[[j, k]]) - (num2 * temp[k]);
                }

                total = total * num1; // Det(kA)=Det(A)
            }
        }

        for i in 0..N {
            det = det * mat[[i, i]];
        }

        det / total
    }
}

impl<T, const M: usize, const LEN: usize> Matrix<T, M, M, LEN>
where
    T: num_traits::Num + Copy + Default + Identity,
{
    /// # Errors
    /// * `MatrixOperationError::InvalidDeterminant` is `self.det() == 0`.
    ///
    /// # Panics
    /// * If it fails to create an empty matrix.
    /// * When trying to get the inverse of a matrix that isn't 2x2. (Currently unimplemented.)
    pub fn inverse(&self) -> Result<Self, MatrixOperationError> {
        match M {
            2 => self.inverse_2x2(),
            _ => self.inverse_gauss_jordan(),
        }
    }

    fn inverse_2x2(&self) -> Result<Self, MatrixOperationError> {
        let det = self.det();
        if det.is_zero() {
            return Err(MatrixOperationError::InvalidDeterminant);
        }

        let min1 = T::zero() - T::one();

        let mut augmented = Self::empty().unwrap();
        augmented.data[0] = self.data[3]; // a
        augmented.data[1] = self.data[1] * min1; // b
        augmented.data[2] = self.data[2] * min1; // c
        augmented.data[3] = self.data[0]; // d

        Ok(augmented.mul_scalar(T::one() / det))
    }

    /// <https://www.mathsisfun.com/algebra/matrix-inverse-row-operations-gauss-jordan.html>
    /// <https://www.codesansar.com/numerical-methods/python-program-inverse-matrix-using-gauss-jordan.htm>
    fn inverse_gauss_jordan(&self) -> Result<Self, MatrixOperationError> {
        todo!();
    }
}

impl<T, const M: usize, const N: usize, const LEN: usize> IntoIterator for Matrix<T, M, N, LEN>
where
    T: Clone,
{
    type IntoIter = IntoIter<T, M, N, LEN>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            data: self.data,
            i: 0,
        }
    }
}

#[derive(Debug)]
pub struct IntoIter<T, const M: usize, const N: usize, const LEN: usize> {
    i: usize,
    data: [T; LEN],
}
impl<T, const M: usize, const N: usize, const LEN: usize> Iterator for IntoIter<T, M, N, LEN>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= LEN {
            None
        } else {
            let val = self.data[self.i].clone();
            self.i += 1;
            Some(val)
        }
    }
}

#[derive(Debug)]
pub struct Iter<'a, T, const M: usize, const N: usize, const LEN: usize> {
    i: usize,
    data: &'a [T; LEN],
}
impl<'a, T, const M: usize, const N: usize, const LEN: usize> Iterator for Iter<'a, T, M, N, LEN> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= LEN {
            None
        } else {
            let val = &self.data[self.i];
            self.i += 1;
            Some(val)
        }
    }
}

#[derive(Debug)]
pub struct IntoIterRow<T, const M: usize, const N: usize, const LEN: usize> {
    i: usize,
    data: [T; LEN],
}
impl<T, const M: usize, const N: usize, const LEN: usize> Iterator for IntoIterRow<T, M, N, LEN>
where
    T: Copy + Default,
{
    type Item = Matrix<T, M, 1, M>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= N {
            return None;
        }

        let offset = self.i * M;
        let mut row: [T; M] = [Default::default(); M];

        for (j, elem) in row.iter_mut().enumerate() {
            *elem = self.data[offset + j];
        }

        self.i += 1;

        if let Ok(mat) = Matrix::<T, M, 1, M>::new(row) {
            Some(mat)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct IterRow<'a, T, const M: usize, const N: usize, const LEN: usize> {
    i: usize,
    data: &'a [T; LEN],
}
impl<'a, T, const M: usize, const N: usize, const LEN: usize> Iterator for IterRow<'a, T, M, N, LEN>
where
    T: 'a,
{
    type Item = Matrix<&'a T, M, 1, M>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= N {
            return None;
        }

        let offset = self.i * M;
        let mut row: [&'a T; M] = [&self.data[offset]; M];

        for (j, elem) in row.iter_mut().enumerate() {
            *elem = &self.data[offset + j];
        }

        self.i += 1;

        if let Ok(mat) = Matrix::<&T, M, 1, M>::new(row) {
            Some(mat)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct IntoIterCol<T, const M: usize, const N: usize, const LEN: usize> {
    i: usize,
    data: [T; LEN],
}
impl<T, const M: usize, const N: usize, const LEN: usize> Iterator for IntoIterCol<T, M, N, LEN>
where
    T: Copy + Default,
{
    type Item = Matrix<T, 1, N, N>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= M {
            return None;
        }

        let mut col: [T; N] = [Default::default(); N];

        for (j, elem) in col.iter_mut().enumerate() {
            *elem = self.data[M * j + self.i];
        }

        self.i += 1;

        if let Ok(mat) = Matrix::<T, 1, N, N>::new(col) {
            Some(mat)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct IterCol<'a, T, const M: usize, const N: usize, const LEN: usize> {
    i: usize,
    data: &'a [T; LEN],
}
impl<'a, T, const M: usize, const N: usize, const LEN: usize> Iterator for IterCol<'a, T, M, N, LEN>
where
    T: 'a,
{
    type Item = Matrix<&'a T, 1, N, N>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= N {
            return None;
        }

        let mut col: [&'a T; N] = [&self.data[M + self.i]; N];
        for (j, elem) in col.iter_mut().enumerate() {
            *elem = &self.data[M * j + self.i];
        }

        self.i += 1;

        if let Ok(mat) = Matrix::<&T, 1, N, N>::new(col) {
            Some(mat)
        } else {
            None
        }
    }
}
