//! Represents an isize type with limited rage.
//!
//! Each step is `isize::MAX/8+1`.
//! `Limited<1>` represents `-isize::MAX/8+1 .. isize::MAX/8+1`.
//! `Limited<2>` represents `-isize::MAX/4+2 .. isize::MAX/4+2`.
//!
//! The theoretical maximum is `<7>` since 8 would already overflow.
//!
//! Currently only `<5>` is supported, as nothing more is required.
//!
//! This allows for additions/subtractions which are guaranteed to not
//! overflow. No overflowing operation exists.

pub(crate) use crate::types::limited::exponent::{Exponent, ExponentOutOfRangeError};
pub(crate) use crate::types::limited::length::{Length, LengthOutOfRangeError};
pub(crate) use crate::types::limited::limited::Limited;
pub(crate) use crate::types::limited::range_to::{RangeTo, RangeToIter, UncheckedFromIsize};

mod exponent;
mod length;
#[allow(clippy::module_inception)]
mod limited;
mod range_to;

// ToIsize

pub(crate) trait ToIsize: Copy {
  fn to_isize(self) -> isize;
}

impl ToIsize for isize {
  #[inline]
  fn to_isize(self) -> isize {
    self
  }
}

// Uncheched

pub(crate) struct Unchecked(pub(crate) isize);

// UnwrapOutOfRange

pub(crate) trait UnwrapOutOfRange {
  type Output;
  fn unwrap_out_of_range(self) -> Self::Output;
}
