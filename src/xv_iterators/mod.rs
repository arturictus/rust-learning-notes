#[cfg(test)]
mod test {
    use super::*;

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
}
