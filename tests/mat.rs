use qmat::{errors::NewMatrixError, matrix, prelude::Matrix};
use rand::Rng;

#[test]
fn new_with_illegal_generics() {
    let res = Matrix::<i32, 2, 2, 5>::new([0; 5]);
    assert!(res.is_err());
    if let Err(e) = res {
        assert_eq!(e, NewMatrixError::IllegalGenerics);
    }
}

#[test]
fn as_flat_array() {
    let data = [0, 1, 2, 3, 4, 5];
    let mat = matrix!(2, 3, data);

    #[allow(clippy::needless_range_loop)]
    for i in 0..6 {
        assert_eq!(data[i], mat.as_flat_array()[i]);
    }
}

#[test]
fn rows_cols_vol() {
    const M: usize = 5;
    const N: usize = 3;
    let mat = matrix!(M, N, [0; 15]);
    assert_eq!(mat.rows(), M);
    assert_eq!(mat.cols(), N);
    assert_eq!(mat.vol(), M * N);
}

#[test]
fn get_row() {
    let mat = matrix!(2, 3, [0, 1, 2, 3, 4, 5]);
    assert_eq!(*mat.get_row(0).as_flat_array(), [0, 1, 2]);
    assert_eq!(*mat.get_row(1).as_flat_array(), [3, 4, 5]);
}

#[test]
fn get_col() {
    let mat = matrix!(2, 3, [0, 1, 2, 3, 4, 5]);
    assert_eq!(*mat.get_col(0).as_flat_array(), [0, 3]);
    assert_eq!(*mat.get_col(1).as_flat_array(), [1, 4]);
    assert_eq!(*mat.get_col(2).as_flat_array(), [2, 5]);
}

#[test]
fn index_mut() {
    const TIMES: usize = 20;
    let mut rng = rand::thread_rng();
    for i in 0..TIMES {
        let mut mat = matrix!(2, 3, [0, 1, 2, 3, 4, 5]);
        let row = i % mat.rows();
        let col = i % mat.cols();
        let val = rng.gen();
        mat[[row, col]] = val;
        assert_eq!(mat[[row, col]], val);
    }
}

#[test]
fn diag() {
    let mat: Matrix<_, 10, 10, 100> = Matrix::diag(4);
    for i in 0..10 {
        for j in 0..10 {
            if i == j {
                assert_eq!(mat[[i, j]], 4);
            } else {
                assert_eq!(mat[[i, j]], 0);
            }
        }
    }
}
