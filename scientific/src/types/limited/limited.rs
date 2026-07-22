use crate::types::limited::exponent::Exponent;
use crate::types::limited::length::Length;
use crate::types::limited::range_to::UncheckedFromIsize;
use crate::types::limited::{ToIsize, Unchecked};
use core::cmp::Ordering;
use core::fmt::{Display, Formatter};
use core::ops::{Add, AddAssign, Div, Rem, Sub, SubAssign};

#[derive(Copy, Clone, Eq, Ord)]
#[repr(transparent)]
pub(crate) struct Limited<const N: usize>(isize);

// Any range

impl<const N: usize> Limited<N> {
  pub(crate) const ZERO: Self = Self(0);

  #[inline]
  pub(super) const fn from_isize_unchecked(value: isize) -> Self {
    Self(value)
  }

  /// Never chain more than one saturating!
  #[inline]
  pub(crate) fn saturating_add<T: ToIsize>(self, rhs: T) -> isize {
    self.0.saturating_add(rhs.to_isize())
  }

  /// Never chain more than one saturating!
  #[inline]
  pub(crate) fn saturating_sub<T: ToIsize>(self, rhs: T) -> isize {
    self.0.saturating_sub(rhs.to_isize())
  }
}

impl<const N: usize> UncheckedFromIsize for Limited<N> {
  #[inline]
  fn from_isize_unchecked(value: isize) -> Self {
    Self::from_isize_unchecked(value)
  }
}

// Range 1

impl Limited<1> {
  #[inline]
  fn new_i8(value: i8) -> Self {
    Self(value.into())
  }
}

impl From<bool> for Limited<1> {
  #[inline]
  fn from(value: bool) -> Self {
    Self(isize::from(value))
  }
}

// ToIsize

impl<const N: usize> ToIsize for Limited<N> {
  #[inline]
  fn to_isize(self) -> isize {
    self.0
  }
}

// Display

impl<const N: usize> Display for Limited<N> {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.0.fmt(f)
  }
}

// PartialEq, PartialOrd

impl<T: ToIsize, const N: usize> PartialEq<T> for Limited<N> {
  #[inline]
  fn eq(&self, other: &T) -> bool {
    self.0.eq(&other.to_isize())
  }
}
impl<T: ToIsize, const N: usize> PartialOrd<T> for Limited<N> {
  #[inline]
  fn partial_cmp(&self, other: &T) -> Option<Ordering> {
    self.0.partial_cmp(&other.to_isize())
  }

  #[inline]
  fn lt(&self, other: &T) -> bool {
    self.0.lt(&other.to_isize())
  }

  #[inline]
  fn le(&self, other: &T) -> bool {
    self.0.le(&other.to_isize())
  }

  #[inline]
  fn gt(&self, other: &T) -> bool {
    self.0.gt(&other.to_isize())
  }

  #[inline]
  fn ge(&self, other: &T) -> bool {
    self.0.ge(&other.to_isize())
  }
}

impl<const N: usize> PartialEq<Limited<N>> for isize {
  #[inline]
  fn eq(&self, other: &Limited<N>) -> bool {
    self.eq(&other.0)
  }
}
impl<const N: usize> PartialOrd<Limited<N>> for isize {
  #[inline]
  fn partial_cmp(&self, other: &Limited<N>) -> Option<Ordering> {
    self.partial_cmp(&other.0)
  }

  #[inline]
  fn lt(&self, other: &Limited<N>) -> bool {
    self.lt(&other.0)
  }

  #[inline]
  fn le(&self, other: &Limited<N>) -> bool {
    self.le(&other.0)
  }

  #[inline]
  fn gt(&self, other: &Limited<N>) -> bool {
    self.gt(&other.0)
  }

  #[inline]
  fn ge(&self, other: &Limited<N>) -> bool {
    self.ge(&other.0)
  }
}

// Add/Sub

macro_rules! limited_add {
  ($ln:literal, $rn:literal) => {
    impl Add<Limited<$rn>> for Limited<$ln> {
      type Output = Limited<{ $ln + $rn }>;

      #[inline]
      fn add(self, rhs: Limited<$rn>) -> Self::Output {
        const _: () = assert!($ln + $rn >= 0 && $ln + $rn < 6);
        Limited(self.0 + rhs.0)
      }
    }
  };
}

limited_add!(1, 1);
limited_add!(1, 2);
limited_add!(1, 3);
limited_add!(1, 4);
limited_add!(2, 1);
limited_add!(2, 2);
limited_add!(2, 3);
limited_add!(3, 1);
limited_add!(3, 2);
limited_add!(4, 1);

macro_rules! limited_sub {
  ($ln:literal, $rn:literal) => {
    impl Sub<Limited<$rn>> for Limited<$ln> {
      type Output = Limited<{ $ln + $rn }>;

      #[inline]
      fn sub(self, rhs: Limited<$rn>) -> Self::Output {
        const _: () = assert!($ln + $rn >= 0 && $ln + $rn < 6);
        Limited(self.0 - rhs.0)
      }
    }
  };
}

limited_sub!(1, 1);
limited_sub!(1, 2);
limited_sub!(1, 3);
limited_sub!(1, 4);
limited_sub!(2, 1);
limited_sub!(2, 2);
limited_sub!(2, 3);
limited_sub!(3, 1);
limited_sub!(3, 2);
limited_sub!(4, 1);

// Add/Sub i8

macro_rules! limited_add_i8 {
  ($n:literal) => {
    impl Add<i8> for Limited<$n> {
      type Output = Limited<{ $n + 1 }>;

      #[inline]
      fn add(self, rhs: i8) -> Self::Output {
        self + Limited::new_i8(rhs)
      }
    }

    impl Add<Limited<$n>> for i8 {
      type Output = Limited<{ $n + 1 }>;

      #[inline]
      fn add(self, rhs: Limited<$n>) -> Self::Output {
        Limited::new_i8(self) + rhs
      }
    }
  };
}
limited_add_i8!(1);
limited_add_i8!(2);
limited_add_i8!(3);
limited_add_i8!(4);

macro_rules! limited_sub_i8 {
  ($n:literal) => {
    impl Sub<i8> for Limited<$n> {
      type Output = Limited<{ $n + 1 }>;

      #[inline]
      fn sub(self, rhs: i8) -> Self::Output {
        self - Limited::new_i8(rhs)
      }
    }

    impl Sub<Limited<$n>> for i8 {
      type Output = Limited<{ $n + 1 }>;

      #[inline]
      fn sub(self, rhs: Limited<$n>) -> Self::Output {
        Limited::new_i8(self) - rhs
      }
    }
  };
}
limited_sub_i8!(1);
limited_sub_i8!(2);
limited_sub_i8!(3);
limited_sub_i8!(4);

// Div/Mod

impl<T: ToIsize, const N: usize> Div<T> for Limited<N> {
  type Output = isize;

  #[inline]
  fn div(self, rhs: T) -> Self::Output {
    self.0 / rhs.to_isize()
  }
}

impl<T: ToIsize, const N: usize> Rem<T> for Limited<N> {
  type Output = isize;

  #[inline]
  fn rem(self, rhs: T) -> Self::Output {
    self.0 % rhs.to_isize()
  }
}

//
// Unchecked
//

impl<const N: usize> Sub<Unchecked> for Limited<N> {
  type Output = Limited<N>;

  #[inline]
  fn sub(self, rhs: Unchecked) -> Self::Output {
    Self(self.0 - rhs.0)
  }
}

impl<const N: usize> AddAssign<Unchecked> for Limited<N> {
  #[inline]
  fn add_assign(&mut self, rhs: Unchecked) {
    self.0 += rhs.0;
  }
}
impl<const N: usize> SubAssign<Unchecked> for Limited<N> {
  #[inline]
  fn sub_assign(&mut self, rhs: Unchecked) {
    self.0 -= rhs.0;
  }
}

impl AddAssign<Unchecked> for Length {
  #[inline]
  fn add_assign(&mut self, rhs: Unchecked) {
    self.0 .0 += rhs.0;
  }
}
impl SubAssign<Unchecked> for Length {
  #[inline]
  fn sub_assign(&mut self, rhs: Unchecked) {
    self.0 .0 -= rhs.0;
  }
}

impl AddAssign<Unchecked> for Exponent {
  #[inline]
  fn add_assign(&mut self, rhs: Unchecked) {
    self.0 .0 += rhs.0;
  }
}
impl SubAssign<Unchecked> for Exponent {
  #[inline]
  fn sub_assign(&mut self, rhs: Unchecked) {
    self.0 .0 -= rhs.0;
  }
}
