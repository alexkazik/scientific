#![cfg_attr(not(any(doc, test, feature = "std")), no_std)]

//! # Arbitrary precision scientific number
//!
//! ## Constants
//!
//! Use `Scientific!` in the crate `scientific-macro` to create constant numbers.
//!
//! ```
//! use scientific_macro::Scientific;
//! let n1 = Scientific!(1e100);
//! let n2 = Scientific!(1e80);
//! assert_eq!(&n1 + &n2, Scientific!(1.00000000000000000001e100));
//! // An f64 has only a precision of about 15.9 digits, this are already 21.
//! ```
//!
//! ## Invocation
//!
//! All functions expect a reference to the [Scientific] number. (See example above.)
//!
//! ## Conversion
//!
//! There are `From` and `TryFrom` traits for conversion between [Scientific] and integers, floats and strings.
//!
//! Converting a number with decimals to an integer will fail.
//!
//! There is a `FromStr` instance (which clones the `str` and calls [Scientific::from_string]).
//!
//! The functions [Scientific::to_bytes] and [Scientific::from_bytes] use a compressed representation and not ASCII
//! (this format will also be used when using serde and non human-readable formats).
//!
//! ## Precision
//!
//! Most functions work in truly arbitrary precision, please be aware of this.
//!
//! For example: adding 1e1000 and 1e-1000, which both have only one byte of mantissa, results in 2001 bytes of mantissa.
//!
//! [Scientific::div], and [Scientific::sqrt] (which depends on div) as also [Scientific::round] require
//! a precision to be specified, the result is only calculated to that precision.
//!
//! It can be specified as [Decimals] or [Digits]. When using decimals specify the number of decimal places to
//! calculate (`2` for `0.01` as the smallest number, `0` for `1` and `-2` for `100`). When using digits specify
//! the number of digits in the mantissa (using <= 0 digits will always result in zero).
//!
//! Shortcuts: [Precision::INTEGER] for integer calculations (aka `Decimals(0)`) and [Precision::F64] for
//! calculations with a slightly better precision as an f64 (aka `Digits(16)`).
//!
//! ## Features
//!
//! - `std`: If activated the library requires `std` and the [std::error::Error] trait is implemented for all error types.
//!   Without it the library is `no_std`.
//!
//! - `arc`: Use of [Arc](alloc::sync::Arc) instead of [Rc](alloc::rc::Rc), which enables [Send] and [Sync] for [Scientific].
//!   Though [Arc](alloc::sync::Arc) is more expensive, but since it's only used during create/clone/drop of
//!   the [Scientific] number it's probably not that much.
//!
//! - `debug`: Enabled tracking of pointer operations and some more checks. Very helpful during development
//!   of this lib.
//!
//! ## Exponent
//!
//! The exponent is represented as an [isize]. It is expected that it will never under-/overflow,
//! even when smaller numbers are added/subtracted, like e.g. the length of the mantissa.
//!
//! This is not checked!

#[macro_use]
extern crate alloc;

#[doc(hidden)]
pub mod __private;
pub(crate) mod conversion;
pub(crate) mod math;
pub(crate) mod ptr;
#[cfg(test)]
mod tests;
pub(crate) mod types;
pub(crate) mod util;

use crate::__private::unsafe_new;
use crate::conversion::raw_parts::{s_as_raw_mantissa, s_from_raw_parts};
use crate::conversion::string::s_parse;
use crate::math::div::export_div;
use crate::math::div_rem::export_div_rem;
use crate::math::neg::export_neg_assign;
use crate::math::powi::export_powi;
use crate::math::sqrt::export_sqrt;
use crate::ptr::Ptr;
pub use crate::types::conversion_error::ConversionError;
pub use crate::types::error::Error;
use crate::types::owner::Owner;
pub use crate::types::precision::Precision;
pub use crate::types::scientific::Scientific;
use crate::types::sign::Sign;
use crate::util::mantissa::{MANTISSA_1, MANTISSA_5};
pub use crate::Precision::{Decimals, Digits};
use alloc::string::String;
use alloc::vec::Vec;
use conversion::bytes_de::s_from_bytes;
use conversion::bytes_ser::s_to_bytes;
use math::round::export_round;
use math::truncate::export_truncate_assign;

impl Scientific {
  // This constant must not change before 0.5 since scientific-macro depends on it.
  pub const ZERO: Scientific = Scientific {
    sign: Sign::Positive,
    data: Ptr::new_invalid(),
    len: 0,
    exponent: 1, // required for exponent() to work
    owner: Owner::None,
  };
  pub const ONE: Scientific = unsafe_new(false, &MANTISSA_1, 0);
  pub(crate) const POINT5: Scientific = unsafe_new(false, &MANTISSA_5, -1);

  #[inline(always)]
  pub fn from_string(mut source: String) -> Result<Scientific, ConversionError> {
    s_parse(source.as_mut_ptr(), source.len(), Owner::new_string(source))
  }

  #[inline(always)]
  pub fn to_bytes(&self) -> Vec<u8> {
    s_to_bytes(self)
  }

  #[inline(always)]
  pub fn from_bytes(bytes: &[u8]) -> Result<Scientific, ConversionError> {
    s_from_bytes(bytes).map_err(From::from)
  }

  // This function must not change before 0.5 since scientific-macro depends on it.
  #[inline(always)]
  pub fn as_raw_mantissa(&self) -> &[u8] {
    s_as_raw_mantissa(self)
  }

  #[inline(always)]
  pub fn from_raw_parts(
    negative: bool,
    mantissa: Vec<u8>,
    exponent: isize,
  ) -> Result<Scientific, ConversionError> {
    s_from_raw_parts(negative, mantissa, exponent)
  }

  #[inline(always)]
  pub fn sqrt(&self, precision: Precision) -> Result<Scientific, Error> {
    export_sqrt(self, precision)
  }

  #[inline(always)]
  pub fn abs(&self) -> Scientific {
    let mut result = self.clone();
    result.sign = Sign::Positive;
    result
  }

  #[inline(always)]
  pub fn abs_assign(&mut self) {
    self.sign = Sign::Positive;
  }

  #[inline(always)]
  pub fn neg_assign(&mut self) {
    export_neg_assign(self);
  }

  #[inline(always)]
  pub fn div(&self, rhs: &Scientific, precision: Precision) -> Result<Scientific, Error> {
    export_div(self, rhs, precision)
  }

  #[inline(always)]
  pub fn div_rem(&self, rhs: &Scientific) -> Result<(Scientific, Scientific), Error> {
    export_div_rem(self, rhs)
  }

  #[inline(always)]
  pub fn rem(&self, rhs: &Scientific) -> Result<Scientific, Error> {
    Ok(export_div_rem(self, rhs)?.1)
  }

  #[inline(always)]
  pub fn truncate_assign(&mut self, precision: Precision) {
    export_truncate_assign(self, precision)
  }

  #[inline(always)]
  pub fn truncate(&self, precision: Precision) -> Scientific {
    let mut r = self.clone();
    export_truncate_assign(&mut r, precision);
    r
  }

  /// round to nearest away from zero
  ///
  /// 0.4, -0.4 => 0.0
  ///
  /// 0.5, 0.6 => 1.0
  ///
  /// -0.5, -0.6 => -1.0
  #[inline(always)]
  pub fn round(&self, precision: Precision) -> Scientific {
    export_round(self, precision)
  }

  #[allow(clippy::len_without_is_empty)]
  #[inline(always)]
  pub fn len(&self) -> isize {
    self.len
  }

  #[inline(always)]
  pub fn decimals(&self) -> isize {
    -self.exponent
  }

  #[inline(always)]
  pub fn exponent0(&self) -> isize {
    self.exponent + self.len
  }

  #[inline(always)]
  pub fn exponent1(&self) -> isize {
    self.exponent + self.len - 1
  }

  // This function must not change before 0.5 since scientific-macro depends on it.
  #[inline(always)]
  pub fn exponent(&self) -> isize {
    self.exponent
  }

  #[inline(always)]
  pub fn powi(&self, exponent: usize) -> Scientific {
    export_powi(self, exponent)
  }

  // This function must not change before 0.5 since scientific-macro depends on it.
  #[inline(always)]
  pub fn is_zero(&self) -> bool {
    self.len == 0
  }

  /// Returns true if self has a positive sign, this excludes 0.
  #[inline(always)]
  pub fn is_sign_positive(&self) -> bool {
    self.len > 0 && self.sign == Sign::Positive
  }

  /// Returns true if self has a negative sign, this excludes 0.
  #[inline(always)]
  pub fn is_sign_negative(&self) -> bool {
    self.len > 0 && self.sign == Sign::Negative
  }
}
