use num::Num;
use std::ops::{Add, Mul};

fn dot1<N>(v1: &[N], v2: &[N]) -> N
where
    N: Add<Output = N> + Mul<Output = N> + Default + Copy,
{
    let mut total = N::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

// Using crate Num which gives all the Num traits in one
fn dot<N: Num + Copy>(v1: &[N], v2: &[N]) -> N {
    let mut total = N::zero();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[test]
fn test_dot() {
    assert_eq!(dot1(&[1], &[2]), 2);
    assert_eq!(dot1(&[1, 3, 7], &[2, 8, 5]), 61);
    assert_eq!(dot1(&[1.5, 3.0, 7.3], &[2.6, 8.0, 5.4]), 67.32);

    assert_eq!(dot(&[1], &[2]), 2);
    assert_eq!(dot(&[1, 3, 7], &[2, 8, 5]), 61);
    assert_eq!(dot(&[1.5, 3.0, 7.3], &[2.6, 8.0, 5.4]), 67.32);
}
