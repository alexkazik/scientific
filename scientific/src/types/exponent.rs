use core::cmp::Ordering;
use core::fmt::{Display, Formatter};
use core::ops::{Add, AddAssign, Deref, Sub, SubAssign};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Exponent(isize);

impl Exponent {
  pub(crate) const ZERO: Exponent = Exponent(0);
  pub(crate) const ONE: Exponent = Exponent(1);
  pub(crate) const NEG_ONE: Exponent = Exponent(-1);

  pub(crate) const fn new(value: isize) -> Exponent {
    Exponent(value)
  }
}

impl Display for Exponent {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.0.fmt(f)
  }
}

impl Deref for Exponent {
  type Target = isize;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Add for Exponent {
  type Output = Exponent;

  fn add(self, rhs: Self) -> Self::Output {
    Exponent(self.0.checked_add(rhs.0).expect("Exponent overflow"))
  }
}
impl Add<isize> for Exponent {
  type Output = Exponent;

  fn add(self, rhs: isize) -> Self::Output {
    Exponent(self.0.checked_add(rhs).expect("Exponent overflow"))
  }
}
impl Add<Exponent> for isize {
  type Output = Exponent;

  fn add(self, rhs: Exponent) -> Self::Output {
    Exponent(self.checked_add(rhs.0).expect("Exponent overflow"))
  }
}

impl AddAssign<isize> for Exponent {
  fn add_assign(&mut self, rhs: isize) {
    self.0 = self.0.checked_add(rhs).expect("Exponent overflow");
  }
}

impl Sub for Exponent {
  type Output = Exponent;

  fn sub(self, rhs: Self) -> Self::Output {
    Exponent(self.0.checked_sub(rhs.0).expect("Exponent overflow"))
  }
}
impl Sub<isize> for Exponent {
  type Output = Exponent;

  fn sub(self, rhs: isize) -> Self::Output {
    Exponent(self.0.checked_sub(rhs).expect("Exponent overflow"))
  }
}
impl Sub<Exponent> for isize {
  type Output = Exponent;

  fn sub(self, rhs: Exponent) -> Self::Output {
    Exponent(self.checked_sub(rhs.0).expect("Exponent overflow"))
  }
}

impl SubAssign<isize> for Exponent {
    fn sub_assign(&mut self, rhs: isize) {
        self.0 = self.0.checked_sub(rhs).expect("Exponent overflow");
    }
}

impl PartialEq<isize> for Exponent {
  fn eq(&self, other: &isize) -> bool {
    self.0.eq(other)
  }
}
impl PartialEq<Exponent> for isize {
  fn eq(&self, other: &Exponent) -> bool {
    self.eq(&other.0)
  }
}
impl PartialOrd<isize> for Exponent {
  fn partial_cmp(&self, other: &isize) -> Option<Ordering> {
    self.0.partial_cmp(other)
  }
}
impl PartialOrd<Exponent> for isize {
  fn partial_cmp(&self, other: &Exponent) -> Option<Ordering> {
    self.partial_cmp(&other.0)
  }
}
