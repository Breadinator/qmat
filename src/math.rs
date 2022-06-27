use std::{ops::Mul, iter::Sum};

pub fn arr_dot<T: Mul+ Sum<<T as Mul>::Output>+Copy, const M: usize>(a: [T; M], b: [T; M]) -> T {
    a.iter().enumerate().map(|(i, x)| {*x * b[i]}).sum()
}