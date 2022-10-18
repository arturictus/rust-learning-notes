mod index;
mod ordering;
use std::fmt::Debug;
use std::ops::{Add, Neg};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T>;
    fn neg(self) -> Complex<T> {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}
use std::ops::AddAssign;

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let a = Complex { re: 2, im: 1 };
        let b = Complex { re: 3, im: 2 };
        assert_eq!(a + b, Complex { re: 5, im: 3 });
        assert_eq!(a.neg(), Complex { re: -2, im: -1 });
        // `PartialEq` floats div by 0.0 returns NaN
        assert_eq!(0.0 / 0.0 == 0.0 / 0.0, false);
        // To make it work Type must implement `Eq` trait
        assert!(Complex { re: 0.0, im: 0.0 } == Complex { re: 0.0, im: 0.0 });
        assert!(
            Complex {
                re: "a".to_string(),
                im: "b".to_string()
            } == Complex {
                re: "a".to_string(),
                im: "b".to_string()
            }
        );
        moving(a);
        moving(a)
        // assert_eq!((a + b), Complex { re: 5, im: 3 })
    }

    #[test]
    fn test_add_assign() {
        let a = &mut Complex { re: 2, im: 1 };
        let b = Complex { re: 3, im: 2 };
        a.add_assign(b);

        assert_eq!(*a == Complex { re: 5, im: 3 }, true);
    }

    #[test]
    fn test_partial_eq() {
        assert!(f64::is_nan(0.0 / 0.0));
        assert_eq!(0.0 / 0.0 == 0.0 / 0.0, false);
        assert_eq!(0.0 / 0.0 != 0.0 / 0.0, true);
        assert_eq!(0.0 / 0.0 < 0.0 / 0.0, false);
        assert_eq!(0.0 / 0.0 > 0.0 / 0.0, false);
        assert_eq!(0.0 / 0.0 <= 0.0 / 0.0, false);
        assert_eq!(0.0 / 0.0 >= 0.0 / 0.0, false);
    }

    fn moving<T: Debug>(e: Complex<T>) {
        println!("{:?}", e)
    }
}
