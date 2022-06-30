#[cfg(test)]
mod matrix {
    #[test]
    fn basic_2x2() {
        let mat = qmat::matrix!(2, 2, [0, 1, 2, 3]);
        assert_eq!(mat[[0, 0]], 0);
        assert_eq!(mat[[0, 1]], 1);
        assert_eq!(mat[[1, 0]], 2);
        assert_eq!(mat[[1, 1]], 3);
    }

    #[test]
    fn basic_4x7() {
        let mut data = [0; 28];
        #[allow(clippy::needless_range_loop)] // for whatever reason didn't work with an iterator
        for i in 0..28 {
            data[i] = i;
        }

        let mat = qmat::matrix!(4, 7, data);
        for (i, val) in mat.as_flat_array().iter().enumerate() {
            assert_eq!(i, *val);
        }
    }

    #[test]
    fn tokentree_2x3() {
        let mat = qmat::matrix!([[0, 1, 2], [3, 4, 5]]);
        assert_eq!(mat.rows(), 2);
        assert_eq!(mat.cols(), 3);
        assert_eq!(mat.vol(), 6);
        assert_eq!(mat[[0, 0]], 0);
        assert_eq!(mat[[0, 1]], 1);
        assert_eq!(mat[[0, 2]], 2);
        assert_eq!(mat[[1, 0]], 3);
        assert_eq!(mat[[1, 1]], 4);
        assert_eq!(mat[[1, 2]], 5);
    }
}

#[cfg(test)]
mod identity {
    macro_rules! test_identity {
        ($T:ty, $M:expr, $ID:expr, $NOTID:expr) => {
            paste::item! {
                #[test]
                fn [<test_ $T _ $M x $M>]() {
                    let mat = qmat::identity!($T, $M);
                    for i in 0..$M {
                        for j in 0..$M {
                            if i == j {
                                assert_eq!(mat[[i, j]], $ID);
                            } else {
                                assert_eq!(mat[[i, j]], $NOTID);
                            }
                        }
                    }
                }
            }
        };
    }

    macro_rules! test_identities_up_to_5 {
        ($T:ty, $ID:expr, $NOTID:expr) => {
            test_identity!($T, 1, $ID, $NOTID);
            test_identity!($T, 2, $ID, $NOTID);
            test_identity!($T, 3, $ID, $NOTID);
            test_identity!($T, 4, $ID, $NOTID);
            test_identity!($T, 5, $ID, $NOTID);
        };
    }

    test_identities_up_to_5!(i8, 1, 0);
    test_identities_up_to_5!(i16, 1, 0);
    test_identities_up_to_5!(i32, 1, 0);
    test_identities_up_to_5!(i64, 1, 0);
    test_identities_up_to_5!(i128, 1, 0);
    test_identities_up_to_5!(isize, 1, 0);

    test_identities_up_to_5!(u8, 1, 0);
    test_identities_up_to_5!(u16, 1, 0);
    test_identities_up_to_5!(u32, 1, 0);
    test_identities_up_to_5!(u64, 1, 0);
    test_identities_up_to_5!(u128, 1, 0);
    test_identities_up_to_5!(usize, 1, 0);

    test_identities_up_to_5!(f32, 1.0, 0.0);
    test_identities_up_to_5!(f64, 1.0, 0.0);
}

#[test]
fn empty() {
    let mat = qmat::empty!(4, 3, i8).unwrap();
    assert_eq!(mat.rows(), 4);
    assert_eq!(mat.cols(), 3);
    assert_eq!(*mat.as_flat_array(), [0i8; 12]);
}
