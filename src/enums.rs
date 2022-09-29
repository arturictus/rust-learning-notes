#![allow(dead_code)]
use std::collections::HashMap;

pub fn main() {}

enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

fn timeunit_examples() {
    println!("{}", TimeUnit::Seconds.plural());
    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    println!(
        "four_score_and_seven_years_ago: {}",
        four_score_and_seven_years_ago.humanize()
    );

    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
    println!("three_hours_from_now: {}", three_hours_from_now.humanize());
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    /// Return the plural noun for this time unit.
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    /// Return the singular noun for this time unit.
    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

/// A timestamp that has been deliberately rounded off, so our program
/// says "6 months ago" instead of "February 9, 2016, at 9:49 AM".
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

impl RoughTime {
    fn humanize(self) -> String {
        match self {
            RoughTime::InTheFuture(unit, 1) => {
                format!("{} {} from now", 1, unit.singular())
            }
            RoughTime::InTheFuture(unit, amount) => {
                format!("{} {} from now", amount, unit.plural())
            }
            RoughTime::JustNow => "Just Now".to_string(),
            RoughTime::InThePast(unit, 1) => {
                format!("{} {} in the past", 1, unit.singular())
            }
            RoughTime::InThePast(unit, amount) => {
                format!("{} {} in the past", amount, unit.plural())
            }
        }
    }
}
