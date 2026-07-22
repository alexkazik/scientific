use crate::types::conversion_error::ConversionError;
use crate::types::limited::limited::Limited;
use crate::types::limited::{ToIsize, Unchecked, UncheckedFromIsize, UnwrapOutOfRange};
use core::cmp::Ordering;
use core::fmt::{Display, Formatter};
use core::ops::{Add, Deref, Div, Rem, Sub};

//
// Error
//

#[derive(Copy, Clone)]
pub(crate) struct LengthOutOfRangeError;

impl LengthOutOfRangeError {
  const fn panic() -> ! {
    panic!("scientific: Length out of range")
  }
}

impl From<LengthOutOfRangeError> for ConversionError {
  #[inline]
  fn from(LengthOutOfRangeError {}: LengthOutOfRangeError) -> Self {
    ConversionError::OutOfRangeError
  }
}

impl<T> UnwrapOutOfRange for Result<T, LengthOutOfRangeError> {
  type Output = T;

  fn unwrap_out_of_range(self) -> Self::Output {
    match self {
      Err(LengthOutOfRangeError) => LengthOutOfRangeError::panic(),
      Ok(value) => value,
    }
  }
}

//
// Length
//

#[derive(Copy, Clone, Eq, Ord)]
#[repr(transparent)]
pub(crate) struct Length(pub(super) Limited<1>);

impl Length {
  pub(crate) const ZERO: Self = Self(Limited::from_isize_unchecked(0));
  pub(crate) const ONE: Self = Self(Limited::from_isize_unchecked(1));

  const MAX: isize = isize::MAX / 4 + 1; // 0x20_00_.. (everything else is 0, a "simple" number)

  #[inline]
  pub(crate) fn try_new<T: ToIsize>(value: T) -> Result<Self, LengthOutOfRangeError> {
    let value = value.to_isize();
    #[allow(clippy::manual_range_contains, clippy::redundant_else)]
    if value < 0 || value > Self::MAX {
      Err(LengthOutOfRangeError)
    } else {
      Ok(Self(Limited::from_isize_unchecked(value)))
    }
  }

  #[inline]
  pub(crate) fn new<T: ToIsize>(value: T) -> Self {
    Self::try_new(value).unwrap_out_of_range()
  }

  #[inline]
  pub(crate) const fn new_i8(value: i8) -> Self {
    Self::from_isize_unchecked(value as isize)
  }

  #[inline]
  pub(crate) const fn from_isize_unchecked(value: isize) -> Self {
    Self(Limited::from_isize_unchecked(value))
  }

  #[inline]
  pub(crate) const fn try_from_usize(value: usize) -> Result<Self, LengthOutOfRangeError> {
    if value > Self::MAX as usize {
      Err(LengthOutOfRangeError)
    } else {
      Ok(Self::from_isize_unchecked(value as isize))
    }
  }

  #[inline]
  pub(crate) const fn from_usize(value: usize) -> Self {
    match Self::try_from_usize(value) {
      Err(LengthOutOfRangeError) => LengthOutOfRangeError::panic(),
      Ok(value) => value,
    }
  }

  #[inline]
  pub(crate) fn to_usize(self) -> usize {
    self.to_isize() as usize
  }
}

impl ToIsize for Length {
  #[inline]
  fn to_isize(self) -> isize {
    self.0.to_isize()
  }
}

impl Deref for Length {
  type Target = Limited<1>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Display for Length {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.0.fmt(f)
  }
}

impl UncheckedFromIsize for Length {
  #[inline]
  fn from_isize_unchecked(value: isize) -> Self {
    Self::from_isize_unchecked(value)
  }
}

// PartialEq/PartialOrd

impl<T: ToIsize> PartialEq<T> for Length {
  #[inline]
  fn eq(&self, other: &T) -> bool {
    self.0.eq(other)
  }
}
impl<T: ToIsize> PartialOrd<T> for Length {
  #[inline]
  fn partial_cmp(&self, other: &T) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }

  #[inline]
  fn lt(&self, other: &T) -> bool {
    self.0.lt(other)
  }

  #[inline]
  fn le(&self, other: &T) -> bool {
    self.0.le(other)
  }

  #[inline]
  fn gt(&self, other: &T) -> bool {
    self.0.gt(other)
  }

  #[inline]
  fn ge(&self, other: &T) -> bool {
    self.0.ge(other)
  }
}

impl PartialEq<Length> for isize {
  #[inline]
  fn eq(&self, other: &Length) -> bool {
    self.eq(&other.0)
  }
}
impl PartialOrd<Length> for isize {
  #[inline]
  fn partial_cmp(&self, other: &Length) -> Option<Ordering> {
    self.partial_cmp(&other.0)
  }

  #[inline]
  fn lt(&self, other: &Length) -> bool {
    self.lt(&other.0)
  }

  #[inline]
  fn le(&self, other: &Length) -> bool {
    self.le(&other.0)
  }

  #[inline]
  fn gt(&self, other: &Length) -> bool {
    self.gt(&other.0)
  }

  #[inline]
  fn ge(&self, other: &Length) -> bool {
    self.ge(&other.0)
  }
}

// Add/Sub

impl Add for Length {
  type Output = Limited<2>;

  #[inline]
  fn add(self, rhs: Length) -> Self::Output {
    self.0 + rhs.0
  }
}

impl Sub for Length {
  type Output = Limited<2>;

  #[inline]
  fn sub(self, rhs: Length) -> Self::Output {
    self.0 - rhs.0
  }
}

macro_rules! length_add {
  ($n:literal) => {
    impl Add<Limited<$n>> for Length {
      type Output = Limited<{ $n + 1 }>;

      #[inline]
      fn add(self, rhs: Limited<$n>) -> Self::Output {
        self.0 + rhs
      }
    }

    impl Add<Length> for Limited<$n> {
      type Output = Limited<{ $n + 1 }>;

      #[inline]
      fn add(self, rhs: Length) -> Self::Output {
        self + rhs.0
      }
    }
  };
}
length_add!(1);
length_add!(2);
length_add!(3);
length_add!(4);

macro_rules! length_sub {
  ($n:literal) => {
    impl Sub<Limited<$n>> for Length {
      type Output = Limited<{ $n + 1 }>;

      #[inline]
      fn sub(self, rhs: Limited<$n>) -> Self::Output {
        self.0 - rhs
      }
    }

    impl Sub<Length> for Limited<$n> {
      type Output = Limited<{ $n + 1 }>;

      #[inline]
      fn sub(self, rhs: Length) -> Self::Output {
        self - rhs.0
      }
    }
  };
}
length_sub!(1);
length_sub!(2);
length_sub!(3);
length_sub!(4);

// Add/Sub i8

impl Add<i8> for Length {
  type Output = Limited<2>;

  #[inline]
  fn add(self, rhs: i8) -> Self::Output {
    self.0 + rhs
  }
}
impl Add<Length> for i8 {
  type Output = Limited<2>;

  #[inline]
  fn add(self, rhs: Length) -> Self::Output {
    self + rhs.0
  }
}

impl Sub<i8> for Length {
  type Output = Limited<2>;

  #[inline]
  fn sub(self, rhs: i8) -> Self::Output {
    self.0 - rhs
  }
}
impl Sub<Length> for i8 {
  type Output = Limited<2>;

  #[inline]
  fn sub(self, rhs: Length) -> Self::Output {
    self - rhs.0
  }
}

// Div/Mod

impl<T: ToIsize> Div<T> for Length {
  type Output = isize;

  #[inline]
  fn div(self, rhs: T) -> Self::Output {
    self.0 / rhs
  }
}

impl<T: ToIsize> Rem<T> for Length {
  type Output = isize;

  #[inline]
  fn rem(self, rhs: T) -> Self::Output {
    self.0 % rhs
  }
}

//
// Unchecked
//

impl Sub<Unchecked> for Length {
  type Output = Length;

  #[inline]
  fn sub(self, rhs: Unchecked) -> Self::Output {
    Self(Limited::from_isize_unchecked(self.to_isize() - rhs.0))
  }
}
