pub fn main() {}

// Using the type to extract lenght
fn dot_product<const N: usize>(a: [f64; N], b: [f64; N]) -> f64 {
    let mut sum = 0.;
    for i in 0..N {
        sum += a[i] * b[i];
    }
    sum
}

#[test]
fn test_dot_product() {
    let mut ans = dot_product([0.2, 0.4, 0.6], [0., 0., 1.]);
    assert_eq!(ans, 0.6);
    // Explicitly provide `3` as the value for `N`.
    ans = dot_product::<3>([0.2, 0.4, 0.6], [0., 0., 1.]);
    assert_eq!(ans, 0.6);
}
