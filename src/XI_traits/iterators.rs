use std::iter;
use std::vec::IntoIter;
fn cyclical_zip_ugly(
    v: Vec<u8>,
    u: Vec<u8>,
) -> iter::Cycle<iter::Chain<IntoIter<u8>, IntoIter<u8>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn cyclical_zip_slow(v: Vec<u8>, u: Vec<u8>) -> Box<dyn Iterator<Item = u8>> {
    Box::new(v.into_iter().chain(u.into_iter()).cycle())
}

fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item = u8> {
    v.into_iter().chain(u.into_iter()).cycle()
}

#[test]
fn test_cyclical_zip() {
    let mut val = cyclical_zip(vec![1, 2, 3], vec![9, 8, 7]);
    assert_eq!(val.next(), Some(1));
    assert_eq!(val.next(), Some(2));
    assert_eq!(val.next(), Some(3));
    assert_eq!(val.next(), Some(9));
    assert_eq!(val.next(), Some(8));
    assert_eq!(val.next(), Some(7));
    assert_eq!(val.next(), Some(1));
}

/// Loop over an iterator, storing the values in a new vector.
fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
    let mut results = Vec::new();
    for value in iter {
        results.push(value);
    }
    results
}

/// Print out all the values produced by an iterator
use std::fmt::Debug;

fn dump<I>(iter: I)
where
    I: Iterator,
    I::Item: Debug,
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

fn dump_str(iter: &mut dyn Iterator<Item = String>) {
    for (index, s) in iter.enumerate() {
        println!("{}: {:?}", index, s);
    }
}

#[test]
fn test_collect_into_vector() {
    let result = collect_into_vector(vec![["1"], ["2"], ["3"]].into_iter());
    assert_eq!(result, vec![["1"], ["2"], ["3"]]);
    let result = collect_into_vector([1, 2, 3].into_iter());
    assert_eq!(result, vec![1, 2, 3]);
}

#[derive(Debug)]
struct Thing(i32);

#[test]
fn test_dump() {
    dump([1, 2, 3].into_iter());
    dump([Thing(1), Thing(12), Thing(3)].into_iter());
    dump_str(
        ["1".to_string(), "2".to_string(), "3".to_string()]
            .into_iter()
            .by_ref(),
    )
}
