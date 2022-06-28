#[cfg(test)]
mod multiplication {
    #[test]
    fn mul_scalar() {
        let mut mat = qmat::matrix!(2, 2, [3, 5, 2, 7]);
        mat = mat.mul_scalar(2);
        assert_eq!(*mat.as_flat_array(), [6, 10, 4, 14]);
    }
}

/// Rounding floats to nearest thousandth.
#[cfg(test)]
mod inverse {
    fn round(x: f64) -> f64 {
        (x * 1000.0).round() / 1000.0
    }

    #[test]
    fn test_2x2_a() {
        let mat: qmat::mat::Matrix<f64, 2, 2, 4> = qmat::matrix!(2, 2, [4.0, 7.0, 2.0, 6.0]);
        let inv = mat.inverse().unwrap();
        assert_eq!(round(inv[[0, 0]]), 0.6);
        assert_eq!(round(inv[[0, 1]]), -0.7);
        assert_eq!(round(inv[[1, 0]]), -0.2);
        assert_eq!(round(inv[[1, 1]]), 0.4);
    }
}
