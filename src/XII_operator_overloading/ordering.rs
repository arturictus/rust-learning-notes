use std::cmp::{Ordering, PartialOrd};
#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T, // inclusive
    upper: T, // exclusive
}

impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lower >= other.upper {
            Some(Ordering::Greater)
        } else if self.upper <= other.lower {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Reverse;

    use super::*;
    #[test]
    fn partial_ordering() {
        assert!(
            Interval {
                upper: 20,
                lower: 12,
            } > Interval {
                upper: 11,
                lower: 9,
            },
        );

        assert!(
            Interval {
                lower: 10,
                upper: 20
            } < Interval {
                lower: 20,
                upper: 40
            }
        );
        assert!(Interval { lower: 7, upper: 8 } >= Interval { lower: 0, upper: 1 });
        assert!(Interval { lower: 7, upper: 8 } <= Interval { lower: 7, upper: 8 });
        // Overlapping intervals aren't ordered with respect to each other.
        let left = Interval {
            lower: 10,
            upper: 30,
        };
        let right = Interval {
            lower: 20,
            upper: 40,
        };
        assert!(!(left < right));
        assert!(!(left >= right));
        assert_eq!(left < right, false)
    }

    #[test]
    fn ordering_intervals() {
        let intervals = &mut [
            Interval { lower: 7, upper: 8 },
            Interval { lower: 0, upper: 1 },
        ];

        intervals.sort_by_key(|i| i.upper);
        assert!(
            *intervals
                == [
                    Interval { lower: 0, upper: 1 },
                    Interval { lower: 7, upper: 8 },
                ]
        );
        intervals.sort_by_key(|i| Reverse(i.upper));
        assert!(
            *intervals
                == [
                    Interval { lower: 7, upper: 8 },
                    Interval { lower: 0, upper: 1 },
                ]
        )
    }
}
