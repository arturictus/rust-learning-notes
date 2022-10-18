#![allow(dead_code)]
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Result, Write};
mod generic_bounds;
mod iterators;
mod other;

pub fn main() {}

// ---- Subtraits -------------
/// Someone in the game world, either the player or some other
/// pixie, gargoyle, squirrel, ogre, etc.
trait Visible {
    fn show(&self) -> Result<()>;
}
// Can not implement Creature without Visible
trait Creature: Visible {
    fn position(&self) -> (i32, i32);
    fn facing(&self) -> bool;
}

struct Broom {}

impl Visible for Broom {
    fn show(&self) -> Result<()> {
        Ok(())
    }
}
impl Creature for Broom {
    fn position(&self) -> (i32, i32) {
        (1, 2)
    }
    fn facing(&self) -> bool {
        true
    }
}

// --------------------------------------
pub fn save_configuration(config: &HashMap<String, String>) -> std::io::Result<()> {
    // Create a JSON serializer to write the data to a file.
    let writer = File::create(config_filename())?;
    let mut serializer = serde_json::Serializer::new(writer);
    // The serde `.serialize()` method does the rest.
    config.serialize(&mut serializer)?;

    Ok(())
}
pub fn read_configuration() -> Result<HashMap<String, String>> {
    let reader = File::open(config_filename()).unwrap();
    let de = serde_json::from_reader(reader).unwrap();

    // The serde `.serialize()` method does the rest.
    println!("{:?}", de);
    Ok(de)
}

fn config_filename() -> String {
    String::from("./config_file.json")
}

#[test]
#[ignore]
fn test_save_configuration() {
    let mut config = HashMap::new();
    config.insert("key".to_string(), "value".to_string());
    match save_configuration(&config) {
        Ok(_) => println!("printed"),
        Err(msg) => println!("{}", msg),
    }
}
#[test]
fn test_read_configuration() {
    let mut config = HashMap::new();
    config.insert("key".to_string(), "value".to_string());
    save_configuration(&config).unwrap();
    assert_eq!(read_configuration().unwrap(), config);
}
// --------------------------------------

pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // Claim to have successfully written the whole buffer.
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        // Ok(())
        todo!()
    }
}

// --------------------------------------

/// Given two values, pick whichever one is less.
fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 <= value2 {
        value1
    } else {
        value2
    }
}

#[test]
fn test_min() {
    let one = vec![1, 2];
    let two = vec![1, 3];
    assert_eq!(min(&one, &two), &one);
    let mut letters = vec![];
    letters.write(b"b").expect("nop");
    letters.write(b"c").expect("nopnop");
    letters.write(b"c").ok();
    assert_eq!(letters, vec![98, 99, 99]);
}

// declarations examples

// fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {}

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
