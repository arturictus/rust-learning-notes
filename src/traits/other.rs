use std::ops::Add;

trait Float {
    const ZERO: Self;
    const ONE: Self;
}

impl Float for f32 {
    const ZERO: f32 = 0.0;
    const ONE: f32 = 1.0;
}

impl Float for f64 {
    const ZERO: f64 = 0.0;
    const ONE: f64 = 1.0;
}

fn add_one<T: Float + Add<Output = T>>(value: T) -> T {
    value + T::ONE
}

fn fib<T: Float + Add<Output = T>>(n: usize) -> T {
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        n => fib::<T>(n - 1) + fib::<T>(n - 2),
    }
}

#[test]
fn test_add_one() {
    assert_eq!(add_one(1.0), 2.0)
}
#[test]
fn test_fib() {
    assert_eq!(fib::<f32>(4), 3.0);
    assert_eq!(fib::<f64>(5), 5.0);
}
