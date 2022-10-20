| Trait                    | Description                                                                                                                      |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------------- |
| Drop                     | Destructors. Cleanup code that Rust runs automatically whenever a value is dropped.                                              |
| Sized                    | Marker trait for types with a fixed size known at compile time, as opposed to types (such as slices) that are dynamically sized. |
| Clone                    | Types that support cloning values.                                                                                               |
| Copy                     | Marker trait for types that can be cloned simply by making a byte-for-byte copy of the memory containing the value.              |
| `Deref` and `DerefMut`   | Traits for smart pointer types.                                                                                                  |
| `Default`                | Types that have a sensible “default value.”                                                                                      |
| `AsRef` and `AsMut`      | Conversion traits for borrowing one type of reference from another.                                                              |
| `Borrow` and `BorrowMut` | Conversion traits, like `AsRef`/`AsMut`, but additionally guaranteeing consistent hashing, ordering, and equality.               |
| `From` and `Into`        | Conversion traits for transforming one type of value into another.                                                               |
| `TryFrom` and `TryInto`  | Conversion traits for transforming one type of value into another, for transformations that might fail.                          |
|   `ToOwned`                        |        Conversion trait for converting a reference to an owned value.                                                                                                                          |
