#![cfg_attr(not(any(doc, test, feature = "std")), no_std)]

//! Arbitrary precision scientific number
//!
//! # Constants
//!
//! Use `Scientific!` in the crate `scientific-macro` to create constant numbers.
//!
//! ```rust
//! use scientific_macro::Scientific;
//! let n1 = Scientific!(1e100);
//! let n2 = Scientific!(1e80);
//! assert_eq!(&n1 + &n2, Scientific!(1.00000000000000000001e100));
//! // An f64 has only a precision of about 15.9 digits, this are already 21.
//! ```
//!
//! # Invocation
//!
//! All functions expect a reference to the [`Scientific`](crate::Scientific) number. (See example above.)
//!
//! # Conversion
//!
//! There are `From` and `TryFrom` traits for conversion between [`Scientific`](crate::Scientific) and integers, floats and strings.
//!
//! Converting a scientific number with decimals to an integer will fail.
//!
//! There is a `FromStr` instance (which clones the `str` and calls [`Scientific::from_string`](crate::Scientific::from_string)).
//!
//! The functions [`Scientific::to_bytes`](crate::Scientific::to_bytes) and [`Scientific::from_bytes`](crate::Scientific::from_bytes) use a compressed representation and not ASCII
//! (this format will also be used when using serde with non human-readable formats).
//!
//! # Precision
//!
//! Most functions work in truly arbitrary precision, please be aware of this.
//!
//! For example: adding 1e1000 and 1e-1000, which both have only one byte of mantissa, results in 2001 bytes of mantissa.
//!
//! [`Scientific::div`](crate::Scientific::div), and [`Scientific::sqrt`](crate::Scientific::sqrt) (which depends on div) as also [`Scientific::round`](crate::Scientific::round) require
//! a precision to be specified, the result is only calculated to that precision.
//!
//! It can be specified as [`Decimals`](crate::Decimals) or [`Digits`](crate::Digits). When using decimals specify the number of decimal places to
//! calculate (`2` for `0.01` as the smallest number, `0` for `1` and `-2` for `100`). When using digits specify
//! the number of digits in the mantissa (using <= 0 digits will always result in zero).
//!
//! Shortcuts: [`Precision::INTEGER`](crate::Precision::INTEGER) for integer calculations (aka `Decimals(0)`) and [`Precision::F64`](crate::Precision::F64) for
//! calculations with a slightly better precision as an f64 (aka `Digits(16)`).
//!
//! # Shifting
//!
//! The shifting operators do shift by one digit (and not one bit as you may expected).
//!
//! # Rounding
//!
//! There are versions of `div` and `round` which support several rounding options. See [`Rounding`](crate::Rounding) and [`div_r`](crate::Scientific::div_r).
//!
//! # Features
//!
//! - `serde`: Enable De-/Serialization with serde.
//!
//! - `std`: If activated the library requires `std` and the [`Error`](::std::error::Error]) trait is implemented for all error types.
//!   Without it the library is `no_std`.
//!
//! - `arc`: Use of [`Arc`](::alloc::sync::Arc) instead of [`Rc`](::alloc::rc::Rc), which enables [`Send`](::core::marker::Send) and [`Sync`](::core::marker::Sync) for [`Scientific`](crate::Scientific).
//!   Though [`Arc`](::alloc::sync::Arc) is more expensive, but since it's only used during create/clone/drop of
//!   the [`Scientific`](crate::Scientific) number it's probably not that much.
//!
//! - `debug`: Enabled tracking of pointer operations and some more checks. Very helpful during development
//!   of this lib.
//!
//! # Exponent
//!
//! The exponent is represented as an [`isize`](::core::isize). It is expected that it will never under-/overflow,
//! even when smaller numbers are added/subtracted, like e.g. the length of the mantissa.
//!
//! This is not checked!

#[macro_use]
extern crate alloc;

pub use crate::types::conversion_error::ConversionError;
pub use crate::types::error::Error;
pub use crate::types::precision::Precision::{self, Decimals, Digits};
pub use crate::types::rounding::{
  Round::{
    self, RoundAwayFromZero, RoundDown, RoundHalfAwayFromZero, RoundHalfDown, RoundHalfToEven,
    RoundHalfToOdd, RoundHalfTowardsZero, RoundHalfUp, RoundUp,
  },
  Rounding, Truncate,
};
pub use crate::types::scientific::Scientific;

#[doc(hidden)]
pub mod __private;
pub(crate) mod conversion;
pub(crate) mod math;
pub(crate) mod ptr;
#[cfg(test)]
mod tests;
pub(crate) mod types;
