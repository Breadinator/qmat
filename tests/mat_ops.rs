#[cfg(test)]
mod mul {
    use qmat::matrix;

    #[test]
    fn basic_scalar_2x3() {
        let a = matrix!(2, 3, [1, 5, 3, 2, 6, 7]);
        let b = 4;
        let output = a.mul_scalar(b);
        assert_eq!(*output.as_flat_array(), [4, 20, 12, 8, 24, 28]);
    }

    #[test]
    fn basic_mat_2x2() {
        let a = matrix!(2, 2, [3, 7, 4, 9]);
        let b = matrix!(2, 2, [6, 2, 5, 8]);
        let output = a.multiply::<2, 4, 4>(&b);
        assert_eq!(*output.as_flat_array(), [53, 62, 69, 80]);
    }

    #[test]
    fn basic_mat_3x3() {
        let a = matrix!(3, 3, [12, 8, 4, 3, 17, 14, 9, 8, 10]);
        let b = matrix!(3, 3, [5, 19, 3, 6, 15, 9, 7, 8, 16]);
        let output = a.multiply::<3, 9, 9>(&b);
        assert_eq!(
            *output.as_flat_array(),
            [136, 380, 172, 215, 424, 386, 163, 371, 259]
        );
    }
}

#[cfg(test)]
mod add {
    use qmat::matrix;

    #[test]
    fn basic_mat_2x2() {
        let a = matrix!(3, 3, [12, 8, 4, 3, 17, 14, 9, 8, 10]);
        let b = matrix!(3, 3, [5, 19, 3, 6, 15, 9, 7, 8, 16]);
        let output = a + b;
        assert_eq!(*output.as_flat_array(), [17, 27, 7, 9, 32, 23, 16, 16, 26]);
    }
}

#[cfg(test)]
mod sub {
    use qmat::matrix;

    #[test]
    fn basic_mat_2x2() {
        let a = matrix!(3, 3, [12, 8, 4, 3, 17, 14, 9, 8, 10]);
        let b = matrix!(3, 3, [5, 19, 3, 6, 15, 9, 7, 8, 16]);
        let output = a - b;
        assert_eq!(*output.as_flat_array(), [7, -11, 1, -3, 2, 5, 2, 0, -6]);
    }
}

#[cfg(test)]
mod inverse {
    use qmat::{errors::MatrixOperationError, matrix};

    #[test]
    fn nonsingular_2x2() {
        let mat = matrix!(2, 2, [3.0, 7.0, 1.0, -4.0]);
        let inv = mat.inverse().unwrap();
        assert_eq!(
            *inv.as_flat_array(),
            [4.0 / 19.0, 7.0 / 19.0, 1.0 / 19.0, -3.0 / 19.0]
        )
    }

    #[test]
    fn singular_2x2() {
        let mat = matrix!(2, 2, [3, 6, 2, 4]);
        let res = mat.inverse();
        assert!(res.is_err());
        if let Err(e) = res {
            assert_eq!(e, MatrixOperationError::InvalidDeterminant);
        }
    }
}
