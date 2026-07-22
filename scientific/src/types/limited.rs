//! Represents an isize type with limited rage.
//!
//! Exponent: Supported is `isize::MIN/4 .. isize::MAX/4+1`,
//! sounds weird but they are the same number in negative and positive.
//!
//! Length: Supported is `0 .. isize::MAX/4+1 + isize::MAX/8+1`.
//!
//! Which is plenty enough for both exponent and length.
//!
//! This is just to ensure that no bad over-/underflow can happen.   
//!
//! It is always safe to add/subtract up to two Length variables and a small number.
//!
//! It is always safe to add/subtract up to three of these variables, at most one Length, and a small number.
//!
//! This is guaranteed due to the range limitation, and makes calculation easier and
//! only the result must be checked, not each step on the way.

use crate::ConversionError;
use core::cmp::Ordering;
use core::ops::{Add, Deref, Sub, SubAssign};

#[derive(Copy, Clone)]
pub(crate) struct OutOfRangeError;

impl From<OutOfRangeError> for ConversionError {
  #[inline]
  fn from(_: OutOfRangeError) -> Self {
    ConversionError::OutOfRangeError
  }
}

//
// Exponent
//

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub(crate) struct Exponent(isize);

impl Exponent {
  pub(crate) const ZERO: Self = Self(0);
  pub(crate) const ONE: Self = Self(1);
  pub(crate) const NEG_ONE: Self = Self(-1);

  const MIN: isize = isize::MIN / 4; // 0xe0.. (everything else is 0, "simple" number)
  const MAX: isize = isize::MAX / 4 + 1; // 0x20.. (everything else is 0, "simple" number)

  #[inline]
  pub(crate) const fn try_new(value: isize) -> Result<Self, OutOfRangeError> {
    if value < Self::MIN || value > Self::MAX {
      Err(OutOfRangeError)
    } else {
      Ok(Self(value))
    }
  }

  #[inline]
  pub(crate) const fn new(value: isize) -> Self {
    match Self::try_new(value) {
      Err(OutOfRangeError) => Self::overflow(),
      Ok(value) => value,
    }
  }

  pub(crate) const fn overflow() -> ! {
    panic!("scientific: Exponent out of range")
  }
}

impl Deref for Exponent {
  type Target = isize;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Add for Exponent {
  type Output = isize;

  #[inline]
  fn add(self, rhs: Exponent) -> Self::Output {
    self.0 + rhs.0
  }
}
impl Add<isize> for Exponent {
  type Output = isize;

  #[inline]
  fn add(self, rhs: isize) -> Self::Output {
    self.0 + rhs
  }
}
impl Add<Exponent> for isize {
  type Output = isize;

  #[inline]
  fn add(self, rhs: Exponent) -> Self::Output {
    self + rhs.0
  }
}

impl Sub for Exponent {
  type Output = isize;

  #[inline]
  fn sub(self, rhs: Exponent) -> Self::Output {
    self.0 - rhs.0
  }
}
impl Sub<isize> for Exponent {
  type Output = isize;

  #[inline]
  fn sub(self, rhs: isize) -> Self::Output {
    self.0 - rhs
  }
}
impl Sub<Exponent> for isize {
  type Output = isize;

  #[inline]
  fn sub(self, rhs: Exponent) -> Self::Output {
    self - rhs.0
  }
}

impl PartialEq<isize> for Exponent {
  #[inline]
  fn eq(&self, other: &isize) -> bool {
    self.0.eq(other)
  }
}
impl PartialEq<Exponent> for isize {
  #[inline]
  fn eq(&self, other: &Exponent) -> bool {
    self.eq(&other.0)
  }
}
impl PartialOrd<isize> for Exponent {
  #[inline]
  fn partial_cmp(&self, other: &isize) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}
impl PartialOrd<Exponent> for isize {
  #[inline]
  fn partial_cmp(&self, other: &Exponent) -> Option<Ordering> {
    self.partial_cmp(&other.0)
  }
}

//
// Length
//

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub(crate) struct Length(isize);

impl Length {
  pub(crate) const ZERO: Self = Self(0);
  pub(crate) const ONE: Self = Self(1);

  const MAX: isize = isize::MAX / 4 + 1 + isize::MAX / 8 + 1; // 0x30_00_.. (everything else is 0, "simple" number)

  #[inline]
  pub(crate) const fn try_new(value: isize) -> Result<Self, OutOfRangeError> {
    if value < 0 || value > Self::MAX {
      Err(OutOfRangeError)
    } else {
      Ok(Self(value))
    }
  }

  #[inline]
  pub(crate) const fn new(value: isize) -> Self {
    match Self::try_new(value) {
      Err(OutOfRangeError) => Self::overflow(),
      Ok(value) => value,
    }
  }

  #[inline]
  pub(crate) const fn get(self) -> isize {
    self.0
  }

  #[inline]
  pub(crate) const fn try_from_usize(value: usize) -> Result<Self, OutOfRangeError> {
    if value > Self::MAX as usize {
      Err(OutOfRangeError)
    } else {
      Ok(Self(value as isize))
    }
  }

  #[inline]
  pub(crate) const fn from_usize(value: usize) -> Self {
    match Self::try_from_usize(value) {
      Err(OutOfRangeError) => Self::overflow(),
      Ok(value) => value,
    }
  }

  #[inline]
  pub(crate) const fn to_usize(self) -> usize {
    self.0 as usize
  }

  pub(crate) const fn overflow() -> ! {
    panic!("scientific: Length out of range")
  }
}

impl Deref for Length {
  type Target = isize;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Add for Length {
  type Output = isize;

  #[inline]
  fn add(self, rhs: Length) -> Self::Output {
    self.0 + rhs.0
  }
}
impl Add<isize> for Length {
  type Output = isize;

  #[inline]
  fn add(self, rhs: isize) -> Self::Output {
    self.0 + rhs
  }
}
impl Add<Length> for isize {
  type Output = isize;

  #[inline]
  fn add(self, rhs: Length) -> Self::Output {
    self + rhs.0
  }
}

impl Sub for Length {
  type Output = isize;

  #[inline]
  fn sub(self, rhs: Length) -> Self::Output {
    self.0 - rhs.0
  }
}
impl Sub<isize> for Length {
  type Output = isize;

  #[inline]
  fn sub(self, rhs: isize) -> Self::Output {
    self.0 - rhs
  }
}
impl Sub<Length> for isize {
  type Output = isize;

  #[inline]
  fn sub(self, rhs: Length) -> Self::Output {
    self - rhs.0
  }
}

impl PartialEq<isize> for Length {
  #[inline]
  fn eq(&self, other: &isize) -> bool {
    self.0.eq(other)
  }
}
impl PartialEq<Length> for isize {
  #[inline]
  fn eq(&self, other: &Length) -> bool {
    self.eq(&other.0)
  }
}
impl PartialOrd<isize> for Length {
  #[inline]
  fn partial_cmp(&self, other: &isize) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}
impl PartialOrd<Length> for isize {
  #[inline]
  fn partial_cmp(&self, other: &Length) -> Option<Ordering> {
    self.partial_cmp(&other.0)
  }
}

//
// Common
//

impl Add<Exponent> for Length {
  type Output = isize;

  #[inline]
  fn add(self, rhs: Exponent) -> Self::Output {
    self.0 + rhs.0
  }
}
impl Add<Length> for Exponent {
  type Output = isize;

  #[inline]
  fn add(self, rhs: Length) -> Self::Output {
    self.0 + rhs.0
  }
}
impl Sub<Exponent> for Length {
  type Output = isize;

  #[inline]
  fn sub(self, rhs: Exponent) -> Self::Output {
    self.0 - rhs.0
  }
}
impl Sub<Length> for Exponent {
  type Output = isize;

  #[inline]
  fn sub(self, rhs: Length) -> Self::Output {
    self.0 - rhs.0
  }
}

//
// Unchecked
//

pub(crate) struct Unchecked(pub(crate) isize);

impl SubAssign<Unchecked> for Length {
  #[inline]
  fn sub_assign(&mut self, rhs: Unchecked) {
    self.0 -= rhs.0;
  }
}
