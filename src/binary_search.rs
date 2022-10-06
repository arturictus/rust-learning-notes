fn binary_search<T>(col: &[T], item: T) -> Option<T>
where
    T: Eq + PartialOrd + Clone,
{
    do_binary_search(col, item, 0, (&col.len() - 1) as u32, |a, b| a == b)
}
fn do_binary_search<T, F>(col: &[T], item: T, left: u32, right: u32, eq: F) -> Option<T>
where
    T: Eq + PartialOrd + Clone,
    F: Fn(&T, &T) -> bool,
{
    if right >= left {
        let mid = left + (right - left) / 2;
        let x = col[mid as usize].clone();
        if eq(&x, &item) {
            return Some(x);
        }
        if x > item {
            return do_binary_search(col, item, left, mid - 1, eq);
        }
        return do_binary_search(col, item, mid + 1, right, eq);
    } else {
        None
    }
}

#[test]
fn test_binary_search() {
    let data = vec![1, 2, 3, 4, 5];
    assert_eq!(binary_search(&data, 3), Some(3));
    assert_eq!(binary_search(&data, 5), Some(5));
    assert_eq!(binary_search(&data, 10), None);
    let data = vec!["a", "b", "C"];
    assert_eq!(binary_search(&data, "a"), Some("a"));
    assert_eq!(binary_search(&data, "z"), None);
}
