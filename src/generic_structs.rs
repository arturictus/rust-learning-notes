#![allow(dead_code)]
use std::f64::consts::FRAC_PI_2; // π/2

pub fn main() {
    queue_examples();
    poly_examples();
    extrema_examples();
}
#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}
fn poly_examples() {
    let poly = Polynomial::<3>::new([3.4f64, 3.4f64, 3.4f64]);
    println!("{:#?}", poly);
    println!("{:#?}", poly.eval(2.4f64));

    // Approximate the `sin` function: sin x ≅ x - 1/6 x³ + 1/120 x⁵
    // Around zero, it's pretty accurate!
    let sine_poly = Polynomial::new([0.0, 1.0, 0.0, -1.0 / 6.0, 0.0, 1.0 / 120.0]);
    println!("{:#?}", sine_poly);
    assert_eq!(sine_poly.eval(0.0), 0.0);
    assert!((sine_poly.eval(FRAC_PI_2) - 1.).abs() < 0.005);
}
/// A polynomial of degree N - 1.
#[derive(Debug)]
struct Polynomial<const N: usize> {
    /// The coefficients of the polynomial.
    ///
    /// For a polynomial a + bx + cx² + ... + zxⁿ⁻¹,
    /// the `i`'th element is the coefficient of xⁱ.
    coefficients: [f64; N],
}

impl<const N: usize> Polynomial<N> {
    fn new(coefficients: [f64; N]) -> Polynomial<N> {
        Polynomial { coefficients }
    }

    /// Evaluate the polynomial at `x`.
    fn eval(&self, x: f64) -> f64 {
        // Horner's method is numerically stable, efficient, and simple:
        // c₀ + x(c₁ + x(c₂ + x(c₃ + ... x(c[n-1] + x c[n]))))
        let mut sum = 0.0;
        for i in (0..N).rev() {
            sum = self.coefficients[i] + x * sum;
        }

        sum
    }
}

fn extrema_examples() {
    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    println!("{:#?}", e);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);
}

#[derive(Debug)]
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[i];
        }
        if slice[i] > *greatest {
            greatest = &slice[i];
        }
    }
    Extrema { greatest, least }
}

fn queue_examples() {
    let mut q = Queue::new();
    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);
    assert!(q.is_empty());
    q.push('☉');
    assert!(!q.is_empty());
    let mut q = Queue::new();
    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');

    let (older, younger) = q.split();
    // q is now uninitialized.
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);

    let mut bq = Box::new(Queue::new());
    bq.push('o');
    assert_eq!(bq.pop(), Some('o'));

    // f64

    let mut qn = Queue::new();
    qn.push(9.0f64);
    qn.push(6.0f64);
    assert_eq!(qn.sum(), 15.0);
    print(&qn);
    print(&qn);
}
/// A first-in, first-out queue of characters.
#[derive(Debug)]
struct Queue<T> {
    older: Vec<T>,   // older elements, eldest last.
    younger: Vec<T>, // younger elements, youngest last.
}

fn print(e: &Queue<f64>) {
    println!("{:#?}", e);
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }
    /// Push a character onto the back of a queue.
    pub fn push(&mut self, c: T) {
        self.younger.push(c);
    }

    /**
    Pop a character off the front of a queue. Return `Some(c)` if there
    was a character to pop, or `None` if the queue was empty.
    */
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            // Bring the elements in younger over to older, and put them in
            // the promised order.
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        // Now older is guaranteed to have something. Vec's pop method
        // already returns an Option, so we're set.
        self.older.pop()
    }
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

impl Queue<f64> {
    fn sum(&self) -> f64 {
        let mut out = 0.0f64;
        self.younger.iter().for_each(|n| {
            out += n;
        });
        self.older.iter().for_each(|n| {
            out += n;
        });
        out
    }
}
