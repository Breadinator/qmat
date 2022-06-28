use rand::Rng;
use qmat::{identities::Identity, mat::Matrix};

#[cfg(test)]
mod matricies {
    use super::*;
    const TIMES: usize = 10;

    #[test]
    fn test_2x2() {
        let mut rng = rand::thread_rng();
        for _ in 0..TIMES {
            let data: [i32; 4] = rng.gen();
            let matrix = Matrix::<_, 2, 2, 4>::new(data).unwrap();
            let id: Matrix<i32, 2, 2, 4> = Matrix::identity();
            assert_eq!(matrix.as_flat_array(), matrix.multiply(&id).as_flat_array());
        }
    }

    // test for the create macro is in `tests::new_matrix`
}

#[cfg(test)]
mod primitives {
    use super::*;
    const TIMES: usize = 100;

    macro_rules! test_random_gen {
        ($T:ty) => {
            paste::item! {
                #[test]
                fn [< test_ $T >]() {
                    let mut rng = rand::thread_rng();
                    for _ in 0..TIMES {
                        let x: $T = rng.gen();
                        assert_eq!(x, x * $T::identity());
                    }
                }
            }
        };
    }

    test_random_gen!(i8);
    test_random_gen!(i16);
    test_random_gen!(i32);
    test_random_gen!(i64);
    test_random_gen!(i128);
    test_random_gen!(isize);

    test_random_gen!(u8);
    test_random_gen!(u16);
    test_random_gen!(u32);
    test_random_gen!(u64);
    test_random_gen!(u128);
    test_random_gen!(usize);

    test_random_gen!(f32);
    test_random_gen!(f64);
}