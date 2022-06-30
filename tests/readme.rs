#![allow(unused)]

#[test]
fn new_matrix_1() {
    use qmat::prelude::*;
    let mat: Matrix<i32, 2, 3, 6> = Matrix::new([0, 1, 2, 3, 4, 5]).unwrap();
    let mat = Matrix::<_, 2, 3, 6>::new([0, 1, 2, 3, 4, 5]).unwrap();
}

#[test]
fn new_matrix_2() {
    use qmat::prelude::*;
    let mat = matrix!(2, 3, [0, 1, 2, 3, 4, 5]);
}

#[test]
fn new_matrix_3() {
    use qmat::prelude::*;
    let mat = matrix!([[0, 1, 2], [3, 4, 5]]);
}

#[test]
fn indexing_usize_2() {
    use qmat::prelude::*;
    let mat = matrix!([[0, 1, 2], [3, 4, 5]]);
    assert_eq!(mat[[1, 1]], 4);
}

#[test]
fn indexing_pos() {
    use qmat::prelude::*;
    let mat = matrix!([[0, 1, 2], [3, 4, 5]]);
    let pos = Position(0, 2);
    assert_eq!(mat[pos], 2);
}
