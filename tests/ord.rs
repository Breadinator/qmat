#[test]
fn test_partial_eq() {
    let a = qmat::matrix!(2, 2, [1, 2, 3, 4]);
    let b = qmat::matrix!(2, 2, [1, 2, 3, 4]);
    assert_eq!(a, b);
}

#[test]
fn test_partial_ne() {
    let a = qmat::matrix!(2, 2, [1, 2, 3, 4]);
    let b = qmat::matrix!(2, 2, [1, 2, 2, 4]);
    assert_ne!(a, b);
}
