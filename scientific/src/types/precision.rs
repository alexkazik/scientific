use core::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Precision {
  /// The number of digits to round/truncate to. If the number is less or equal to zero then the result is always zero!
  Digits(isize),
  Decimals(isize),
}

impl Precision {
  pub const INTEGER: Precision = Precision::Decimals(0);
  pub const F64: Precision = Precision::Digits(16);
}

impl Default for Precision {
  fn default() -> Self {
    Precision::INTEGER
  }
}

impl Add<isize> for Precision {
  type Output = Precision;

  fn add(self, rhs: isize) -> Self::Output {
    match self {
      Precision::Digits(d) => Precision::Digits(d + rhs),
      Precision::Decimals(d) => Precision::Decimals(d + rhs),
    }
  }
}

impl AddAssign<isize> for Precision {
  fn add_assign(&mut self, rhs: isize) {
    match self {
      Precision::Digits(d) => *d += rhs,
      Precision::Decimals(d) => *d += rhs,
    }
  }
}

impl Sub<isize> for Precision {
  type Output = Precision;

  fn sub(self, rhs: isize) -> Self::Output {
    match self {
      Precision::Digits(d) => Precision::Digits(d - rhs),
      Precision::Decimals(d) => Precision::Decimals(d - rhs),
    }
  }
}

impl SubAssign<isize> for Precision {
  fn sub_assign(&mut self, rhs: isize) {
    match self {
      Precision::Digits(d) => *d -= rhs,
      Precision::Decimals(d) => *d -= rhs,
    }
  }
}
