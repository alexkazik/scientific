use crate::types::conversion_error::ConversionError;
use crate::types::limited::length::Length;
use crate::types::limited::limited::Limited;
use crate::types::limited::{ToIsize, UncheckedFromIsize, UnwrapOutOfRange};
use core::cmp::Ordering;
use core::fmt::{Display, Formatter};
use core::ops::{Add, Deref, Div, Rem, Sub};

//
// Error
//

#[derive(Copy, Clone)]
pub(crate) struct ExponentOutOfRangeError;

impl ExponentOutOfRangeError {
  const fn panic() -> ! {
    panic!("scientific: Exponent out of range")
  }
}

impl From<ExponentOutOfRangeError> for ConversionError {
  #[inline]
  fn from(ExponentOutOfRangeError {}: ExponentOutOfRangeError) -> Self {
    ConversionError::OutOfRangeError
  }
}

impl<T> UnwrapOutOfRange for Result<T, ExponentOutOfRangeError> {
  type Output = T;

  fn unwrap_out_of_range(self) -> Self::Output {
    match self {
      Err(ExponentOutOfRangeError) => ExponentOutOfRangeError::panic(),
      Ok(value) => value,
    }
  }
}

//
// Exponent
//

#[derive(Copy, Clone, Eq, Ord)]
#[repr(transparent)]
pub(crate) struct Exponent(pub(super) Limited<1>);

impl Exponent {
  pub(crate) const ZERO: Self = Self(Limited::from_isize_unchecked(0));
  pub(crate) const ONE: Self = Self(Limited::from_isize_unchecked(1));
  pub(crate) const NEG_ONE: Self = Self(Limited::from_isize_unchecked(-1));

  const MIN: isize = isize::MIN / 4; // 0xe0_00_.. (everything else is 0, a "simple" number)
  const MAX: isize = isize::MAX / 4 + 1; // 0x20_00_.. (everything else is 0, a "simple" number)

  #[inline]
  const fn try_from_isize(value: isize) -> Result<Self, ExponentOutOfRangeError> {
    if value < Self::MIN || value > Self::MAX {
      Err(ExponentOutOfRangeError)
    } else {
      Ok(Self(Limited::from_isize_unchecked(value)))
    }
  }

  #[inline]
  pub(crate) const fn from_isize(value: isize) -> Self {
    match Self::try_from_isize(value) {
      Err(ExponentOutOfRangeError) => ExponentOutOfRangeError::panic(),
      Ok(value) => value,
    }
  }

  #[inline]
  pub(crate) fn try_new<T: ToIsize>(value: T) -> Result<Self, ExponentOutOfRangeError> {
    Self::try_from_isize(value.to_isize())
  }

  #[inline]
  pub(crate) fn new<T: ToIsize>(value: T) -> Self {
    Self::try_from_isize(value.to_isize()).unwrap_out_of_range()
  }
}

impl ToIsize for Exponent {
  #[inline]
  fn to_isize(self) -> isize {
    self.0.to_isize()
  }
}

impl Deref for Exponent {
  type Target = Limited<1>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Display for Exponent {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.0.fmt(f)
  }
}

impl UncheckedFromIsize for Exponent {
  #[inline]
  fn from_isize_unchecked(value: isize) -> Self {
    Self(Limited::from_isize_unchecked(value))
  }
}

// PartialEq/PartialOrd

impl<T: ToIsize> PartialEq<T> for Exponent {
  #[inline]
  fn eq(&self, other: &T) -> bool {
    self.0.eq(other)
  }
}
impl<T: ToIsize> PartialOrd<T> for Exponent {
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

impl PartialEq<Exponent> for isize {
  #[inline]
  fn eq(&self, other: &Exponent) -> bool {
    self.eq(&other.0)
  }
}
impl PartialOrd<Exponent> for isize {
  #[inline]
  fn partial_cmp(&self, other: &Exponent) -> Option<Ordering> {
    self.partial_cmp(&other.0)
  }

  #[inline]
  fn lt(&self, other: &Exponent) -> bool {
    self.lt(&other.0)
  }

  #[inline]
  fn le(&self, other: &Exponent) -> bool {
    self.le(&other.0)
  }

  #[inline]
  fn gt(&self, other: &Exponent) -> bool {
    self.gt(&other.0)
  }

  #[inline]
  fn ge(&self, other: &Exponent) -> bool {
    self.ge(&other.0)
  }
}

// Add/Sub

impl Add for Exponent {
  type Output = Limited<2>;

  #[inline]
  fn add(self, rhs: Exponent) -> Self::Output {
    self.0 + rhs.0
  }
}

impl Sub for Exponent {
  type Output = Limited<2>;

  #[inline]
  fn sub(self, rhs: Exponent) -> Self::Output {
    self.0 - rhs.0
  }
}

macro_rules! exponent_add {
  ($n:literal) => {
    impl Add<Limited<$n>> for Exponent {
      type Output = Limited<{ $n + 1 }>;

      #[inline]
      fn add(self, rhs: Limited<$n>) -> Self::Output {
        self.0 + rhs
      }
    }

    impl Add<Exponent> for Limited<$n> {
      type Output = Limited<{ $n + 1 }>;

      #[inline]
      fn add(self, rhs: Exponent) -> Self::Output {
        self + rhs.0
      }
    }
  };
}
exponent_add!(1);
exponent_add!(2);
exponent_add!(3);
exponent_add!(4);

macro_rules! exponent_sub {
  ($n:literal) => {
    impl Sub<Limited<$n>> for Exponent {
      type Output = Limited<{ $n + 1 }>;

      #[inline]
      fn sub(self, rhs: Limited<$n>) -> Self::Output {
        self.0 - rhs
      }
    }

    impl Sub<Exponent> for Limited<$n> {
      type Output = Limited<{ $n + 1 }>;

      #[inline]
      fn sub(self, rhs: Exponent) -> Self::Output {
        self - rhs.0
      }
    }
  };
}
exponent_sub!(1);
exponent_sub!(2);
exponent_sub!(3);
exponent_sub!(4);

// Add/Sub i8

impl Add<i8> for Exponent {
  type Output = Limited<2>;

  #[inline]
  fn add(self, rhs: i8) -> Self::Output {
    self.0 + rhs
  }
}
impl Add<Exponent> for i8 {
  type Output = Limited<2>;

  #[inline]
  fn add(self, rhs: Exponent) -> Self::Output {
    self + rhs.0
  }
}

impl Sub<i8> for Exponent {
  type Output = Limited<2>;

  #[inline]
  fn sub(self, rhs: i8) -> Self::Output {
    self.0 - rhs
  }
}
impl Sub<Exponent> for i8 {
  type Output = Limited<2>;

  #[inline]
  fn sub(self, rhs: Exponent) -> Self::Output {
    self - rhs.0
  }
}

// Div/Mod

impl<T: ToIsize> Div<T> for Exponent {
  type Output = isize;

  #[inline]
  fn div(self, rhs: T) -> Self::Output {
    self.0 / rhs
  }
}

impl<T: ToIsize> Rem<T> for Exponent {
  type Output = isize;

  #[inline]
  fn rem(self, rhs: T) -> Self::Output {
    self.0 % rhs
  }
}

// Add exponent and length

impl Add<Length> for Exponent {
  type Output = Limited<2>;

  #[inline]
  fn add(self, rhs: Length) -> Self::Output {
    self.0 + rhs.0
  }
}
impl Add<Exponent> for Length {
  type Output = Limited<2>;

  #[inline]
  fn add(self, rhs: Exponent) -> Self::Output {
    self.0 + rhs.0
  }
}
