#[cfg(doc)]
use crate::types::rounding::Round::{
  RoundAwayFromZero, RoundDown, RoundHalfAwayFromZero, RoundHalfDown, RoundHalfToEven,
  RoundHalfToOdd, RoundHalfTowardsZero, RoundHalfUp, RoundUp,
};
use core::cmp::Ordering;

/// Trait to implement different rounding methods.
///
/// Method                    | Result is -2  | Result is -1  | Result is 0  | Result is 1 | Result is 2 | Implementor
/// ------------------------- | :-----------: | :-----------: | :----------: | :---------: | :---------: | ------------------
/// Directed, Down            | \[-2.0, -1.0) | \[-1.0,  0.0) |  \[0.0, 1.0) | \[1.0, 2.0) | \[2.0, 3.0) | [`RoundDown`]
/// Directed, Up              |  (-3.0, -2.0] |  (-2.0, -1.0] |  (-1.0, 0.0] |  (0.0, 1.0] |  (1.0, 2.0] | [`RoundUp`]
/// Directed, Towards 0       |  (-3.0, -2.0] |  (-2.0, -1.0] |  (-1.0, 1.0) | \[1.0, 2.0) | \[2.0, 3.0) | [`Truncate`]
/// Directed, Away From 0     | \[-2.0, -1.0) | \[-1.0,  0.0) |      0.0     |  (0.0, 1.0] |  (1.0, 2.0] | [`RoundAwayFromZero`]
/// Nearest, Half Down        |  (-2.5, -1.5] |  (-1.5, -0.5] |  (-0.5, 0.5] |  (0.5, 1.5] |  (1.5, 2.5] | [`RoundHalfDown`]
/// Nearest, Half Up          | \[-2.5, -1.5) | \[-1.5, -0.5) | \[-0.5, 0.5) | \[0.5, 1.5) | \[1.5, 2.5) | [`RoundHalfUp`]
/// Nearest, Half Towards 0   | \[-2.5, -1.5) | \[-1.5, -0.5) | \[-0.5, 0.5] |  (0.5, 1.5] |  (1.5, 2.5] | [`RoundHalfTowardsZero`]
/// Nearest, Half Away From 0 |  (-2.5, -1.5] |  (-1.5, -0.5] |  (-0.5, 0.5) | \[0.5, 1.5) | \[1.5, 2.5) | [`RoundHalfAwayFromZero`]
/// Nearest, Half To Even     | \[-2.5, -1.5] |  (-1.5, -0.5) | \[-0.5, 0.5] |  (0.5, 1.5) | \[1.5, 2.5] | [`RoundHalfToEven`]
/// Nearest, half To Odd      |  (-2.5, -1.5) | \[-1.5, -0.5] |  (-0.5, 0.5) | \[0.5, 1.5] |  (1.5, 2.5) | [`RoundHalfToOdd`]

pub trait Rounding: Copy {
  /// Defaults to `false`.
  #[inline(always)]
  #[must_use]
  fn is_truncate() -> bool {
    false
  }
  /// Return whether the rounding results in the number (ignoring the sign) should be increased.
  ///
  /// `before` and `after` are the digits before and after the dot (or any other point to perform
  /// the rounding).
  ///
  /// Valid numbers for `before` are 0-9, and 1-9 for `after`.
  ///
  /// If `is_truncate` returns `true` then this function must always return `false`.
  ///
  /// Please see the source for examples.
  fn round_away_from_zero(self, is_negative: bool, before: i8, after: i8) -> bool;
}

/// Also known as round towards zero, please see [Rounding] for an overview.
#[derive(Copy, Clone)]
pub struct Truncate;

impl Rounding for Truncate {
  #[inline(always)]
  fn is_truncate() -> bool {
    true
  }

  #[inline(always)]
  fn round_away_from_zero(self, _is_negative: bool, _before: i8, _after: i8) -> bool {
    false
  }
}

/// Please see [Rounding] for an overview.
#[derive(Copy, Clone)]
pub enum Round {
  RoundAwayFromZero,
  /// Also known as floor.
  RoundDown,
  /// Also known as ceiling.
  RoundUp,
  RoundHalfDown,
  RoundHalfUp,
  RoundHalfTowardsZero,
  RoundHalfAwayFromZero,
  RoundHalfToEven,
  RoundHalfToOdd,
}

impl Rounding for Round {
  #[inline(always)]
  fn round_away_from_zero(self, is_negative: bool, before: i8, after: i8) -> bool {
    match self {
      Round::RoundAwayFromZero => true,
      Round::RoundDown => is_negative,
      Round::RoundUp => !is_negative,
      Round::RoundHalfDown => match after.cmp(&5) {
        Ordering::Less => false,
        Ordering::Equal => is_negative,
        Ordering::Greater => true,
      },
      Round::RoundHalfUp => match after.cmp(&5) {
        Ordering::Less => false,
        Ordering::Equal => !is_negative,
        Ordering::Greater => true,
      },
      Round::RoundHalfTowardsZero => after > 5,
      Round::RoundHalfAwayFromZero => after >= 5,
      Round::RoundHalfToEven => match after.cmp(&5) {
        Ordering::Less => false,
        Ordering::Equal => before & 1 != 0,
        Ordering::Greater => true,
      },
      Round::RoundHalfToOdd => match after.cmp(&5) {
        Ordering::Less => false,
        Ordering::Equal => before & 1 == 0,
        Ordering::Greater => true,
      },
    }
  }
}
