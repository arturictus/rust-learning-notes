use rand::random; // In Cargo.toml dependencies: rand = "0.7"
use std::fmt::Debug;
use std::iter::from_fn;

fn dump<T, U>(t: T)
where
    T: IntoIterator<Item = U>,
    U: Debug,
{
    for u in t {
        println!("{:?}", u);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_fn() {
        // Generate the lengths of 1000 random line segments whose endpoints
        // are uniformly distributed across the interval [0, 1]. (This isn't a
        // distribution you're going to find in the `rand_distr` crate, but
        // it's easy to make yourself.)
        let lengths: Vec<f64> = from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
            .take(1000)
            .collect();

        assert_eq!(lengths.len(), 1000);

        for n in lengths {
            assert!(n < 1.0f64)
        }
    }

    #[test]
    fn test_dump() {
        dump(vec![1, 2, 3])
    }

    #[test]
    fn for_example() {
        println!("There's:");
        let v = vec!["antimony", "arsenic", "aluminum", "selenium"];

        for element in &v {
            println!("{}", element);
        }

        let mut iterator = (&v).into_iter();
        while let Some(element) = iterator.next() {
            println!("{}", element);
        }
    }
    #[test]
    fn using_next() {
        let v = vec![4, 20, 12, 8, 6];
        let mut iterator = v.iter();
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), Some(&20));
        assert_eq!(iterator.next(), Some(&12));
        assert_eq!(iterator.next(), Some(&8));
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.next(), None);

        println!("{:?}", v)
    }

    #[test]
    fn using_path() {
        use std::ffi::OsStr;
        use std::path::Path;

        let path = Path::new("C:/Users/JimB/Downloads/Fedora.iso");
        let mut iterator = path.iter();
        assert_eq!(iterator.next(), Some(OsStr::new("C:")));
        assert_eq!(iterator.next(), Some(OsStr::new("Users")));
        assert_eq!(iterator.next(), Some(OsStr::new("JimB")));
    }

    #[test]
    fn using_btree_iterator() {
        // You should usually use HashSet, but its iteration order is
        // nondeterministic, so BTreeSet works better in examples.
        use std::collections::BTreeSet;
        let mut favorites = BTreeSet::new();
        favorites.insert("Lucy in the Sky With Diamonds".to_string());
        favorites.insert("Liebesträume No. 3".to_string());
        println!("{:#?}", favorites);
        let mut it = favorites.into_iter();
        assert_eq!(it.next(), Some("Liebesträume No. 3".to_string()));
        assert_eq!(it.next(), Some("Lucy in the Sky With Diamonds".to_string()));
        assert_eq!(it.next(), None);
    }
}
