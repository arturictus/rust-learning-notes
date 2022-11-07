// Deref and DerefMut
struct Selector<T> {
    /// Elements available in this `Selector`.
    elements: Vec<T>,

    /// The index of the "current" element in `elements`. A `Selector`
    /// behaves like a pointer to the current element.
    current: usize,
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

#[test]
fn test_selector() {
    let mut s = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 2,
    };

    // Because `Selector` implements `Deref`, we can use the `*` operator to
    // refer to its current element.
    assert_eq!(*s, 'z');

    // Assert that 'z' is alphabetic, using a method of `char` directly on a
    // `Selector`, via deref coercion.
    assert!(s.is_alphabetic());

    // Change the 'z' to a 'w', by assigning to the `Selector`'s referent.
    *s = 'w';

    assert_eq!(s.elements, ['x', 'y', 'w']);
}

struct Appellation {
    name: String,
    nicknames: Vec<String>,
}
impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hashset_default() {
        use std::collections::HashSet;
        let squares = [4, 9, 16, 25, 36, 49, 64];
        let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>) =
            squares.iter().partition(|&n| n & (n - 1) == 0);

        println!("{:?}", powers_of_two);
        println!("{:?}", impure);
        assert_eq!(powers_of_two.len(), 3);
        assert_eq!(impure.len(), 4);
    }
    #[test]
    fn test_using_appellation() {
        let mut a = Appellation {
            name: "Zeus".to_string(),
            nicknames: vec![
                "cloud collector".to_string(),
                "king of the gods".to_string(),
            ],
        };

        println!("before assignment");
        a = Appellation {
            name: "Hera".to_string(),
            nicknames: vec![],
        };
        println!("at end of block");
    }
}
