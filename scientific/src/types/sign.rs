use core::ops::{BitXor, Not};

#[derive(Eq, PartialEq, Copy, Clone)]
pub(crate) enum Sign {
  Positive,
  Negative,
}

impl Not for Sign {
  type Output = Self;

  #[inline(always)]
  fn not(self) -> Self::Output {
    match self {
      Sign::Positive => Sign::Negative,
      Sign::Negative => Sign::Positive,
    }
  }
}

impl BitXor for Sign {
  type Output = Sign;

  #[inline(always)]
  fn bitxor(self, rhs: Self) -> Self::Output {
    if self == rhs {
      Sign::Positive
    } else {
      Sign::Negative
    }
  }
}
