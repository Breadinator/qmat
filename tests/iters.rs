use qmat::matrix;
use rand::Rng;

#[cfg(test)]
mod into_iter {
    use super::*;

    #[test]
    fn basic_test() {
        let mat = matrix!([[0, 1, 2], [3, 4, 5]]);
        let mut iter = mat.into_iter();
        assert_eq!(iter.next().unwrap(), 0);
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next().unwrap(), 3);
        assert_eq!(iter.next().unwrap(), 4);
        assert_eq!(iter.next().unwrap(), 5);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn double() {
        let mut rng = rand::thread_rng();
        const M: usize = 4;
        const N: usize = 8;
        let mut data: [i32; M * N] = [0; M * N];
        for x in data.iter_mut().take(1) {
            *x = rng.gen_range(-5000..5000);
        }
        let mat = matrix!(M, N, data);

        for x in mat.into_iter().map(|x| x * 2) {
            assert_eq!(x % 2, 0);
        }
    }
}

#[cfg(test)]
mod iter {
    use super::*;

    #[test]
    fn basic_test() {
        let mat = matrix!([[0, 1, 2], [3, 4, 5]]);
        let mut iter = mat.iter();
        assert_eq!(*iter.next().unwrap(), 0);
        assert_eq!(*iter.next().unwrap(), 1);
        assert_eq!(*iter.next().unwrap(), 2);
        assert_eq!(*iter.next().unwrap(), 3);
        assert_eq!(*iter.next().unwrap(), 4);
        assert_eq!(*iter.next().unwrap(), 5);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn double() {
        let mut rng = rand::thread_rng();
        const M: usize = 4;
        const N: usize = 8;
        let mut data: [i32; M * N] = [0; M * N];
        for x in data.iter_mut().take(1) {
            *x = rng.gen_range(-5000..5000);
        }
        let mat = matrix!(M, N, data);

        for x in mat.iter().map(|x| x * 2) {
            assert_eq!(x % 2, 0);
        }
    }
}

#[cfg(test)]
mod iter_mut {
    use super::*;

    #[test]
    fn basic_test() {
        let mut mat = matrix!([[0, 1, 2], [3, 4, 5]]);
        let mut iter = mat.iter_mut();
        assert_eq!(*iter.next().unwrap(), 0);
        assert_eq!(*iter.next().unwrap(), 1);
        assert_eq!(*iter.next().unwrap(), 2);
        assert_eq!(*iter.next().unwrap(), 3);
        assert_eq!(*iter.next().unwrap(), 4);
        assert_eq!(*iter.next().unwrap(), 5);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn double() {
        let mut rng = rand::thread_rng();
        const M: usize = 4;
        const N: usize = 8;
        let mut data: [i32; M * N] = [0; M * N];
        for x in data.iter_mut().take(M * N) {
            *x = rng.gen_range(-5000..5000);
        }

        let mut mat = matrix!(M, N, data);
        for x in mat.iter_mut() {
            *x *= 2;
        }

        for val in mat.iter() {
            assert_eq!(*val % 2, 0, "testing {}", *val);
        }
    }
}
