# qmat
[![Version](https://img.shields.io/crates/v/qmat)](https://crates.io/crates/qmat)
[![Docs](https://img.shields.io/docsrs/qmat)](https://docs.rs/qmat/latest)
[![codecov](https://codecov.io/gh/Breadinator/qmat/branch/main/graph/badge.svg?token=5351LB1WAN)](https://codecov.io/gh/Breadinator/qmat)
[![Build Status](https://img.shields.io/github/workflow/status/Breadinator/qmat/Rust)](https://github.com/Breadinator/qmat/actions/workflows/rust.yml)
[![open issues](https://img.shields.io/github/issues-raw/Breadinator/qmat)](https://github.com/Breadinator/qmat/issues)
[![License](https://img.shields.io/github/license/Breadinator/qmat)](https://github.com/Breadinator/qmat/blob/main/LICENSE)
![Code Size](https://img.shields.io/github/languages/code-size/Breadinator/qmat)

**qmat** is a simple library for 2-dimensional matrices.

## Usage
### New matrix
There are three main ways to create a new matrix.

```rust
use qmat::prelude::*;

// Creates the matrix 2x3
//     [0, 1, 2]
//     [3, 4, 5]
// The generics are the data type, the number of rows, the
// number of cols then the lenth of the data (rows * cols) 
let mat: Matrix<i32, 2, 3, 6> = Matrix::new([0, 1, 2, 3, 4, 5]).unwrap();

// Or,
let mat = Matrix::<_, 2, 3, 6>::new([0, 1, 2, 3, 4, 5]).unwrap();
```

```rust
use qmat::prelude::*;

// Creates the same matrix using the analagous macro pattern.
// Automatically unwraps the errors.
let mat = matrix!(2, 3, [0, 1, 2, 3, 4, 5]);
```

```rust
use qmat::prelude::*;
let mat = matrix!([[0, 1, 2], [3, 4, 5]]);
```

Matrices can also be created using [Matrix::empty](https://docs.rs/qmat/latest/qmat/mat/struct.Matrix.html#method.empty) and [Matrix::diag](https://docs.rs/qmat/latest/qmat/mat/struct.Matrix.html#method.diag).

### Retrieving a value
#### Using a [usize; 2]
```rust
use qmat::prelude::*;
let mat = matrix!([[0, 1, 2], [3, 4, 5]]);
println!("{}", mat[[1, 1]]); // 4
```

#### Using the position struct
```rust
use qmat::prelude::*;
let mat = matrix!([[0, 1, 2], [3, 4, 5]]);
let pos = Position(0, 2);
println!("{}", mat[pos]); // 2
```

### Matrix operations
### Iterators

## Todo
* Implement mutable row and col iterators
* Implement inverting for matrices that aren't of the shape (2, 2)
* Allow indexing for anything that can be converted into [usize; 2]
* Optimise
* Add examples for matrix operations and iterators to README.md