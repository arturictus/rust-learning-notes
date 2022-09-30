use std::fmt::Debug;
use std::hash::Hash;
use std::io::Write;

pub fn main() {
    let one = vec![1, 2];
    let two = vec![1, 3];
    println!("{:?}", min(one, two));
    let mut letters = vec![];
    letters.write(b"b").expect("nop");
    letters.write(b"c").expect("nopnop");
    letters.write(b"c").ok();
    println!("{:?}", letters);
    dbg!(letters);
}

/// Given two values, pick whichever one is less.
fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 <= value2 {
        value1
    } else {
        value2
    }
}

// declarations examples

fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {}

/*
/// Run a query on a large, partitioned data set.
/// See <http://research.google.com/archive/mapreduce.html>.
fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(
    data: &DataSet,
    map: M,
    reduce: R,
) -> Results {
}

fn run_query<M, R>(data: &DataSet, map: M, reduce: R) -> Results
    where M: Mapper + Serialize,
          R: Reducer + Serialize
{ ... }
*/
