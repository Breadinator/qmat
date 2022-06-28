#[test]
fn det_2x2_nonzero_a() {
    let mat = qmat::matrix!(2, 2, [3, 7, 1, -4]);
    assert_eq!(mat.det(), -19);
}

#[test]
fn det_2x2_singular_a() {
    let mat = qmat::matrix!(2, 2, [3, 6, 2, 4]);
    assert_eq!(mat.det(), 0);
}

#[test]
fn det_3x3_nonzero_a() {
    let mat = qmat::matrix!(3, 3, [2, -3, 1, 2, 0, -1, 1, 4, 5]);
    assert_eq!(mat.det(), 49)
}

#[test]
fn det_3x3_nonzero_b() {
    let mat = qmat::matrix!(3, 3, [1, 3, 2, -3, -1, -3, 2, 3, 1]);
    assert_eq!(mat.det(), -15)
}

#[test]
fn det_3x3_nonzero_c() {
    let mat = qmat::matrix!(3, 3, [1, 2, 3, 3, 2, 1, 2, 1, 3]);
    assert_eq!(mat.det(), -12)
}
