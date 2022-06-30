use qmat::prelude::*;

#[test]
fn into_iter_row() {
    let mat: Matrix<i32, 3, 3, 9> = matrix!([[0, 1, 2], [3, 4, 5], [6, 7, 8]]);
    let mut iter = mat.into_iter_row();

    assert_eq!(
        iter.next(),
        Some(Matrix::<i32, 3, 1, 3>::new([0, 1, 2]).unwrap())
    );
    assert_eq!(
        iter.next(),
        Some(Matrix::<i32, 3, 1, 3>::new([3, 4, 5]).unwrap())
    );
    assert_eq!(
        iter.next(),
        Some(Matrix::<i32, 3, 1, 3>::new([6, 7, 8]).unwrap())
    );

    assert_eq!(iter.next(), None);
}

#[test]
fn iter_row() {
    const ROW_1: [i32; 3] = [0, 1, 2];
    const ROW_2: [i32; 3] = [3, 4, 5];
    const ROW_3: [i32; 3] = [6, 7, 8];

    let mat: Matrix<i32, 3, 3, 9> = matrix!([ROW_1, ROW_2, ROW_3]);
    let mut iter = mat.iter_row();

    assert_eq!(
        iter.next(),
        Some(Matrix::<&i32, 3, 1, 3>::new([&0, &1, &2]).unwrap())
    );
    assert_eq!(
        iter.next(),
        Some(Matrix::<&i32, 3, 1, 3>::new([&3, &4, &5]).unwrap())
    );
    assert_eq!(
        iter.next(),
        Some(Matrix::<&i32, 3, 1, 3>::new([&6, &7, &8]).unwrap())
    );

    assert_eq!(iter.next(), None);
}

#[test]
fn iter_row_mut() {}

#[test]
fn into_iter_col() {
    let mat: Matrix<i32, 3, 3, 9> = matrix!([[0, 1, 2], [3, 4, 5], [6, 7, 8]]);
    let mut iter = mat.into_iter_col();

    assert_eq!(
        iter.next(),
        Some(Matrix::<i32, 1, 3, 3>::new([0, 3, 6]).unwrap())
    );
    assert_eq!(
        iter.next(),
        Some(Matrix::<i32, 1, 3, 3>::new([1, 4, 7]).unwrap())
    );
    assert_eq!(
        iter.next(),
        Some(Matrix::<i32, 1, 3, 3>::new([2, 5, 8]).unwrap())
    );

    assert_eq!(iter.next(), None);
}

#[test]
fn iter_col() {
    const ROW_1: [i32; 3] = [0, 1, 2];
    const ROW_2: [i32; 3] = [3, 4, 5];
    const ROW_3: [i32; 3] = [6, 7, 8];

    let mat: Matrix<i32, 3, 3, 9> = matrix!([ROW_1, ROW_2, ROW_3]);
    let mut iter = mat.iter_col();

    assert_eq!(
        iter.next(),
        Some(Matrix::<&i32, 1, 3, 3>::new([&0, &3, &6]).unwrap())
    );
    assert_eq!(
        iter.next(),
        Some(Matrix::<&i32, 1, 3, 3>::new([&1, &4, &7]).unwrap())
    );
    assert_eq!(
        iter.next(),
        Some(Matrix::<&i32, 1, 3, 3>::new([&2, &5, &8]).unwrap())
    );

    assert_eq!(iter.next(), None);
}

#[test]
fn iter_col_mut() {}
