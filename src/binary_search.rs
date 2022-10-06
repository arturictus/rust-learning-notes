// example:
// https://www.geeksforgeeks.org/binary-search/
// fn binary_search<T: Eq + PartialOrd + Add>(

fn binary_search(col: &Vec<u32>, item: u32, low: u32, max: u32) -> Option<u32> {
    if max >= low {
        let mid = low + (max - low) / 2;
        let x = col[mid as usize].clone();
        println!("current element: {}, current index: {}", x, mid);
        if x == item {
            println!("found");
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
