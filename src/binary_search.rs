// example:
// https://www.geeksforgeeks.org/binary-search/
// fn binary_search<T: Eq + PartialOrd + Add>(

use std::ops::Add;

fn binary_search<T: Eq + PartialOrd + Add + Clone>(
    col: &Vec<T>,
    item: T,
    low: u32,
    max: u32,
) -> Option<T> {
    if max >= low {
        let mid = low + (max - low) / 2;
        let x = col[mid as usize].clone();
        if x == item {
            return Some(item);
        }
        if x > item {
            return binary_search(col, item, low, mid - 1);
        }
        return binary_search(col, item, mid + 1, max);
    } else {
        None
    }
}

#[test]
fn test_binary_search() {
    let data = vec![1, 2, 3, 4, 5];
    assert_eq!(
        binary_search(&data, 3, 0, (&data.len() - 1) as u32),
        Some(3)
    );
    assert_eq!(
        binary_search(&data, 5, 0, (&data.len() - 1) as u32),
        Some(5)
    );
}
